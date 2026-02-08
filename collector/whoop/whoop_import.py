#!/usr/bin/env python3
"""
Whoop Data Import Script for FxLifeSheet (v2 API)

This script fetches recovery, sleep, and workout data from Whoop v2 API
using OAuth 2.0 authentication and inserts it into the FxLifeSheet database.

Usage:
    python whoop_import.py [--date YYYY-MM-DD] [--days N] [--from-last]

Options:
    --date YYYY-MM-DD    Import data for specific date
    --days N             Number of days to import (default: 1)
    --from-last          Import from last data point in database to today

Environment Variables:
    WHOOP_CLIENT_ID - Your Whoop app client ID
    WHOOP_CLIENT_SECRET - Your Whoop app client secret
    WHOOP_ACCESS_TOKEN - Your Whoop access token (from OAuth flow)
    DATABASE_URL - PostgreSQL connection string
"""

import os
import sys
import argparse
import logging
from datetime import datetime, timedelta, timezone
from typing import Optional, Dict, Any, List
import uuid
import json
import time
import smtplib
from email.mime.text import MIMEText
from email.mime.multipart import MIMEMultipart

import requests
import psycopg2
import psycopg2.extras
from dotenv import load_dotenv
from oauth_helper import WhoopOAuthHelper

# Load environment variables
load_dotenv()

# Configure logging
logging.basicConfig(
    level=logging.INFO,
    format='%(asctime)s - %(levelname)s - %(message)s'
)
logger = logging.getLogger(__name__)

class WhoopImporter:
    def __init__(self):
        self.client_id = os.getenv("WHOOP_CLIENT_ID")
        self.client_secret = os.getenv("WHOOP_CLIENT_SECRET")
        self.access_token = os.getenv("WHOOP_ACCESS_TOKEN")
        self.refresh_token = os.getenv("WHOOP_REFRESH_TOKEN")
        self.database_url = os.getenv("DATABASE_URL")
        
        if not self.database_url:
            raise ValueError("Missing required environment variable: DATABASE_URL")
        
        if not self.access_token and not self.refresh_token:
            raise ValueError("Either WHOOP_ACCESS_TOKEN or WHOOP_REFRESH_TOKEN must be provided")
        
        self.base_url = "https://api.prod.whoop.com"
        self.headers = {"Content-Type": "application/json"}
        self.db_conn = None
        self.import_id = str(uuid.uuid4())
        self.import_timestamp = datetime.now()
        
    def ensure_valid_access_token(self) -> None:
        """Ensure we have a valid access token, refreshing if necessary"""
        if not self.access_token and self.refresh_token:
            logger.info("No access token found, refreshing using refresh token...")
            self.refresh_access_token()
        elif self.access_token:
            logger.info("Access token found, testing connection...")
        
        # Update headers with current access token
        self.headers["Authorization"] = f"Bearer {self.access_token}"
        
    def refresh_access_token(self) -> bool:
        """Refresh access token using refresh token with retry logic
        
        Returns:
            bool: True if refresh successful, False if refresh token is expired
        """
        max_retries = 3
        base_delay = 1  # seconds
        
        for attempt in range(max_retries):
            try:
                oauth_helper = WhoopOAuthHelper()
                token_response = oauth_helper.refresh_access_token(self.refresh_token)
                
                self.access_token = token_response.get('access_token')
                if not self.access_token:
                    raise ValueError("No access token received from refresh")
                    
                # Update refresh token if a new one was provided
                if 'refresh_token' in token_response:
                    self.refresh_token = token_response['refresh_token']
                
                # Persist tokens to .env file for future runs
                self._update_env_file(self.access_token, self.refresh_token)
                    
                logger.info("Successfully refreshed access token")
                return True
                
            except requests.exceptions.HTTPError as e:
                if e.response.status_code == 400:
                    # Refresh token expired - cannot recover automatically
                    logger.error("Refresh token expired. Manual re-authentication required.")
                    self._send_token_expiry_alert()
                    return False
                elif e.response.status_code in [429, 503, 504]:  # Rate limit or server errors
                    if attempt < max_retries - 1:
                        delay = base_delay * (2 ** attempt)  # Exponential backoff
                        logger.warning(f"Token refresh failed (attempt {attempt + 1}/{max_retries}), retrying in {delay}s: {e}")
                        time.sleep(delay)
                        continue
                logger.error(f"Failed to refresh access token after {max_retries} attempts: {e}")
                return False
            except Exception as e:
                if attempt < max_retries - 1:
                    delay = base_delay * (2 ** attempt)
                    logger.warning(f"Token refresh failed (attempt {attempt + 1}/{max_retries}), retrying in {delay}s: {e}")
                    time.sleep(delay)
                    continue
                logger.error(f"Failed to refresh access token after {max_retries} attempts: {e}")
                return False
        
        return False
    
    def _update_env_file(self, access_token: str, refresh_token: str) -> None:
        """Update .env file with new tokens"""
        try:
            env_file = '.env'
            if not os.path.exists(env_file):
                logger.warning(f"{env_file} not found, skipping token persistence")
                return
                
            with open(env_file, 'r') as f:
                content = f.read()
            
            # Update or add access token
            if 'WHOOP_ACCESS_TOKEN=' in content:
                # Replace existing token
                import re
                content = re.sub(
                    r'WHOOP_ACCESS_TOKEN=.*',
                    f'WHOOP_ACCESS_TOKEN={access_token}',
                    content
                )
            else:
                # Add new token
                content += f'\nWHOOP_ACCESS_TOKEN={access_token}'
            
            # Update or add refresh token
            if 'WHOOP_REFRESH_TOKEN=' in content:
                # Replace existing token
                import re
                content = re.sub(
                    r'WHOOP_REFRESH_TOKEN=.*',
                    f'WHOOP_REFRESH_TOKEN={refresh_token}',
                    content
                )
            else:
                # Add new token
                content += f'\nWHOOP_REFRESH_TOKEN={refresh_token}'
            
            with open(env_file, 'w') as f:
                f.write(content)
                
            logger.info("Updated .env file with new tokens")
            
        except Exception as e:
            logger.warning(f"Failed to update .env file: {e}")
    
    def _send_token_expiry_alert(self) -> None:
        """Send alert when tokens expire and manual re-authentication is needed"""
        try:
            alert_email = os.getenv('ALERT_EMAIL')
            smtp_server = os.getenv('SMTP_SERVER')
            smtp_user = os.getenv('SMTP_USER')
            smtp_pass = os.getenv('SMTP_PASS')
            
            if not all([alert_email, smtp_server, smtp_user, smtp_pass]):
                logger.warning("Email alerting not configured - skipping token expiry notification")
                return
            
            msg = MIMEMultipart()
            msg['From'] = smtp_user
            msg['To'] = alert_email
            msg['Subject'] = "🚨 Whoop Collector: Token Re-authentication Required"
            
            body = f"""
            Whoop Data Collector Alert
            
            The Whoop API refresh token has expired and manual re-authentication is required.
            
            Action Required:
            1. Run: python oauth_helper.py
            2. Complete OAuth flow in browser
            3. Update .env file with new tokens
            4. Restart the collector container
            
            Container: whoop-collector
            Time: {datetime.now().strftime('%Y-%m-%d %H:%M:%S UTC')}
            
            The collector will continue attempting to run every 6 hours but will fail until tokens are refreshed.
            """
            
            msg.attach(MIMEText(body, 'plain'))
            
            server = smtplib.SMTP(smtp_server, 587)
            server.starttls()
            server.login(smtp_user, smtp_pass)
            server.send_message(msg)
            server.quit()
            
            logger.info(f"Token expiry alert sent to {alert_email}")
            
        except Exception as e:
            logger.error(f"Failed to send token expiry alert: {e}")
        
    def test_whoop_connection(self) -> bool:
        """Test connection to Whoop API v2
        
        Returns:
            bool: True if connection successful, False if tokens are expired
        """
        try:
            # Test API connection with user profile endpoint
            response = requests.get(f"{self.base_url}/developer/v1/user/profile/basic", headers=self.headers)
            response.raise_for_status()
            logger.info("Successfully connected to Whoop API v2")
            return True
        except requests.exceptions.HTTPError as e:
            if e.response.status_code == 401 and self.refresh_token:
                logger.info("Access token expired, refreshing...")
                if self.refresh_access_token():
                    # Update headers and retry
                    self.headers["Authorization"] = f"Bearer {self.access_token}"
                    try:
                        response = requests.get(f"{self.base_url}/developer/v1/user/profile/basic", headers=self.headers)
                        response.raise_for_status()
                        logger.info("Successfully connected to Whoop API v2 after token refresh")
                        return True
                    except Exception as retry_e:
                        logger.error(f"Failed to connect after token refresh: {retry_e}")
                        return False
                else:
                    logger.error("Cannot refresh tokens - manual re-authentication required")
                    return False
            else:
                logger.error(f"Failed to connect to Whoop API v2: {e}")
                return False
        except Exception as e:
            logger.error(f"Failed to connect to Whoop API v2: {e}")
            return False
            
    def connect_database(self) -> None:
        """Connect to PostgreSQL database"""
        try:
            self.db_conn = psycopg2.connect(self.database_url)
            self.db_conn.set_isolation_level(psycopg2.extensions.ISOLATION_LEVEL_AUTOCOMMIT)
            logger.info("Successfully connected to database")
        except Exception as e:
            logger.error(f"Failed to connect to database: {e}")
            raise
            
    def get_last_whoop_data_date(self) -> Optional[str]:
        """Get the date of the last Whoop data point in the database
        
        Returns:
            Optional[str]: ISO date string of last data point, or None if no data found
        """
        cursor = self.db_conn.cursor()
        
        try:
            # Query for the most recent Whoop data across all metrics
            query = """
                SELECT MAX(matcheddate) 
                FROM raw_data 
                WHERE source = 'whoop' 
                AND key LIKE 'whoop%'
            """
            
            cursor.execute(query)
            result = cursor.fetchone()
            
            if result and result[0]:
                last_date = result[0]
                logger.info(f"Last Whoop data found: {last_date}")
                return last_date.strftime('%Y-%m-%d')
            else:
                logger.info("No previous Whoop data found in database")
                return None
                
        except Exception as e:
            logger.error(f"Error querying last Whoop data date: {e}")
            return None
        finally:
            cursor.close()
            
    def get_whoop_data(self, endpoint: str, start_date: str, end_date: str) -> List[Dict[str, Any]]:
        """Fetch data from Whoop API v2 for date range with pagination"""
        all_records = []
        next_token = None
        
        try:
            while True:
                # Build URL with parameters
                url = f"{self.base_url}/{endpoint}"
                params = {
                    "start": start_date,
                    "end": end_date,
                    "limit": 25  # API default limit
                }
                if next_token:
                    params["nextToken"] = next_token
                
                response = requests.get(url, headers=self.headers, params=params)
                response.raise_for_status()
                data = response.json()
                
                # Add records to collection
                if "records" in data:
                    all_records.extend(data["records"])
                    logger.info(f"Retrieved {len(data['records'])} records from {endpoint}")
                
                # Check for more pages
                next_token = data.get("next_token")
                if not next_token:
                    break
                    
            logger.info(f"Retrieved {len(all_records)} total records from {endpoint}")
            return all_records
            
        except Exception as e:
            logger.error(f"Failed to fetch data from {endpoint}: {e}")
            raise
            
    def iso_to_components(self, iso_datetime: str) -> Dict[str, Any]:
        """Convert ISO datetime to components for FxLifeSheet schema"""
        # Parse ISO datetime (handles both Z and timezone offsets)
        if iso_datetime.endswith('Z'):
            dt = datetime.fromisoformat(iso_datetime.replace('Z', '+00:00'))
        else:
            dt = datetime.fromisoformat(iso_datetime)
            
        # Convert to timestamp for database
        timestamp = int(dt.timestamp() * 1000)
        
        return {
            'timestamp': timestamp,
            'yearmonth': dt.year * 100 + dt.month,
            'yearweek': dt.year * 100 + dt.isocalendar()[1],
            'year': dt.year,
            'quarter': (dt.month - 1) // 3 + 1,
            'month': dt.month,
            'day': dt.day,
            'hour': dt.hour,
            'minute': dt.minute,
            'week': dt.isocalendar()[1],
            'matcheddate': dt.date()
        }
        
    def insert_raw_data(self, key: str, question: str, data_type: str, value: str, components: Dict[str, Any]) -> bool:
        """Insert data into raw_data table with duplicate detection
        
        Returns:
            bool: True if record was inserted, False if duplicate was skipped
        """
        
        cursor = self.db_conn.cursor()
        
        # First check if this record already exists
        check_query = """
            SELECT COUNT(*) FROM raw_data 
            WHERE key = %s AND question = %s AND matcheddate = %s AND source = 'whoop' AND value = %s
        """
        
        cursor.execute(check_query, (key, question, components['matcheddate'], value))
        count = cursor.fetchone()[0]
        
        if count > 0:
            logger.info(f"SKIP: {key} - {question} for {components['matcheddate']} (duplicate)")
            return False
        
        # Insert new record
        insert_query = """
            INSERT INTO raw_data (
                timestamp, yearmonth, yearweek, year, quarter, month, day, 
                hour, minute, week, key, question, type, value, matcheddate,
                source, importedat, importid
            ) VALUES (
                %s, %s, %s, %s, %s, %s, %s, %s, %s, %s, %s, %s, %s, %s, %s, %s, %s, %s
            )
        """
        
        cursor.execute(insert_query, (
            components['timestamp'], components['yearmonth'], components['yearweek'], 
            components['year'], components['quarter'], components['month'], 
            components['day'], components['hour'], components['minute'], 
            components['week'], key, question, data_type, value, 
            components['matcheddate'], 'whoop', self.import_timestamp, self.import_id
        ))
        
        if cursor.rowcount > 0:
            logger.info(f"INSERT: {key} - {question} for {components['matcheddate']} = {value}")
            return True
        return False
        
    def process_cycles_and_recovery(self, cycle_data: List[Dict[str, Any]]) -> None:
        """Fetch and process recovery data for each cycle"""
        records_inserted = 0
        records_skipped = 0
        date_range = set()

        for cycle in cycle_data:
            try:
                cycle_id = cycle.get('id')
                if not cycle_id:
                    logger.warning("Cycle record missing ID, skipping")
                    continue

                # Fetch recovery data for this cycle
                try:
                    recovery_url = f"{self.base_url}/developer/v2/cycle/{cycle_id}/recovery"
                    response = requests.get(recovery_url, headers=self.headers)

                    if response.status_code == 404:
                        # No recovery data for this cycle yet
                        continue

                    response.raise_for_status()
                    recovery = response.json()

                except requests.exceptions.HTTPError as e:
                    if e.response.status_code == 404:
                        # No recovery data available for this cycle
                        continue
                    logger.warning(f"Failed to fetch recovery for cycle {cycle_id}: {e}")
                    continue

                # Get timestamp from recovery record
                timestamp_str = recovery.get('created_at') or recovery.get('updated_at')
                if not timestamp_str:
                    logger.warning(f"No timestamp in recovery for cycle {cycle_id}")
                    continue

                components = self.iso_to_components(timestamp_str)
                date_range.add(components['matcheddate'])

                # Check if recovery is scored
                if recovery.get('score_state') != 'SCORED':
                    continue

                # Extract recovery metrics
                if 'score' in recovery:
                    score_data = recovery['score']

                    # Recovery Score (0-100 percentage)
                    if 'recovery_score' in score_data:
                        if self.insert_raw_data(
                            'whoopRecoveryScore',
                            'Whoop Recovery Score',
                            'number',
                            str(score_data['recovery_score']),
                            components
                        ):
                            records_inserted += 1
                        else:
                            records_skipped += 1

                    # Resting Heart Rate
                    if 'resting_heart_rate' in score_data:
                        if self.insert_raw_data(
                            'whoopRHR',
                            'Whoop Resting Heart Rate',
                            'number',
                            str(score_data['resting_heart_rate']),
                            components
                        ):
                            records_inserted += 1
                        else:
                            records_skipped += 1

                    # Heart Rate Variability
                    if 'hrv_rmssd_milli' in score_data:
                        if self.insert_raw_data(
                            'whoopHRV',
                            'Whoop Heart Rate Variability',
                            'number',
                            str(score_data['hrv_rmssd_milli']),
                            components
                        ):
                            records_inserted += 1
                        else:
                            records_skipped += 1

                    # Skin Temperature
                    if 'skin_temp_celsius' in score_data:
                        if self.insert_raw_data(
                            'whoopSkinTemp',
                            'Whoop Skin Temperature',
                            'number',
                            str(score_data['skin_temp_celsius']),
                            components
                        ):
                            records_inserted += 1
                        else:
                            records_skipped += 1

                    # SpO2 (new in v2)
                    if 'spo2_percentage' in score_data:
                        if self.insert_raw_data(
                            'whoopSpO2',
                            'Whoop Blood Oxygen (SpO2)',
                            'number',
                            str(score_data['spo2_percentage']),
                            components
                        ):
                            records_inserted += 1
                        else:
                            records_skipped += 1

            except Exception as e:
                logger.error(f"Error processing cycle {cycle.get('id', 'unknown')}: {e}")
                continue

        if date_range:
            date_min, date_max = min(date_range), max(date_range)
            logger.info(f"Recovery data: {records_inserted} inserted, {records_skipped} skipped (dates: {date_min} to {date_max})")
        else:
            logger.info(f"Recovery data: No records processed")
        
    def process_sleep_data(self, sleep_data: List[Dict[str, Any]]) -> None:
        """Process and insert v2 sleep data into database"""
        records_inserted = 0
        records_skipped = 0
        date_range = set()
        
        for sleep in sleep_data:
            try:
                # Get timestamp from sleep start time
                start_time = sleep.get('start')
                if not start_time:
                    logger.warning("No start time found for sleep record, skipping")
                    continue
                    
                components = self.iso_to_components(start_time)
                
                # Track date for summary
                date_range.add(components['matcheddate'])
                
                # Extract sleep metrics from v2 API structure
                if sleep.get('score_state') == 'SCORED' and 'score' in sleep:
                    score_data = sleep['score']
                    
                    # Sleep Performance Percentage
                    if 'sleep_performance_percentage' in score_data:
                        if self.insert_raw_data(
                            'whoopSleepPerformance', 
                            'Whoop Sleep Performance', 
                            'number', 
                            str(score_data['sleep_performance_percentage']), 
                            components
                        ):
                            records_inserted += 1
                        else:
                            records_skipped += 1
                    
                    # Sleep Efficiency Percentage
                    if 'sleep_efficiency_percentage' in score_data:
                        if self.insert_raw_data(
                            'whoopSleepEfficiency', 
                            'Whoop Sleep Efficiency', 
                            'number', 
                            str(score_data['sleep_efficiency_percentage']), 
                            components
                        ):
                            records_inserted += 1
                        else:
                            records_skipped += 1
                    
                    # Respiratory Rate
                    if 'respiratory_rate' in score_data:
                        if self.insert_raw_data(
                            'whoopRespiratoryRate', 
                            'Whoop Respiratory Rate', 
                            'number', 
                            str(score_data['respiratory_rate']), 
                            components
                        ):
                            records_inserted += 1
                        else:
                            records_skipped += 1
                        
                    # Sleep Consistency Percentage
                    if 'sleep_consistency_percentage' in score_data:
                        if self.insert_raw_data(
                            'whoopSleepConsistency', 
                            'Whoop Sleep Consistency', 
                            'number', 
                            str(score_data['sleep_consistency_percentage']), 
                            components
                        ):
                            records_inserted += 1
                        else:
                            records_skipped += 1
                
            except Exception as e:
                logger.error(f"Error processing sleep record: {e}")
                continue
                
        if date_range:
            date_min, date_max = min(date_range), max(date_range)
            logger.info(f"Sleep data: {records_inserted} inserted, {records_skipped} skipped (dates: {date_min} to {date_max})")
        else:
            logger.info(f"Sleep data: No records processed")
        
    def process_workout_data(self, workout_data: List[Dict[str, Any]]) -> None:
        """Process and insert v2 workout data into database"""
        records_inserted = 0
        records_skipped = 0
        date_range = set()
        
        for workout in workout_data:
            try:
                # Get timestamp from workout start time
                start_time = workout.get('start')
                if not start_time:
                    logger.warning("No start time found for workout record, skipping")
                    continue
                    
                components = self.iso_to_components(start_time)
                
                # Track date for summary
                date_range.add(components['matcheddate'])
                
                # Extract workout metrics from v2 API structure
                if workout.get('score_state') == 'SCORED' and 'score' in workout:
                    score_data = workout['score']
                    
                    # Strain Score
                    if 'strain' in score_data:
                        if self.insert_raw_data(
                            'whoopStrain', 
                            'Whoop Strain Score', 
                            'number', 
                            str(score_data['strain']), 
                            components
                        ):
                            records_inserted += 1
                        else:
                            records_skipped += 1
                    
                    # Average Heart Rate
                    if 'average_heart_rate' in score_data:
                        if self.insert_raw_data(
                            'whoopAvgHeartRate', 
                            'Whoop Average Heart Rate', 
                            'number', 
                            str(score_data['average_heart_rate']), 
                            components
                        ):
                            records_inserted += 1
                        else:
                            records_skipped += 1
                    
                    # Max Heart Rate
                    if 'max_heart_rate' in score_data:
                        if self.insert_raw_data(
                            'whoopMaxHeartRate', 
                            'Whoop Max Heart Rate', 
                            'number', 
                            str(score_data['max_heart_rate']), 
                            components
                        ):
                            records_inserted += 1
                        else:
                            records_skipped += 1
                        
                    # Kilojoules (Energy)
                    if 'kilojoule' in score_data:
                        if self.insert_raw_data(
                            'whoopKilojoules', 
                            'Whoop Kilojoules', 
                            'number', 
                            str(score_data['kilojoule']), 
                            components
                        ):
                            records_inserted += 1
                        else:
                            records_skipped += 1
                        
                    # Distance in meters
                    if 'distance_meter' in score_data:
                        if self.insert_raw_data(
                            'whoopDistance', 
                            'Whoop Distance (meters)', 
                            'number', 
                            str(score_data['distance_meter']), 
                            components
                        ):
                            records_inserted += 1
                        else:
                            records_skipped += 1
                
            except Exception as e:
                logger.error(f"Error processing workout record: {e}")
                continue
                
        if date_range:
            date_min, date_max = min(date_range), max(date_range)
            logger.info(f"Workout data: {records_inserted} inserted, {records_skipped} skipped (dates: {date_min} to {date_max})")
        else:
            logger.info(f"Workout data: No records processed")
        
    def run(self, date: Optional[str] = None, days: int = 1, from_last: bool = False) -> None:
        """Run the import process with robust error handling"""
        try:
            # Connect to database first so we can query for last data point
            self.connect_database()
            
            # Set up date range in ISO format for v2 API
            if date:
                start_date = datetime.strptime(date, '%Y-%m-%d')
            elif from_last:
                # Find the last data point and start from the next day
                last_date_str = self.get_last_whoop_data_date()
                if last_date_str:
                    last_date = datetime.strptime(last_date_str, '%Y-%m-%d')
                    start_date = last_date + timedelta(days=1)
                    
                    # Check if start date is in the future
                    today = datetime.now().date()
                    if start_date.date() > today:
                        logger.info(f"Already up to date. Last data: {last_date_str}, today: {today}")
                        return
                    
                    logger.info(f"Starting import from day after last data point: {start_date.strftime('%Y-%m-%d')}")
                else:
                    # No previous data found, start from 30 days ago
                    start_date = datetime.now() - timedelta(days=30)
                    logger.info(f"No previous data found, starting from 30 days ago: {start_date.strftime('%Y-%m-%d')}")
            else:
                start_date = datetime.now() - timedelta(days=days)
                
            # For from_last mode, end date is today
            if from_last:
                end_date = datetime.now()
            else:
                end_date = start_date + timedelta(days=days)
            
            # v2 API expects ISO format timestamps
            start_date_str = start_date.strftime('%Y-%m-%dT00:00:00.000Z')
            end_date_str = end_date.strftime('%Y-%m-%dT23:59:59.999Z')
            
            logger.info(f"Importing Whoop v2 data from {start_date.strftime('%Y-%m-%d')} to {end_date.strftime('%Y-%m-%d')}")
            
            # Ensure we have valid access token and connect to services
            self.ensure_valid_access_token()
            
            if not self.test_whoop_connection():
                logger.error("Cannot connect to Whoop API - tokens may be expired. Skipping this run.")
                return
            
            # Fetch cycles and then get recovery data for each cycle
            logger.info("Fetching cycles...")
            cycle_data = self.get_whoop_data('developer/v2/cycle', start_date_str, end_date_str)

            if cycle_data:
                logger.info(f"Fetching recovery data for {len(cycle_data)} cycles...")
                self.process_cycles_and_recovery(cycle_data)
            else:
                logger.info("No cycle data found for date range")
            
            # Fetch and process sleep data
            logger.info("Fetching sleep data...")
            sleep_data = self.get_whoop_data('developer/v2/activity/sleep', start_date_str, end_date_str)
            if sleep_data:
                self.process_sleep_data(sleep_data)
            else:
                logger.info("No sleep data found for date range")
            
            # Fetch and process workout data
            logger.info("Fetching workout data...")
            workout_data = self.get_whoop_data('developer/v2/activity/workout', start_date_str, end_date_str)
            if workout_data:
                self.process_workout_data(workout_data)
            else:
                logger.info("No workout data found for date range")
            
            logger.info("Whoop v2 data import completed successfully")
            
        except Exception as e:
            logger.error(f"Import failed: {e}")
            # Don't raise exception in production - let cron continue with next scheduled run
            # Only exit with error code for debugging purposes when run manually
            if os.getenv('PRODUCTION_MODE', 'true').lower() == 'true':
                logger.info("Production mode: continuing despite error - next run in 6 hours")
            else:
                raise
        finally:
            if self.db_conn:
                self.db_conn.close()
                
def main():
    parser = argparse.ArgumentParser(description='Import Whoop data to FxLifeSheet')
    parser.add_argument('--date', type=str, help='Specific date to import (YYYY-MM-DD)')
    parser.add_argument('--days', type=int, default=1, help='Number of days to import (default: 1)')
    parser.add_argument('--from-last', action='store_true', help='Import from the last data point to today')
    
    args = parser.parse_args()
    
    try:
        importer = WhoopImporter()
        importer.run(date=args.date, days=args.days, from_last=args.from_last)
    except Exception as e:
        logger.error(f"Script failed: {e}")
        sys.exit(1)

if __name__ == "__main__":
    main()
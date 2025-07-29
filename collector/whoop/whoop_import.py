#!/usr/bin/env python3
"""
Whoop Data Import Script for FxLifeSheet (v2 API)

This script fetches recovery, sleep, and workout data from Whoop v2 API
using OAuth 2.0 authentication and inserts it into the FxLifeSheet database.

Usage:
    python whoop_import.py [--date YYYY-MM-DD] [--days N]

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
        
    def refresh_access_token(self) -> None:
        """Refresh access token using refresh token"""
        try:
            oauth_helper = WhoopOAuthHelper()
            token_response = oauth_helper.refresh_access_token(self.refresh_token)
            
            self.access_token = token_response.get('access_token')
            if not self.access_token:
                raise ValueError("No access token received from refresh")
                
            # Update refresh token if a new one was provided
            if 'refresh_token' in token_response:
                self.refresh_token = token_response['refresh_token']
                
            logger.info("Successfully refreshed access token")
            
        except Exception as e:
            logger.error(f"Failed to refresh access token: {e}")
            raise
        
    def test_whoop_connection(self) -> None:
        """Test connection to Whoop API v2"""
        try:
            # Test API connection with user profile endpoint
            response = requests.get(f"{self.base_url}/developer/v1/user/profile/basic", headers=self.headers)
            response.raise_for_status()
            logger.info("Successfully connected to Whoop API v2")
        except Exception as e:
            logger.error(f"Failed to connect to Whoop API v2: {e}")
            raise
            
    def connect_database(self) -> None:
        """Connect to PostgreSQL database"""
        try:
            self.db_conn = psycopg2.connect(self.database_url)
            self.db_conn.set_isolation_level(psycopg2.extensions.ISOLATION_LEVEL_AUTOCOMMIT)
            logger.info("Successfully connected to database")
        except Exception as e:
            logger.error(f"Failed to connect to database: {e}")
            raise
            
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
        
    def insert_raw_data(self, key: str, question: str, data_type: str, value: str, components: Dict[str, Any]) -> None:
        """Insert data into raw_data table"""
        
        cursor = self.db_conn.cursor()
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
        
    def process_recovery_data(self, recovery_data: List[Dict[str, Any]]) -> None:
        """Process and insert v2 recovery data into database"""
        records_inserted = 0
        
        for recovery in recovery_data:
            try:
                # Get timestamp from created_at or updated_at
                timestamp_str = recovery.get('created_at') or recovery.get('updated_at')
                if not timestamp_str:
                    logger.warning("No timestamp found for recovery record, skipping")
                    continue
                    
                components = self.iso_to_components(timestamp_str)
                
                # Extract recovery metrics from v2 API structure
                if recovery.get('score_state') == 'SCORED' and 'score' in recovery:
                    score_data = recovery['score']
                    
                    # Recovery Score (0-100 percentage)
                    if 'recovery_score' in score_data:
                        self.insert_raw_data(
                            'whoopRecoveryScore', 
                            'Whoop Recovery Score', 
                            'number', 
                            str(score_data['recovery_score']), 
                            components
                        )
                        records_inserted += 1
                    
                    # Resting Heart Rate
                    if 'resting_heart_rate' in score_data:
                        self.insert_raw_data(
                            'whoopRHR', 
                            'Whoop Resting Heart Rate', 
                            'number', 
                            str(score_data['resting_heart_rate']), 
                            components
                        )
                        records_inserted += 1
                    
                    # Heart Rate Variability
                    if 'hrv_rmssd_milli' in score_data:
                        self.insert_raw_data(
                            'whoopHRV', 
                            'Whoop Heart Rate Variability', 
                            'number', 
                            str(score_data['hrv_rmssd_milli']), 
                            components
                        )
                        records_inserted += 1
                        
                    # Additional v2 metrics
                    if 'skin_temp_celsius' in score_data:
                        self.insert_raw_data(
                            'whoopSkinTemp', 
                            'Whoop Skin Temperature', 
                            'number', 
                            str(score_data['skin_temp_celsius']), 
                            components
                        )
                        records_inserted += 1
                
            except Exception as e:
                logger.error(f"Error processing recovery record: {e}")
                continue
                
        logger.info(f"Successfully inserted {records_inserted} recovery records into database")
        
    def process_sleep_data(self, sleep_data: List[Dict[str, Any]]) -> None:
        """Process and insert v2 sleep data into database"""
        records_inserted = 0
        
        for sleep in sleep_data:
            try:
                # Get timestamp from sleep start time
                start_time = sleep.get('start')
                if not start_time:
                    logger.warning("No start time found for sleep record, skipping")
                    continue
                    
                components = self.iso_to_components(start_time)
                
                # Extract sleep metrics from v2 API structure
                if sleep.get('score_state') == 'SCORED' and 'score' in sleep:
                    score_data = sleep['score']
                    
                    # Sleep Performance Percentage
                    if 'sleep_performance_percentage' in score_data:
                        self.insert_raw_data(
                            'whoopSleepPerformance', 
                            'Whoop Sleep Performance', 
                            'number', 
                            str(score_data['sleep_performance_percentage']), 
                            components
                        )
                        records_inserted += 1
                    
                    # Sleep Efficiency Percentage
                    if 'sleep_efficiency_percentage' in score_data:
                        self.insert_raw_data(
                            'whoopSleepEfficiency', 
                            'Whoop Sleep Efficiency', 
                            'number', 
                            str(score_data['sleep_efficiency_percentage']), 
                            components
                        )
                        records_inserted += 1
                    
                    # Respiratory Rate
                    if 'respiratory_rate' in score_data:
                        self.insert_raw_data(
                            'whoopRespiratoryRate', 
                            'Whoop Respiratory Rate', 
                            'number', 
                            str(score_data['respiratory_rate']), 
                            components
                        )
                        records_inserted += 1
                        
                    # Sleep Consistency Percentage
                    if 'sleep_consistency_percentage' in score_data:
                        self.insert_raw_data(
                            'whoopSleepConsistency', 
                            'Whoop Sleep Consistency', 
                            'number', 
                            str(score_data['sleep_consistency_percentage']), 
                            components
                        )
                        records_inserted += 1
                
            except Exception as e:
                logger.error(f"Error processing sleep record: {e}")
                continue
                
        logger.info(f"Successfully inserted {records_inserted} sleep records into database")
        
    def process_workout_data(self, workout_data: List[Dict[str, Any]]) -> None:
        """Process and insert v2 workout data into database"""
        records_inserted = 0
        
        for workout in workout_data:
            try:
                # Get timestamp from workout start time
                start_time = workout.get('start')
                if not start_time:
                    logger.warning("No start time found for workout record, skipping")
                    continue
                    
                components = self.iso_to_components(start_time)
                
                # Extract workout metrics from v2 API structure
                if workout.get('score_state') == 'SCORED' and 'score' in workout:
                    score_data = workout['score']
                    
                    # Strain Score
                    if 'strain' in score_data:
                        self.insert_raw_data(
                            'whoopStrain', 
                            'Whoop Strain Score', 
                            'number', 
                            str(score_data['strain']), 
                            components
                        )
                        records_inserted += 1
                    
                    # Average Heart Rate
                    if 'average_heart_rate' in score_data:
                        self.insert_raw_data(
                            'whoopAvgHeartRate', 
                            'Whoop Average Heart Rate', 
                            'number', 
                            str(score_data['average_heart_rate']), 
                            components
                        )
                        records_inserted += 1
                    
                    # Max Heart Rate
                    if 'max_heart_rate' in score_data:
                        self.insert_raw_data(
                            'whoopMaxHeartRate', 
                            'Whoop Max Heart Rate', 
                            'number', 
                            str(score_data['max_heart_rate']), 
                            components
                        )
                        records_inserted += 1
                        
                    # Kilojoules (Energy)
                    if 'kilojoule' in score_data:
                        self.insert_raw_data(
                            'whoopKilojoules', 
                            'Whoop Kilojoules', 
                            'number', 
                            str(score_data['kilojoule']), 
                            components
                        )
                        records_inserted += 1
                        
                    # Distance in meters
                    if 'distance_meter' in score_data:
                        self.insert_raw_data(
                            'whoopDistance', 
                            'Whoop Distance (meters)', 
                            'number', 
                            str(score_data['distance_meter']), 
                            components
                        )
                        records_inserted += 1
                
            except Exception as e:
                logger.error(f"Error processing workout record: {e}")
                continue
                
        logger.info(f"Successfully inserted {records_inserted} workout records into database")
        
    def run(self, date: Optional[str] = None, days: int = 1) -> None:
        """Run the import process"""
        try:
            # Set up date range in ISO format for v2 API
            if date:
                start_date = datetime.strptime(date, '%Y-%m-%d')
            else:
                start_date = datetime.now() - timedelta(days=days)
                
            end_date = start_date + timedelta(days=days)
            
            # v2 API expects ISO format timestamps
            start_date_str = start_date.strftime('%Y-%m-%dT00:00:00.000Z')
            end_date_str = end_date.strftime('%Y-%m-%dT23:59:59.999Z')
            
            logger.info(f"Importing Whoop v2 data from {start_date.strftime('%Y-%m-%d')} to {end_date.strftime('%Y-%m-%d')}")
            
            # Ensure we have valid access token and connect to services
            self.ensure_valid_access_token()
            self.test_whoop_connection()
            self.connect_database()
            
            # Fetch and process recovery data from cycles (contains recovery info)
            logger.info("Fetching recovery data...")
            recovery_data = self.get_whoop_data('developer/v1/recovery', start_date_str, end_date_str)
            
            if recovery_data:
                self.process_recovery_data(recovery_data)
            else:
                logger.info("No recovery data found for date range")
            
            # Fetch and process sleep data
            logger.info("Fetching sleep data...")
            sleep_data = self.get_whoop_data('developer/v1/activity/sleep', start_date_str, end_date_str)
            if sleep_data:
                self.process_sleep_data(sleep_data)
            else:
                logger.info("No sleep data found for date range")
            
            # Fetch and process workout data
            logger.info("Fetching workout data...")
            workout_data = self.get_whoop_data('developer/v1/activity/workout', start_date_str, end_date_str)
            if workout_data:
                self.process_workout_data(workout_data)
            else:
                logger.info("No workout data found for date range")
            
            logger.info("Whoop v2 data import completed successfully")
            
        except Exception as e:
            logger.error(f"Import failed: {e}")
            raise
        finally:
            if self.db_conn:
                self.db_conn.close()
                
def main():
    parser = argparse.ArgumentParser(description='Import Whoop data to FxLifeSheet')
    parser.add_argument('--date', type=str, help='Specific date to import (YYYY-MM-DD)')
    parser.add_argument('--days', type=int, default=1, help='Number of days to import (default: 1)')
    
    args = parser.parse_args()
    
    try:
        importer = WhoopImporter()
        importer.run(date=args.date, days=args.days)
    except Exception as e:
        logger.error(f"Script failed: {e}")
        sys.exit(1)

if __name__ == "__main__":
    main()
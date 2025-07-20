#!/usr/bin/env python3
"""
Whoop OAuth 2.0 Helper Script

This script helps you obtain an access token for the Whoop API v2
using the OAuth 2.0 authorization code flow.

Usage:
    python oauth_helper.py

Environment Variables:
    WHOOP_CLIENT_ID - Your Whoop app client ID
    WHOOP_CLIENT_SECRET - Your Whoop app client secret
    WHOOP_REDIRECT_URI - Your registered redirect URI
"""

import os
import sys
import urllib.parse
import secrets
import base64
import hashlib
from dotenv import load_dotenv
import requests

# Load environment variables
load_dotenv()

class WhoopOAuthHelper:
    def __init__(self):
        self.client_id = os.getenv("WHOOP_CLIENT_ID")
        self.client_secret = os.getenv("WHOOP_CLIENT_SECRET")
        self.redirect_uri = os.getenv("WHOOP_REDIRECT_URI", "http://localhost:8080/callback")
        
        if not all([self.client_id, self.client_secret]):
            raise ValueError("Missing required environment variables: WHOOP_CLIENT_ID, WHOOP_CLIENT_SECRET")
        
        self.auth_url = "https://api.prod.whoop.com/oauth/oauth2/auth"
        self.token_url = "https://api.prod.whoop.com/oauth/oauth2/token"
        
    def generate_pkce_challenge(self):
        """Generate PKCE code verifier and challenge for enhanced security"""
        code_verifier = base64.urlsafe_b64encode(secrets.token_bytes(32)).decode('utf-8')
        code_verifier = code_verifier.rstrip('=')
        
        code_challenge = base64.urlsafe_b64encode(
            hashlib.sha256(code_verifier.encode('utf-8')).digest()
        ).decode('utf-8')
        code_challenge = code_challenge.rstrip('=')
        
        return code_verifier, code_challenge
        
    def get_authorization_url(self):
        """Generate authorization URL for user consent"""
        # Generate state for CSRF protection
        state = secrets.token_urlsafe(32)
        
        # Generate PKCE challenge
        code_verifier, code_challenge = self.generate_pkce_challenge()
        
        # Build authorization URL
        params = {
            'client_id': self.client_id,
            'redirect_uri': self.redirect_uri,
            'response_type': 'code',
            'scope': 'read:recovery read:sleep read:workout read:profile',
            'state': state,
            'code_challenge': code_challenge,
            'code_challenge_method': 'S256'
        }
        
        auth_url = f"{self.auth_url}?{urllib.parse.urlencode(params)}"
        
        return auth_url, state, code_verifier
        
    def exchange_code_for_token(self, authorization_code, code_verifier):
        """Exchange authorization code for access token"""
        token_data = {
            'grant_type': 'authorization_code',
            'client_id': self.client_id,
            'client_secret': self.client_secret,
            'code': authorization_code,
            'redirect_uri': self.redirect_uri,
            'code_verifier': code_verifier
        }
        
        response = requests.post(self.token_url, data=token_data)
        response.raise_for_status()
        
        return response.json()
        
    def refresh_access_token(self, refresh_token):
        """Refresh access token using refresh token"""
        token_data = {
            'grant_type': 'refresh_token',
            'client_id': self.client_id,
            'client_secret': self.client_secret,
            'refresh_token': refresh_token
        }
        
        response = requests.post(self.token_url, data=token_data)
        response.raise_for_status()
        
        return response.json()

def main():
    print("Whoop OAuth 2.0 Helper")
    print("=" * 30)
    
    try:
        helper = WhoopOAuthHelper()
        
        # Generate authorization URL
        auth_url, state, code_verifier = helper.get_authorization_url()
        
        print("1. Open the following URL in your browser:")
        print(f"   {auth_url}")
        print()
        print("2. Log in to your Whoop account and authorize the application")
        print()
        print("3. After authorization, you'll be redirected to your redirect URI")
        print("   Copy the 'code' parameter from the redirect URL")
        print()
        
        # Get authorization code from user
        authorization_code = input("Enter the authorization code: ").strip()
        
        if not authorization_code:
            print("Error: No authorization code provided")
            sys.exit(1)
            
        print()
        print("Exchanging code for access token...")
        
        # Exchange code for token
        token_response = helper.exchange_code_for_token(authorization_code, code_verifier)
        
        print("Success! Here are your tokens:")
        print("=" * 40)
        print(f"Access Token: {token_response.get('access_token')}")
        print(f"Token Type: {token_response.get('token_type', 'Bearer')}")
        print(f"Expires In: {token_response.get('expires_in')} seconds")
        
        if 'refresh_token' in token_response:
            print(f"Refresh Token: {token_response.get('refresh_token')}")
            
        print()
        print("Add this to your environment variables:")
        print(f"WHOOP_ACCESS_TOKEN={token_response.get('access_token')}")
        
        if 'refresh_token' in token_response:
            print(f"WHOOP_REFRESH_TOKEN={token_response.get('refresh_token')}")
            
        print()
        print("You can now use the whoop_import.py script!")
        
    except Exception as e:
        print(f"Error: {e}")
        sys.exit(1)

if __name__ == "__main__":
    main()
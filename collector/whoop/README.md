# Whoop Data Import Script (v2 API)

This script imports recovery, sleep, and workout data from Whoop v2 API using OAuth 2.0 authentication into your FxLifeSheet database.

## Setup

1. Install dependencies:
   ```bash
   cd collector/whoop
   pip install -r requirements.txt
   ```

2. Create Whoop Developer App:
   - Go to [Whoop Developer Dashboard](https://developer.whoop.com/)
   - Create a new app and note your Client ID and Client Secret
   - Set up OAuth 2.0 redirect URLs for your application

3. Get Access Token:
   - Implement OAuth 2.0 flow to get access token
   - Or use existing OAuth libraries to obtain user consent
   - Store the access token securely

4. Configure your credentials in environment variables or `.env` file:
   - `WHOOP_ACCESS_TOKEN`: OAuth 2.0 access token for API access
   - `WHOOP_CLIENT_ID`: Your app's client ID (optional, for token refresh)
   - `WHOOP_CLIENT_SECRET`: Your app's client secret (optional, for token refresh)
   - `DATABASE_URL`: PostgreSQL connection string (same as telegram bot)

## Usage

### Import today's data:
```bash
python whoop_import.py
```

### Import specific date:
```bash
python whoop_import.py --date 2024-01-15
```

### Import last 7 days:
```bash
python whoop_import.py --days 7
```

## Data Imported

### Recovery Data (from cycles endpoint)
- **Recovery Score**: Overall recovery percentage (0-100)
- **Resting Heart Rate**: RHR in beats per minute
- **Heart Rate Variability**: HRV in milliseconds
- **Skin Temperature**: Temperature in Celsius

### Sleep Data
- **Sleep Performance**: Sleep performance percentage
- **Sleep Efficiency**: Sleep efficiency percentage
- **Respiratory Rate**: Breathing rate during sleep
- **Sleep Consistency**: Sleep consistency percentage

### Workout Data
- **Strain Score**: Workout strain score
- **Average Heart Rate**: Average HR during workout
- **Max Heart Rate**: Maximum HR during workout
- **Kilojoules**: Energy expenditure
- **Distance**: Distance covered in meters

All data is stored in the `raw_data` table with:
- Source: "whoop"
- Import tracking with unique ID and timestamp
- Proper time series indexing for visualization

## Important Notes

- **Migration Required**: Whoop v1 API will be discontinued on October 1, 2025
- **OAuth 2.0**: This script uses the new OAuth 2.0 authentication method
- **Enhanced Data**: v2 API provides more comprehensive health and fitness metrics
- **Rate Limiting**: API calls are paginated and respect Whoop's rate limits

## Visualization

The imported data will appear in your FxLifeSheet dashboard with enhanced health and fitness tracking across recovery, sleep, and workout metrics.# Multi-arch build enabled
# ARM v7 build

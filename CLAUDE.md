# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## What is FxLifeSheet?

FxLifeSheet is a **comprehensive personal life tracking and optimization system** that helps you understand yourself better through data. It answers the question: "What factors actually influence my happiness, productivity, and health?"

### The Problem It Solves

Most people struggle to:
- Identify patterns in their mood, energy, and productivity
- Understand what lifestyle factors actually impact their well-being
- Remember how they were feeling weeks or months ago
- Make data-driven decisions about their health and habits
- Track progress towards personal goals across multiple dimensions

### What FxLifeSheet Does

FxLifeSheet collects, stores, and visualizes data about your life across multiple dimensions:

**🧠 Mental Health & Mood**
- Happiness, anxiety, and energy levels (tracked 8x daily)
- Excitement about the future
- Worries and concerns
- Daily accomplishments

**💪 Physical Health & Fitness**
- Sleep quality and consistency
- WHOOP metrics (recovery, HRV, heart rate, SpO2, strain)
- Workout details and progressive overload
- Diet and nutrition (meals, vegetables, alcohol, junk food)
- Intermittent fasting duration
- Breathing exercises

**📊 Productivity & Learning**
- Pomodoros completed
- Learning and skill development
- Reading and writing habits
- Meeting load
- Time spent on teaching and side projects

**💼 Work & Career**
- User empathy and impact
- Research and innovation
- Team growth observations
- Weekly reflections on what's working/not working

**👥 Social Life**
- Time with family, partner, and friends
- Socializing frequency

**🎯 Personal Growth**
- Personal branding efforts
- Publishing and writing
- Side projects
- Paper reading

### How It Works

1. **Easy Data Collection**: Interact with a Telegram bot throughout the day to log data
2. **Automated Import**: External devices (WHOOP) automatically sync health data
3. **Smart Scheduling**: Bot reminds you at the right times (morning, evening, weekly)
4. **Rich Visualization**: Web dashboard shows trends, correlations, and patterns
5. **Long-term Insights**: Years of data help you understand what actually matters

### Key Features

- **Multi-channel data collection**: Telegram bot (primary) + external API integrations
- **Flexible question system**: Easily add new metrics via JSON configuration
- **Time-series analysis**: Track trends over days, weeks, months, and years
- **Cross-metric correlations**: See how sleep affects mood, or workouts impact recovery
- **Manual and automated tracking**: Balance between conscious logging and passive monitoring
- **Privacy-first**: Self-hosted, you own all your data
- **Developer-friendly**: Open architecture, easy to extend and customize

### Who It's For

FxLifeSheet is designed for:
- **Quantified self enthusiasts** who want deep insights into their patterns
- **High-performers** optimizing health, productivity, and decision-making
- **Developers and founders** tracking work-life balance and business metrics
- **Athletes** monitoring recovery and performance
- **Anyone** serious about self-improvement through data

### Philosophy

FxLifeSheet is built on several core principles:

1. **Measure what matters**: Track inputs (behaviors) not just outputs (results)
2. **Long-term perspective**: Patterns emerge over months and years, not days
3. **Holistic view**: Physical health, mental well-being, and productivity are interconnected
4. **Data-driven decisions**: Let data guide your lifestyle changes, not guesswork
5. **Own your data**: Full control and privacy of personal information
6. **Sustainable tracking**: Make logging easy enough to maintain for years

## Technical Overview

FxLifeSheet is a comprehensive personal life tracking system with a microservices architecture consisting of:
- **Telegram Bot** (Node.js/TypeScript) - Primary data collection via chat interface
- **Visualization Backend** (Rust) - API server for data retrieval
- **Visualization Frontend** (React/TypeScript) - Web dashboard
- **PostgreSQL Database** - Central data storage (can be local or remote)
- **Collectors** - External data importers (WHOOP, etc.)
- **Subjective Dashboard** - Additional analysis tools

## Core Architecture

### Data Flow
1. User interacts with Telegram bot (`/awake`, `/asleep`, `/week`, `/mood`)
2. Bot reads questions from `lifesheet.json` (stored in Dropbox)
3. Responses stored in PostgreSQL `raw_data` table
4. Scheduler (`telegram_bot/src/scheduler.js`) sends reminders based on question schedules
5. External collectors (like WHOOP) import data directly to `raw_data` table
6. Visualization backend (Rust) serves data via REST API
7. Frontend renders charts and dashboards

### Key Database Schema

**raw_data** - Main data table:
- `timestamp` - Unix timestamp in milliseconds
- `key` - Metric identifier (e.g., 'mood', 'whoopRecoveryScore')
- `question` - Human-readable question text
- `type` - Data type (range, boolean, text, number, location)
- `value` - The recorded value
- `yearmonth`, `yearweek`, `year`, `quarter`, `month`, `day`, `hour`, `minute`, `week` - Time indexing
- `matcheddate` - Date for easy querying
- `source` - Data source ('telegram' or 'whoop')
- `importid`, `importedat` - Import tracking

**questions** - Metadata table:
- `key` - Matches key in raw_data
- `question` - Question text
- `question_type` - Type of question
- `is_visible_in_visualizer` - Whether to show in dashboard
- `category` - Classification (Physical Health, Mental Health, Productivity, etc.)
- `display_name` - UI label
- `is_positive` - Whether higher values are better
- `is_reverse` - Reverse scoring
- `graph_type` - Visualization type (line, bar, etc.)

**Other tables**:
- `last_run` - Tracks when questions were last answered
- `metadata` - System metadata storage
- `category` - Question categories for organization

## Configuration Management

### lifesheet.json
- **Location**: `~/Dropbox/Public/lifesheet.json`
- **Purpose**: Defines all trackable metrics and their properties
- **Structure**: Six main sections (mood, awake, asleep, workout, week, whoop)
- **Syncing**: Stored in Dropbox, loaded to database via `questionDump.py`

**Loading configuration to database**:
```bash
cd db
python3 questionDump.py "postgres://user:pass@host:5432/dbname" "https://dl.dropboxusercontent.com/s/zeukqsmaw1ppnhe/lifesheet.json"
```

### Question Configuration Structure

Each section contains:
- `description` - Purpose of the survey
- `schedule` - Frequency: `eightTimesADay`, `daily`, `weekly`, `manual`
- `questions` - Array of question objects

Question object fields:
```json
{
    "key": "uniqueIdentifier",
    "displayName": "UI Label",
    "question": "Question text shown to user",
    "type": "range|boolean|text|number|location|header",
    "category": "Physical Health|Mental Health|Productivity|Work|Social|Hobbies",
    "isVisibleInVisualizer": true,
    "isPositive": true,
    "isReverse": false,
    "graphType": "line",
    "buttons": {}  // For range types
}
```

## Development Commands

### Telegram Bot (`telegram_bot/`)
```bash
npm run worker-dev      # Development with auto-compile
npm run scheduler-dev   # Development scheduler
npm run worker          # Production worker
npm run scheduler       # Production scheduler
npm run prettier        # Format code
npm run typescript      # Compile TypeScript
```

### Visualization Backend (`viz/backend/`)
```bash
cargo build --release   # Build Rust backend
cargo run              # Run development server
```

### Visualization Frontend (`viz/frontend/`)
```bash
npm start              # Development server
npm run build          # Production build
npm test              # Run tests
```

### Full Stack Development
```bash
docker-compose up      # All services with database
docker-compose up db   # Database only
```

### Database Management
```bash
# Connect to database
psql "postgres://user:pass@host:5432/dbname"

# Common queries
SELECT DISTINCT key FROM raw_data WHERE source = 'telegram';
SELECT COUNT(*), key FROM raw_data GROUP BY key ORDER BY COUNT DESC;
SELECT * FROM raw_data WHERE key = 'mood' ORDER BY timestamp DESC LIMIT 10;
```

## WHOOP Collector

### Overview
The WHOOP collector imports health and fitness data from WHOOP v2 API into FxLifeSheet.

**Location**: `collector/whoop/`

### WHOOP v2 API Migration
- **Deadline**: October 1, 2025 (v1 API discontinued)
- **Authentication**: OAuth 2.0 with refresh tokens
- **Required Scopes**: `offline read:recovery read:cycles read:sleep read:workout read:profile`

### Setup

1. **Install dependencies**:
```bash
cd collector/whoop
# Dependencies managed via uv (pyproject.toml)
```

2. **Configure environment** (`.env` file):
```bash
WHOOP_CLIENT_ID=your_client_id
WHOOP_CLIENT_SECRET=your_client_secret
WHOOP_REDIRECT_URI=http://localhost:8080/callback
WHOOP_ACCESS_TOKEN=your_access_token
WHOOP_REFRESH_TOKEN=your_refresh_token
DATABASE_URL=postgres://user:pass@host:5432/dbname
```

3. **OAuth Authentication**:
```bash
# Simplified token exchange (recommended)
uv run exchange_token.py
# Opens browser -> authorize -> paste callback URL -> tokens auto-saved

# Alternative: Full OAuth flow with PKCE
uv run oauth_helper.py
```

### WHOOP API v2 Endpoints

**Base URL**: `https://api.prod.whoop.com`

Correct endpoints (as of Dec 2025):
- `/developer/v2/cycle` - Daily physiological cycles
- `/developer/v2/cycle/{cycle_id}/recovery` - Recovery data for specific cycle
- `/developer/v2/activity/sleep` - Sleep activities
- `/developer/v2/activity/workout` - Workout activities
- `/developer/v1/user/profile/basic` - User profile (for connection testing)

### Data Imported (14 Metrics)

**Recovery & Health** (Physical Health category):
1. `whoopRecoveryScore` - Overall recovery percentage (0-100)
2. `whoopRHR` - Resting heart rate (bpm)
3. `whoopHRV` - Heart rate variability (ms)
4. `whoopSkinTemp` - Skin temperature (°C)
5. `whoopSpO2` - Blood oxygen saturation (%) *New in v2*
6. `whoopSleepPerformance` - Sleep performance percentage
7. `whoopSleepEfficiency` - Sleep efficiency percentage
8. `whoopRespiratoryRate` - Breathing rate during sleep
9. `whoopSleepConsistency` - Sleep consistency percentage

**Workout** (Workout category):
10. `whoopStrain` - Workout strain score
11. `whoopAvgHeartRate` - Average HR during workout
12. `whoopMaxHeartRate` - Maximum HR during workout
13. `whoopKilojoules` - Energy expenditure
14. `whoopDistance` - Distance covered in meters

### Running WHOOP Import

```bash
cd collector/whoop

# Import from last data point to today (recommended for regular runs)
uv run whoop_import.py --from-last

# Import specific date
uv run whoop_import.py --date 2024-01-15

# Import last N days
uv run whoop_import.py --days 7
```

### Import Process Flow

1. **Connect to database** - Check for existing data
2. **Ensure valid token** - Refresh if needed (auto-updates .env)
3. **Fetch cycles** - Get daily physiological cycles
4. **Fetch recovery** - For each cycle, get recovery data from `/cycle/{id}/recovery`
5. **Fetch sleep** - Get sleep activities
6. **Fetch workout** - Get workout activities
7. **Duplicate detection** - Skip records that already exist
8. **Insert data** - Store with proper timestamps and indexing

### Key Implementation Details

**Token Management**:
- Access tokens expire after 1 hour
- Refresh tokens used to get new access tokens
- Auto-refresh with retry logic (3 attempts with exponential backoff)
- Tokens auto-saved to `.env` file
- Email alerts when manual re-auth needed (if SMTP configured)

**Data Processing**:
- Recovery data fetched per-cycle (not in bulk)
- Duplicate detection by `key + question + matcheddate + source + value`
- All data marked with `source='whoop'` for filtering
- Import tracking via unique `importid` and `importedat` timestamp

**Error Handling**:
- 404 on recovery endpoint = no recovery data for that cycle yet (normal)
- 401 = token expired, auto-refresh attempted
- Production mode continues on errors (for automated runs)

### Troubleshooting

**401 Unauthorized errors**:
- Check if tokens need refresh: `WHOOP_REFRESH_TOKEN` in `.env`
- Verify scopes include: `read:recovery read:cycles read:sleep read:workout`
- Re-authenticate if refresh token expired: `uv run exchange_token.py`

**404 Not Found errors**:
- Verify endpoint paths (v2 uses different paths than v1)
- Recovery data: Must use `/developer/v2/cycle/{id}/recovery`
- Sleep data: `/developer/v2/activity/sleep` (not `/developer/v1/sleep`)
- Workout data: `/developer/v2/activity/workout` (not `/developer/v1/workout`)

**No data being imported**:
- Check database connection (remote vs local)
- Verify `DATABASE_URL` in `.env`
- Check if cycles have `score_state == 'SCORED'`
- Some cycles may not have recovery data yet (normal)

**SpO2 data missing**:
- SpO2 only available from certain firmware/device versions
- Check if data exists: `SELECT COUNT(*) FROM raw_data WHERE key = 'whoopSpO2'`
- May only have data from recent dates (e.g., Oct 2025 onwards)

### Adding New WHOOP Metrics

If WHOOP adds new metrics to their v2 API:

1. **Update `whoop_import.py`**:
   - Add metric extraction in appropriate `process_*` method
   - Follow existing pattern with `insert_raw_data()`

2. **Update `lifesheet.json`**:
   ```json
   {
       "key": "whoopNewMetric",
       "displayName": "New Metric Name",
       "question": "Whoop New Metric Description",
       "category": "Physical Health",
       "type": "number",
       "isVisibleInVisualizer": true,
       "isPositive": true,
       "graphType": "line"
   }
   ```

3. **Upload to Dropbox**: `~/Dropbox/Public/lifesheet.json`

4. **Load to database**:
   ```bash
   cd db
   python3 questionDump.py "$DATABASE_URL" "https://dl.dropboxusercontent.com/s/zeukqsmaw1ppnhe/lifesheet.json"
   ```

5. **Verify**:
   ```bash
   psql "$DATABASE_URL" -c "SELECT * FROM questions WHERE key = 'whoopNewMetric';"
   ```

## Environment Variables

### Telegram Bot
```bash
TELEGRAM_BOT_TOKEN    # Bot authentication
TELEGRAM_USER_ID      # User identification
TELEGRAM_CHAT_ID      # Chat identification
DATABASE_URL          # PostgreSQL connection
```

### WHOOP Collector
```bash
WHOOP_CLIENT_ID       # OAuth app client ID
WHOOP_CLIENT_SECRET   # OAuth app client secret
WHOOP_ACCESS_TOKEN    # Current access token
WHOOP_REFRESH_TOKEN   # Refresh token (for auto-renewal)
WHOOP_REDIRECT_URI    # OAuth callback URL (default: http://localhost:8080/callback)
DATABASE_URL          # PostgreSQL connection

# Optional: Email alerts
ALERT_EMAIL           # Email for token expiry alerts
SMTP_SERVER           # SMTP server (e.g., smtp.gmail.com)
SMTP_USER             # SMTP username
SMTP_PASS             # SMTP password
```

## Database Connections

### Local Development
```bash
DATABASE_URL=postgresql://user@localhost/dbname
```

### Remote Production
```bash
DATABASE_URL=postgres://user:pass@206.189.140.208:5432/dbname
```

**Important**: The WHOOP collector uses the `DATABASE_URL` from its `.env` file, which may point to a remote database. Always verify which database you're querying:
```bash
# Check WHOOP collector database
grep DATABASE_URL collector/whoop/.env

# Query remote database
psql "postgres://user:pass@host:5432/dbname" -c "SELECT COUNT(*) FROM raw_data WHERE source='whoop';"
```

## Deployment Architecture

### Production Setup
- **Nginx** - SSL termination, reverse proxy (`conf/nginx.conf`)
- **Docker Compose** - Service orchestration with profiles
- **Cloudflare** - SSL certificates
- **PostgreSQL** - Persistent data with volume mapping

### Service Dependencies
- `worker` and `scheduler` depend on `db`
- `viz` depends on `db`
- `nginx` depends on `viz` (when using with_nginx profile)

## Testing

### Telegram Bot Testing
- Development mode: `npm run worker-dev`
- Chrome DevTools debugging: `chrome://inspect`
- Test commands directly in Telegram chat

### Database Testing
```bash
# Access PostgreSQL
docker-compose exec db psql -U soumyadeepmukherjee

# Query raw_data
SELECT * FROM raw_data WHERE key = 'mood' ORDER BY timestamp DESC LIMIT 10;

# Check last_run for scheduling
SELECT * FROM last_run ORDER BY last_run_time DESC;
```

### Full Stack Testing
```bash
docker-compose up                    # Complete system
# Frontend: http://localhost:3000 (development)
# Backend API: http://localhost:8080
# Database: localhost:5432
```

### WHOOP Import Testing
```bash
cd collector/whoop

# Test with single day
uv run whoop_import.py --days 1

# Verify data imported
psql "$DATABASE_URL" -c "SELECT COUNT(*), key FROM raw_data WHERE source='whoop' GROUP BY key ORDER BY key;"

# Check recent SpO2 data
psql "$DATABASE_URL" -c "SELECT matcheddate, value FROM raw_data WHERE key='whoopSpO2' ORDER BY matcheddate DESC LIMIT 10;"
```

## Common Workflows

### Adding a New Metric to Telegram Bot

1. Edit `telegram_bot/lifesheet.json` (in Dropbox)
2. Add question definition with proper structure
3. Restart bot worker: `npm run worker`
4. Load to database: `cd db && python3 questionDump.py "$DATABASE_URL" "$DROPBOX_JSON_URL"`
5. Test in Telegram chat
6. Update frontend if visualization needed

### Viewing Collected Data

```bash
# Recent mood data
psql "$DATABASE_URL" -c "SELECT timestamp, value FROM raw_data WHERE key='mood' ORDER BY timestamp DESC LIMIT 20;"

# Daily summaries
psql "$DATABASE_URL" -c "SELECT matcheddate, COUNT(*) FROM raw_data GROUP BY matcheddate ORDER BY matcheddate DESC LIMIT 30;"

# WHOOP recovery scores
psql "$DATABASE_URL" -c "SELECT matcheddate, value FROM raw_data WHERE key='whoopRecoveryScore' ORDER BY matcheddate DESC LIMIT 10;"
```

### Backup and Recovery

```bash
# Backup database
pg_dump "$DATABASE_URL" > backup_$(date +%Y%m%d).sql

# Restore database
psql "$DATABASE_URL" < backup_20241221.sql

# Backup lifesheet.json (already in Dropbox)
cp ~/Dropbox/Public/lifesheet.json ~/backups/lifesheet_$(date +%Y%m%d).json
```

## File Structure

```
FxLifeSheet/
├── CLAUDE.md                    # This file
├── telegram_bot/                # Node.js/TypeScript bot
│   ├── src/
│   │   ├── worker.ts           # Main bot logic
│   │   ├── scheduler.js        # Cron-based reminders
│   │   └── classes/            # Database, Telegram, config utilities
│   ├── lifesheet.json          # Question configuration (symlink to Dropbox)
│   └── package.json
├── viz/
│   ├── backend/                # Rust API server
│   │   ├── src/
│   │   │   ├── web/           # API endpoints
│   │   │   └── model/         # Data models
│   │   └── Cargo.toml
│   └── frontend/              # React dashboard
│       ├── src/
│       │   └── components/    # UI components
│       └── package.json
├── collector/
│   └── whoop/                 # WHOOP data importer
│       ├── whoop_import.py    # Main import script
│       ├── oauth_helper.py    # OAuth with PKCE
│       ├── exchange_token.py  # Simplified OAuth
│       ├── .env               # Configuration
│       └── README.md
├── db/
│   ├── a_create_tables.sql    # Database schema
│   └── questionDump.py        # Load lifesheet.json to DB
├── conf/
│   └── nginx.conf             # Nginx configuration
└── docker-compose.yml         # Service orchestration
```

## Best Practices

### When Modifying Questions
1. Always update `lifesheet.json` in Dropbox first
2. Run `questionDump.py` to sync to database
3. Restart relevant services (bot, frontend)
4. Test the full flow before deploying

### When Adding External Data Sources
1. Follow WHOOP collector pattern
2. Store in `raw_data` with unique `source` identifier
3. Add questions to `lifesheet.json`
4. Implement duplicate detection
5. Add proper error handling and logging
6. Support `--from-last` for incremental imports

### Database Queries
- Always use parameterized queries to prevent SQL injection
- Index on `timestamp`, `key`, `matcheddate` for performance
- Use `source` field to filter by data origin
- Consider time zones (data stored in UTC, displayed in user's timezone)

### OAuth and API Keys
- Never commit secrets to git
- Use `.env` files (already in .gitignore)
- Implement token refresh logic for long-running processes
- Add retry logic with exponential backoff
- Alert on token expiry if running automated imports

## Useful Resources

- Telegram Bot API: https://core.telegram.org/bots/api
- WHOOP API v2 Docs: https://developer.whoop.com/api/
- PostgreSQL Docs: https://www.postgresql.org/docs/
- Docker Compose: https://docs.docker.com/compose/

## Recent Changes (Dec 2025)

- ✅ Migrated WHOOP collector from v1 to v2 API
- ✅ Added `whoopSpO2` (Blood Oxygen) metric
- ✅ Updated OAuth flow to include `read:recovery` scope
- ✅ Fixed API endpoints (v2 paths)
- ✅ Implemented per-cycle recovery data fetching
- ✅ Added automatic token refresh with retry logic
- ✅ Synced lifesheet.json with all 14 WHOOP metrics

## Support

For issues or questions about this codebase:
1. Check this CLAUDE.md file first
2. Review component-specific README files
3. Check logs: `docker-compose logs [service]`
4. Verify environment variables are set correctly
5. Test database connectivity

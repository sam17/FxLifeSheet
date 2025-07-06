# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Overview

FxLifeSheet is a personal life tracking system with a microservices architecture consisting of:
- **Telegram Bot** (Node.js/TypeScript) - Data collection via chat interface
- **Visualization Backend** (Rust) - API server for data retrieval  
- **Visualization Frontend** (React/TypeScript) - Web dashboard
- **PostgreSQL Database** - Central data storage
- **Additional services** - Collector, subjective dashboard, analysis tools

## Core Architecture

### Data Flow
1. User interacts with Telegram bot (`/awake`, `/asleep`, `/week`, `/mood`)
2. Bot reads questions from `telegram_bot/lifesheet.json`
3. Responses stored in PostgreSQL `raw_data` table
4. Scheduler (`src/scheduler.js`) sends reminders based on question schedules
5. Visualization backend (Rust) serves data via REST API
6. Frontend renders charts and dashboards

### Key Database Schema
- `raw_data` - Main data table with `timestamp`, `key`, `question`, `type`, `value`
- `last_run` - Tracks when questions were last answered
- `metadata` - System metadata storage
- `category` - Question categories for organization

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

## Key Configuration Files

### Question Configuration (`telegram_bot/lifesheet.json`)
- Defines all trackable metrics and their types
- Question types: `range`, `boolean`, `text`, `number`, `location`
- Scheduling: `eightTimesADay`, `daily`, `weekly`, `manual`
- Categories: Mental Health, Physical Health, Productivity, Hobbies, Social

### Environment Variables
Required for telegram bot operation:
```bash
TELEGRAM_BOT_TOKEN    # Bot authentication
TELEGRAM_USER_ID      # User identification  
TELEGRAM_CHAT_ID      # Chat identification
DATABASE_URL          # PostgreSQL connection
```

## Component Interactions

### Telegram Bot Architecture
- `src/worker.ts` - Main bot logic, question handling, database interaction
- `src/scheduler.js` - Cron-based reminder system
- `src/classes/` - Database, Telegram, and config utility classes
- Questions configured in JSON, not hardcoded

### Visualization Stack
- **Backend**: Rust with Warp framework, SQLx for PostgreSQL
- **Frontend**: React with TypeScript, Ant Design UI, D3.js charts
- **API**: RESTful endpoints at `/api/data/`, `/api/metadata`, `/api/questions`
- **Build**: Multi-stage Docker build combining Rust backend + React frontend

### Database Design
- Time-series data in `raw_data` table
- Flexible schema accommodating various question types
- Temporal indexing with `yearmonth`, `yearweek`, `matcheddate`
- Import tracking with `importid`, `importedat`

## Development Patterns

### Adding New Questions
1. Modify `telegram_bot/lifesheet.json` with new question definition
2. Restart bot worker to load new configuration
3. Test question flow via Telegram
4. Update visualization frontend if new data visualization needed

### Database Changes
1. Update `db/a_create_tables.sql` for schema changes
2. Run migrations on development database
3. Test with docker-compose recreation
4. Update model classes in both TypeScript and Rust

### API Development
1. Backend changes in `viz/backend/src/web/` and `viz/backend/src/model/`
2. Frontend integration in `viz/frontend/src/components/`
3. CORS configuration handled in Rust backend
4. Full stack testing via Docker Compose

## Deployment Architecture

### Production Setup
- **Nginx** - SSL termination, reverse proxy configured in `conf/nginx.conf`
- **Docker Compose** - Service orchestration with profiles
- **Cloudflare** - SSL certificates referenced in compose file
- **PostgreSQL** - Persistent data with volume mapping

### Service Dependencies
- `worker` and `scheduler` depend on `db`
- `viz` depends on `db` 
- `nginx` depends on `viz` (when using with_nginx profile)

## Testing and Debugging

### Telegram Bot Testing
- Use `npm run worker-dev` for development with auto-compilation
- Chrome DevTools debugging available via `chrome://inspect` after `npm run dev`
- Test commands directly in Telegram chat

### Database Testing
- Access PostgreSQL via `docker-compose exec db psql -U soumyadeepmukherjee`
- Query `raw_data` table to verify data collection
- Check `last_run` table for scheduling behavior

### Full Stack Testing
- Use `docker-compose up` to test complete system
- Frontend accessible at `http://localhost:3000` (development)
- Backend API at `http://localhost:8080` (via viz service)
- Database at `localhost:5432`
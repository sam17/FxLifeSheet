FROM python:3.11-slim

WORKDIR /app

# Install system dependencies
RUN apt-get update && apt-get install -y \
    libpq-dev \
    gcc \
    cron \
    && rm -rf /var/lib/apt/lists/*

# Copy application code
COPY whoop_import.py .
COPY oauth_helper.py .
COPY requirements.txt .

# Install Python dependencies using pip (better ARM compatibility)
RUN pip install --no-cache-dir -r requirements.txt

# cron runs with a minimal PATH and no container env vars, so the crontab
# is generated at container startup with PATH + WHOOP_*/DATABASE_URL inlined.
RUN touch /var/log/whoop_cron.log

CMD set -e && \
    echo "Starting whoop collector..." && \
    { echo "PATH=/usr/local/bin:/usr/bin:/bin"; \
      env | grep -E '^WHOOP_|^DATABASE_URL='; \
      echo "0 */6 * * * cd /app && /usr/local/bin/python3 whoop_import.py >> /var/log/whoop_cron.log 2>&1"; \
    } | crontab - && \
    echo "Running initial import..." && \
    cd /app && (/usr/local/bin/python3 whoop_import.py >> /var/log/whoop_cron.log 2>&1 || echo "Initial import failed (expected if no credentials), continuing with scheduler...") && \
    echo "Starting cron scheduler..." && \
    cron && exec tail -f /var/log/whoop_cron.log
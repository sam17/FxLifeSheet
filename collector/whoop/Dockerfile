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

# Set up cron job
RUN echo "0 */6 * * * cd /app && python3 whoop_import.py >> /var/log/whoop_cron.log 2>&1" > /etc/cron.d/whoop-cron
RUN chmod 0644 /etc/cron.d/whoop-cron
RUN crontab /etc/cron.d/whoop-cron
RUN touch /var/log/whoop_cron.log

# Run cron in foreground and tail logs
CMD echo "Starting whoop collector..." && \
    echo "Running initial import..." && \
    cd /app && (python3 whoop_import.py >> /var/log/whoop_cron.log 2>&1 || echo "Initial import failed (expected if no credentials), continuing with scheduler...") && \
    echo "Starting cron scheduler..." && \
    cron && exec tail -f /var/log/whoop_cron.log
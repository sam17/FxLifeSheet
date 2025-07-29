FROM python:3.11-slim

WORKDIR /app

# Install system dependencies
RUN apt-get update && apt-get install -y \
    libpq-dev \
    gcc \
    cron \
    && rm -rf /var/lib/apt/lists/*

# Install uv
COPY --from=ghcr.io/astral-sh/uv:latest /uv /usr/local/bin/uv

# Copy uv files first for better layer caching
COPY pyproject.toml uv.lock ./

# Install dependencies with uv
RUN uv sync --frozen

# Copy application code
COPY whoop_import.py .
COPY oauth_helper.py .

# Set up cron job
RUN echo "0 */6 * * * cd /app && /usr/local/bin/uv run whoop_import.py >> /var/log/whoop_cron.log 2>&1" > /etc/cron.d/whoop-cron
RUN chmod 0644 /etc/cron.d/whoop-cron
RUN crontab /etc/cron.d/whoop-cron
RUN touch /var/log/whoop_cron.log

# Run cron in foreground and tail logs
CMD cron && tail -f /var/log/whoop_cron.log
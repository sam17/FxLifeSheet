FROM python:3.11-slim

WORKDIR /app

# Install system dependencies
RUN apt-get update && apt-get install -y \
    libpq-dev \
    gcc \
    cron \
    curl \
    && rm -rf /var/lib/apt/lists/*

# Install uv using the official install script (supports multiple architectures)
RUN curl -LsSf https://astral.sh/uv/install.sh | sh && \
    mv /root/.local/bin/uv /usr/local/bin/uv && \
    mv /root/.local/bin/uvx /usr/local/bin/uvx

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
CMD echo "Starting whoop collector..." && \
    echo "Running initial import..." && \
    cd /app && (/usr/local/bin/uv run whoop_import.py >> /var/log/whoop_cron.log 2>&1 || echo "Initial import failed (expected if no credentials), continuing with scheduler...") && \
    echo "Starting cron scheduler..." && \
    cron && exec tail -f /var/log/whoop_cron.log
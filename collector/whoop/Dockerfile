# Multi-stage build: keep gcc + libpq-dev only in the builder so the runtime
# image stays small. psycopg2-binary has no prebuilt wheel for arm/v6, so pip
# still has to compile it from source -- but only inside the builder layer.
FROM python:3.11-slim AS builder

# libc6-dev is needed for stdlib.h etc. -- with --no-install-recommends gcc
# doesn't pull it in transitively the way it used to.
RUN apt-get update && apt-get install -y --no-install-recommends \
        libpq-dev \
        gcc \
        libc6-dev \
    && rm -rf /var/lib/apt/lists/*

COPY requirements.txt /tmp/requirements.txt
RUN pip install --no-cache-dir --prefix=/install --no-warn-script-location \
        -r /tmp/requirements.txt

# ---- runtime ----
FROM python:3.11-slim

RUN apt-get update && apt-get install -y --no-install-recommends \
        libpq5 \
        cron \
    && rm -rf /var/lib/apt/lists/*

COPY --from=builder /install /usr/local

WORKDIR /app
COPY whoop_import.py oauth_helper.py ./

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

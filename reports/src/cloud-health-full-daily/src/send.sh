#!/bin/bash
# ══════════════════════════════════════════════════════════════════════
# Send pre-built HTML report via Maddy SMTP
# Usage: send.sh [html_file]
#   html_file defaults to ../dist/cloud_health_daily.html
# ══════════════════════════════════════════════════════════════════════
set -eu

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
# Reports engine writes to the SHARED reports/dist/, not per-crate dist/.
# SCRIPT_DIR = reports/src/cloud-health-full-daily/src/  →  reports/ is 3 up.
HTML_FILE="${1:-$SCRIPT_DIR/../../../dist/cloud_health_daily.html}"
DATE=$(date '+%Y-%m-%d')

if [ ! -f "$HTML_FILE" ]; then
  echo "ERROR: HTML file not found: $HTML_FILE"
  echo "Run report_daily.sh first to generate it."
  exit 1
fi

# Build MIME message (headers + HTML body)
MIME_FILE=$(mktemp)
trap 'rm -f "$MIME_FILE"' EXIT

cat > "$MIME_FILE" <<EOHEADERS
From: no-reply@diegonmarcos.com
To: me@diegonmarcos.com
Subject: C3 Daily Ops Report - $DATE
MIME-Version: 1.0
Content-Type: text/html; charset=UTF-8

EOHEADERS

cat "$HTML_FILE" >> "$MIME_FILE"

# Send via Maddy SMTP :587
curl -s --url "smtp://10.0.0.3:587" \
  --ssl-reqd -k \
  --user "no-reply@diegonmarcos.com:${NOREPLY_PASSWORD}" \
  --mail-from "no-reply@diegonmarcos.com" \
  --mail-rcpt "me@diegonmarcos.com" \
  -T "$MIME_FILE"
SEND_RC=$?

if [ $SEND_RC -eq 0 ]; then
  echo "C3 Daily Ops Report sent for $DATE via Maddy SMTP :587"
else
  echo "FAILED to send report (curl exit $SEND_RC)"
  exit 1
fi

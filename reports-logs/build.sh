#!/bin/sh
# Thin shim → engine lives at reports-logs/src/build.sh.
# Usage:  reports-logs/build.sh [all|docker|systemd|network|dns|tls|mail|cloudflare|list|clean]
#         reports-logs/build.sh <module> [vm=<alias>] [service=<name>]
exec sh "$(cd "$(dirname "$0")" && pwd)/src/build.sh" "$@"

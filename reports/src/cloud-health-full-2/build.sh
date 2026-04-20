#!/bin/sh
set -eu
ROOT="$(cd "$(dirname "$0")" && pwd)"
BINARY="cloud-health-full-2"
SUPPORT_DIRS="cache"
. "$ROOT/../_crate_engine.sh"
engine_dispatch "$@"

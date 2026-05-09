#!/usr/bin/env bash
# Smoke test against api-dev.favcrm.io.
#
# Usage:
#   FAVCRM_API_KEY=fav_mcp_... ./smoke.sh
#
# Or pass key as $1:
#   ./smoke.sh fav_mcp_...
#
# Exits non-zero on any failure.
set -euo pipefail

URL="${FAVCRM_MCP_URL:-https://api-dev.favcrm.io/mcp}"
KEY="${1:-${FAVCRM_API_KEY:-}}"

if [[ -z "$KEY" ]]; then
  echo "✗ FAVCRM_API_KEY not set" >&2
  exit 2
fi

cd "$(dirname "$0")"

echo "→ cargo check"
cargo check --release

echo "→ cargo build --release"
cargo build --release

BIN="./target/release/favcrm"
COMMON=(--url "$URL" --api-key "$KEY")

run() {
  local label="$1"; shift
  echo
  echo "→ $label"
  echo "  $ $BIN ${COMMON[*]} $*"
  "$BIN" "${COMMON[@]}" "$@"
}

run "whoami"   whoami
run "orgs"     orgs list
run "dashboard" dashboard
run "members"  members search --limit 3
run "bookings" bookings list --limit 3
run "invoices" invoices list --limit 3
run "raw tool" tool list_tags '{}'
run "json out" --json bookings stats

echo
echo "✓ smoke pass"

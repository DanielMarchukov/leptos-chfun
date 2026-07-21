#!/usr/bin/env bash
# Build the SSR app, boot it on a local port, and assert the page + its
# hydration assets actually serve. Exits non-zero on any failure.
#
# Port 3000 may already be taken on this box (e.g. a local Dokploy
# instance), so the port is configurable:
#   PORT=3200 ./scripts/smoke-test.sh
set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
REPO_ROOT="$(cd "${SCRIPT_DIR}/.." && pwd)"
cd "${REPO_ROOT}"

PORT="${PORT:-3100}"
ADDR="127.0.0.1:${PORT}"
BASE_URL="http://${ADDR}"
BIN="${REPO_ROOT}/target/debug/leptos-chfun"
LOG_FILE="$(mktemp -t leptos-chfun-smoke.XXXXXX.log)"

SERVER_PID=""
FAILURES=0
RESULTS=()

log() { printf '%s\n' "$*"; }

cleanup() {
  local exit_code=$?
  if [[ -n "${SERVER_PID}" ]] && kill -0 "${SERVER_PID}" 2>/dev/null; then
    kill "${SERVER_PID}" 2>/dev/null || true
    wait "${SERVER_PID}" 2>/dev/null || true
  fi
  if (( exit_code != 0 || FAILURES > 0 )); then
    log ""
    log "----- server log (${LOG_FILE}) -----"
    cat "${LOG_FILE}" 2>/dev/null || true
    log "-------------------------------------"
  fi
  rm -f "${LOG_FILE}"
}
trap cleanup EXIT INT TERM

record() {
  local name="$1" ok="$2" detail="${3:-}"
  if [[ "${ok}" == "0" ]]; then
    RESULTS+=("PASS  ${name}")
  else
    RESULTS+=("FAIL  ${name}${detail:+ - ${detail}}")
    FAILURES=$((FAILURES + 1))
  fi
}

assert_status() {
  local name="$1" path="$2" expected="$3"
  local actual
  actual="$(curl -s -o /dev/null -w '%{http_code}' "${BASE_URL}${path}" || echo "000")"
  if [[ "${actual}" == "${expected}" ]]; then
    record "${name}" 0
  else
    record "${name}" 1 "expected HTTP ${expected}, got ${actual}"
  fi
}

assert_body_contains() {
  local name="$1" path="$2" needle="$3"
  local body
  body="$(curl -s "${BASE_URL}${path}" || true)"
  if grep -qF "${needle}" <<<"${body}"; then
    record "${name}" 0
  else
    record "${name}" 1 "response body did not contain '${needle}'"
  fi
}

log "==> Building (cargo leptos build)"
cargo leptos build

if [[ ! -x "${BIN}" ]]; then
  log "FAIL: built binary not found at ${BIN}"
  exit 1
fi

log "==> Starting server on ${BASE_URL} (log: ${LOG_FILE})"
LEPTOS_OUTPUT_NAME=leptos-chfun \
LEPTOS_SITE_ROOT=target/site \
LEPTOS_SITE_PKG_DIR=pkg \
LEPTOS_SITE_ADDR="${ADDR}" \
"${BIN}" >"${LOG_FILE}" 2>&1 &
SERVER_PID=$!

log "==> Waiting for readiness on ${BASE_URL}/healthz"
curl --silent --show-error --fail \
  --retry-connrefused --retry 30 --retry-delay 1 \
  --output /dev/null \
  "${BASE_URL}/healthz"

# Prefer the pkg asset paths as actually referenced by the rendered page,
# falling back to the known output-name paths if parsing finds nothing.
HTML="$(curl -s "${BASE_URL}/")"
JS_PATH="$(grep -oE '/pkg/[A-Za-z0-9_.-]+\.js' <<<"${HTML}" | head -n1 || true)"
WASM_PATH="$(grep -oE '/pkg/[A-Za-z0-9_.-]+\.wasm' <<<"${HTML}" | head -n1 || true)"
JS_PATH="${JS_PATH:-/pkg/leptos-chfun.js}"
WASM_PATH="${WASM_PATH:-/pkg/leptos-chfun.wasm}"

log "==> Running assertions"
assert_status        "GET / returns 200"                     "/"        "200"
assert_body_contains  "GET / body contains 'Chean Hui Toh'"   "/"        "Chean Hui Toh"
assert_body_contains  "GET /healthz body is 'ok'"             "/healthz" "ok"
assert_status         "GET /healthz returns 200"              "/healthz" "200"
assert_status         "GET /version returns 200"              "/version" "200"
assert_status         "GET ${JS_PATH} returns 200"            "${JS_PATH}"   "200"
assert_status         "GET ${WASM_PATH} returns 200"          "${WASM_PATH}" "200"

log ""
log "===== Smoke test summary ====="
for r in "${RESULTS[@]}"; do
  log "  ${r}"
done
log "==============================="

if (( FAILURES > 0 )); then
  log "FAIL: ${FAILURES} assertion(s) failed"
  exit 1
fi

log "PASS: all assertions passed"
exit 0

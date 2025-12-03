#!/bin/sh

WASM_MODULE="target/wasm32-wasip1/release-wasi/js-eval-boa.wasm"

echo "--- Running with expression (-e) ---"
wazero run "${WASM_MODULE}" -e "10 + 20"

echo "\n--- Running with stdin ---"
echo "'stdin_result';" | wazero run "${WASM_MODULE}"

echo "\n--- Running long but finite loop (1s) with 0.5s timeout ---"
cat ./dos-safe-test.js | wazero run -timeout=500ms "${WASM_MODULE}"
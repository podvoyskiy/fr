#!/bin/bash

MODE=$1

if [[ "$MODE" != "dev" && "$MODE" != "prod" ]]; then
    echo "Error: first arg must be 'dev' or 'prod'"
    exit 1
fi

case "$MODE" in
    prod)
        CMD="./target/release/fr"
        [[ ! -f "$CMD" ]] && { echo "binary not found. run: cargo build --release"; exit 1; }
        ;;
    *)
        CMD="cargo run --quiet --"
        ;;
esac

shift

if [ $# -gt 0 ]; then
    $CMD "$@"
else
    eval "$($CMD)"
fi
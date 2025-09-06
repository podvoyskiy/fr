#!/bin/bash

# FR Development Helper Function

# Add this to your ~/.bashrc:
# if [ -f /path/to/project/fr-dev.sh ]; then
#     . /path/to/project/fr-dev.sh
# fi

# Usage:
# frlocal dev          # development mode (cargo run)
# frlocal prod         # production mode (requires built binary)
# frlocal dev --help   # dev mode with arguments  
# frlocal prod --help  # prod mode with arguments

frlocal() {
    if [[ $# -eq 0 ]]; then
        echo "Error: first argument must be 'dev' or 'prod'"
        return 1
    fi
    
    local MODE="$1"
    shift
    
    if [[ "$MODE" != "dev" && "$MODE" != "prod" ]]; then
        echo "Error: first argument must be 'dev' or 'prod'"
        return 1
    fi
    
    case "$MODE" in
        prod)
            if [[ ! -f "./target/release/fr" ]]; then
                echo "Error: production binary not found. Run: cargo build --release"
                return 1
            fi
            CMD="./target/release/fr"
            ;;
        dev)
            CMD="cargo run --quiet --"
            ;;
    esac
    
    if [ $# -gt 0 ]; then
        $CMD "$@"
    else
        local selected_cmd
        selected_cmd=$($CMD)
        
        if [[ -n "$selected_cmd" ]]; then
            eval "$selected_cmd"
        fi
    fi
}
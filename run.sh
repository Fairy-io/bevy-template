#!/bin/bash

SCRIPT_NAME=$1

case "$SCRIPT_NAME" in
    app)
        ./scripts/app.sh
        ;;

    lib)
        cargo watch -w lib -- ./scripts/lib.sh
        ;;
    
esac

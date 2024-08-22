#!/bin/bash

cargo build -p lib
cargo watch -w src -x "run --features=\"dev\""

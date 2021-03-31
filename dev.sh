#!/usr/bin/env bash

# Enable strict mode
# http://redsymbol.net/articles/unofficial-bash-strict-mode/
set -euo pipefail
IFS=$'\n\t'

# Launch backend
docker-compose up -d

# Run migrations
diesel migration run

# Start backend
RUST_BACKTRACE=1 cargo watch --exec run --ignore "images/*"
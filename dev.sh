#!/usr/bin/env bash

# Enable strict mode
# http://redsymbol.net/articles/unofficial-bash-strict-mode/
set -euo pipefail
IFS=$'\n\t'

# Launch backend
docker-compose up -d
docker-compose stop backend
cargo watch --exec run --ignore "images/*"
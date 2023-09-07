#!/bin/bash

set -e

source $(git rev-parse --show-toplevel)/scripts/retry_command.sh

# cleanup
echo "[*] removing old docker container"
docker ps -a | tail -n 1 | cut -d' ' -f 1 | xargs docker stop

echo "[*] starting new docker container"
retry_command "docker run --rm -it -d -p 8000:8000 --name stellar stellar/quickstart:soroban-dev@sha256:ed57f7a7683e3568ae401f5c6e93341a9f77d8ad41191bf752944d7898981e0c --standalone --enable-soroban-rpc" 10 1

echo "[*] SETUP SUCCESSFUL"

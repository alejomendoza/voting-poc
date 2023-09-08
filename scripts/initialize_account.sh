#!/bin/bash

set -e

source $(git rev-parse --show-toplevel)/scripts/retry_command.sh

CURRENT_DIR=$(git rev-parse --show-toplevel)/scripts

PUBLIC_KEY=$(cat $CURRENT_DIR/.config/public_key)
FRIENDBOT_URL=$(cat $CURRENT_DIR/.config/friendbot_url)

# initialize account
echo "[*] initializing account"
retry_command "curl $FRIENDBOT_URL?addr=$PUBLIC_KEY" 20 3 "502 Bad Gateway"

#!/bin/bash

set -e

CURRENT_DIR=$(git rev-parse --show-toplevel)/scripts

export NETWORK_PASSPHRASE=$(cat $CURRENT_DIR/.config/network_passphrase)
export RPC_URL=$(cat $CURRENT_DIR/.config/rpc_url)
export SECRET_KEY=$(cat $CURRENT_DIR/.config/secret_key)

function invoke_function {
  CONTRACT_ID="$1"
  FN_NAME="$2"
  soroban contract invoke --id $CONTRACT_ID --source $SECRET_KEY --rpc-url $RPC_URL --network-passphrase "$NETWORK_PASSPHRASE" -- $FN_NAME ${@:3}
}

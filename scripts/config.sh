#!/bin/bash

set -e

CURRENT_DIR=$(git rev-parse --show-toplevel)/scripts

export PUBLIC_KEY="GCSKTE64HQRTJR2F3I5HZYJSCEJMCDNMJTD77HCA3IO3NQBCIURU4SEE"
export SECRET_KEY="SC4GNDOETTVW7FCOKWZ4M7NND3M4XGALQ7OXXL43EP237QEUSHFXQVCG"

if [ "$1" == "localhost" ]; then
  RPC_URL="http://localhost:8000/soroban/rpc"
  NETWORK_PASSPHRASE="Standalone Network ; February 2017"
  FRIENDBOT_URL="http://localhost:8000/friendbot"
elif [ "$1" == "futurenet" ]; then
  RPC_URL="https://rpc-futurenet.stellar.org:443"
  NETWORK_PASSPHRASE="Test SDF Future Network ; October 2022"
  FRIENDBOT_URL="https://friendbot-futurenet.stellar.org"
else
  echo "Usage: $0 <localhost|futurenet>"
  exit 1
fi

rm -rf $CURRENT_DIR/.config || true
mkdir $CURRENT_DIR/.config

echo $PUBLIC_KEY > $CURRENT_DIR/.config/public_key
echo $SECRET_KEY > $CURRENT_DIR/.config/secret_key
echo $RPC_URL > $CURRENT_DIR/.config/rpc_url
echo $NETWORK_PASSPHRASE > $CURRENT_DIR/.config/network_passphrase
echo $FRIENDBOT_URL > $CURRENT_DIR/.config/friendbot_url

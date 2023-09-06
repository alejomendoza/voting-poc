#!/bin/bash

set -e

CONFIG_DIR="$(git rev-parse --show-toplevel)/e2e_tests/config.sh"
source $CONFIG_DIR

# generate key pair https://laboratory.stellar.org/#account-creator?network=test
# and insert generated keys below:
PUBLIC_KEY="GCSKTE64HQRTJR2F3I5HZYJSCEJMCDNMJTD77HCA3IO3NQBCIURU4SEE"
SECRET_KEY="SC4GNDOETTVW7FCOKWZ4M7NND3M4XGALQ7OXXL43EP237QEUSHFXQVCG"

# cleanup
# docker ps -a | tail -n 1 | cut -d' ' -f 1 | xargs docker stop
# start local dev server
# docker run --rm -it -d -p 8000:8000 --name stellar stellar/quickstart:soroban-dev@sha256:ed57f7a7683e3568ae401f5c6e93341a9f77d8ad41191bf752944d7898981e0c --standalone --enable-soroban-rpc

# initialize account
# curl "http://localhost:8000/friendbot?addr=GCSKTE64HQRTJR2F3I5HZYJSCEJMCDNMJTD77HCA3IO3NQBCIURU4SEE"

echo "[*] building contracts"
soroban contract build

echo "[*] deploying contracts"
VOTING_SYSTEM_ID=$(soroban contract deploy --wasm target/wasm32-unknown-unknown/release/voting_system.wasm --source $SECRET_KEY --rpc-url $RPC_URL --network-passphrase 'Standalone Network ; February 2017')
EXTERNAL_DATA_PROVIDER_ID=$(soroban contract deploy --wasm target/wasm32-unknown-unknown/release/voting_external_data_provider.wasm --source $SECRET_KEY --rpc-url $RPC_URL --network-passphrase 'Standalone Network ; February 2017')

function invoke_function {
  CONTRACT_ID="$1"
  FN_NAME="$2"
  ARGS="${@:3}"
  soroban contract invoke --id $CONTRACT_ID --source $SECRET_KEY --rpc-url $RPC_URL --network-passphrase 'Standalone Network ; February 2017' -- $FN_NAME $ARGS
}

# prepare neurons
echo "[*] invoking initialize"
invoke_function $VOTING_SYSTEM_ID initialize
echo "[*] invoking add_layer"
invoke_function $VOTING_SYSTEM_ID add_layer
echo "[*] invoking set_layer_aggregator"
invoke_function $VOTING_SYSTEM_ID set_layer_aggregator --layer_id 0 --aggregator Sum
echo "[*] invoking add_neuron"
invoke_function $VOTING_SYSTEM_ID add_neuron --layer_id 0 --neuron Dummy
echo "[*] invoking add_project"
invoke_function $VOTING_SYSTEM_ID add_project --project_id project001
echo "[*] invoking vote"
invoke_function $VOTING_SYSTEM_ID vote --voter_id user001 --project_id project001 --vote Yes
echo "[*] invoking tally"
invoke_function $VOTING_SYSTEM_ID tally

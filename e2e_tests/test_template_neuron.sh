#!/bin/bash

set -e

CONFIG_DIR="$(git rev-parse --show-toplevel)/e2e_tests/config.sh"
source $CONFIG_DIR

echo "[*] building contracts"
soroban contract build

echo "[*] deploying contracts"
TEMPLATE_NEURON_ID=$(soroban contract deploy --wasm target/wasm32-unknown-unknown/release/voting_template_neuron.wasm --source $SECRET_KEY --rpc-url $RPC_URL --network-passphrase 'Standalone Network ; February 2017')

function invoke_function {
  CONTRACT_ID="$1"
  FN_NAME="$2"
  ARGS="${@:3}"
  soroban contract invoke --id $CONTRACT_ID --source $SECRET_KEY --rpc-url $RPC_URL --network-passphrase 'Standalone Network ; February 2017' -- $FN_NAME $ARGS
}

echo "[*] invoking oracle_function"
ORACLE_RESULT=$(invoke_function $TEMPLATE_NEURON_ID oracle_function --_voter_id user001 --_project_id project001)
echo $ORACLE_RESULT

echo "[*] invoking set_weight"
invoke_function $TEMPLATE_NEURON_ID set_weight --new_weight "[2,5]"

echo "[*] invoking weight_function"
invoke_function $TEMPLATE_NEURON_ID weight_function --raw_neuron_vote $ORACLE_RESULT

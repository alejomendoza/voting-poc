#!/bin/bash

set -e

CURRENT_DIR=$(git rev-parse --show-toplevel)/scripts

$CURRENT_DIR/config.sh futurenet
$CURRENT_DIR/initialize_account.sh
$CURRENT_DIR/deploy_contracts.sh
$CURRENT_DIR/test_voting_system.sh

# for further functions invokations, follow this example:
# $ . ./scripts/invoke_function.sh
# $ invoke_function $(cat ./scripts/.contracts/voting_system_id) get_neural_governance

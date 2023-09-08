#!/bin/bash

set -e

function retry_command {
  CMD="$1"
  RETRY_LIMIT="$2"
  INTERVAL="$3"
  INCORRECT_OUTPUT="$4"

  echo "[*] running command: $CMD"

  RES="FAIL"
  COUNT=0
  while [ "$RES" == "FAIL" ]; do
    if [ $COUNT -gt 0 ]; then
      sleep $INTERVAL;
    fi
    echo "[*] retrying command ($COUNT)..."
    RES=$($CMD || echo "FAIL")
    COUNT=$((COUNT+1))
    if [ $COUNT -gt $RETRY_LIMIT ]; then
      echo "[*] maximum retries reached ($COUNT), exiting"
      exit 1
    fi
    if [ "$RES" == "$INCORRECT_OUTPUT" ]; then
      RES="FAIL"
    fi
  done
}

#!/bin/bash

set -e

# e.g. junod
CHAIN=$1

# e.g. 14047318
BLOCK=$2

if [[ -n "$CHAIN" ]]; then
  "$CHAIN" q staking historical-info $BLOCK --output json > "$CHAIN"_block_info_"$BLOCK".json
  cat "$CHAIN"_block_info_"$BLOCK".json | jq . > "$CHAIN"_block_info_formatted_"$BLOCK".json
else
  echo "no chain specified"
  exit 1
fi

#!/bin/bash

set -e

# e.g. junod
BINARY=$1

# e.g juno, juno-1 etc
CHAIN=$2

# e.g. 14047318
BLOCK=$3

# e.g. https://api.foo.com
NODE=$4

# only a sense check really
if [[ -n "$BINARY" ]]; then
  "$BINARY" q staking historical-info $BLOCK --node "$NODE" --output json > "$CHAIN"_block_info_"$BLOCK".json
  cat "$CHAIN"_block_info_"$BLOCK".json | jq . > "$CHAIN"_block_info_formatted_"$BLOCK".json
else
  echo "no binary specified"
  exit 1
fi

#!/bin/bash

set -e

RPC=$1
FOLDER=$2
HEIGHT=$3

curl -X GET "$RPC/validators?height=$HEIGHT&page=1&per_page=200" -H "accept:application/json" | jq . > $FOLDER/$HEIGHT.json

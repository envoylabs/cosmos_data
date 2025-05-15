# Time series data

This data has been collected from archive nodes via the command

    curl -X GET "https://<archive>:<port>/validators?height=<height>&page=1&per_page=200" -H "accept:application/json" | jq .

In many cases, suitable endpoints were not available to accept queries against the historical_info staking query endpoint, so it was necessary to get voting power from the validators query.

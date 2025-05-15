# Time series data

If you have access to a Cosmos network archive you can query its x/staking historical_info endpoint for block data. However, coverage for this is spotty compared to access to the validators query endpoint, which does provide voting power. From this you can reconstruct the timestamp of the block from other endpoints and tally the total VP. There is a script in the folder that will allow you to query an archive.

This data has been collected from archive nodes via the command:

    curl -X GET "https://<archive>:<port>/validators?height=<height>&page=1&per_page=200" -H "accept:application/json" | jq .

In many cases, suitable endpoints were not available to accept queries against the historical_info staking query endpoint, so it was necessary to get voting power from the validators query.

To run the script and have data collected to a folder, first make a folder for the chain,

    mkdir foochain

Then run the script:

    ./write_json.sh https://archive.foochain.com foochain 1

Where the first argument is the archive (plus port), the second argument is the folder you created, and the third is the block height to query.

If you `more ./foochain/1.json` you will find the output JSON.

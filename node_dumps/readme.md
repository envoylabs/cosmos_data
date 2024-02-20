# Node dumps

These dumps were created at a specific block on each chain on 2024-02-20 at between 1300 and 1600 UTC.

The block obviously differs, but is encoded into the file name in each case, as well as being available in the metadata contained within.

In order to get this data for yourself, compile the relevant node binary to access its CLI, then run the `query_valset.sh` script:

    ./query_valset.sh junod 14047318

NB: the first argument should be the chain's daemon/binary name, and the second, the desired block height to query

If the binary is not on the same machine as a running, sync'd version of the chain in question, you will need to either append `--node <address of valid RPC>` to the commands, or run `query_valset_remote.sh`, passing in an appropriate RPC. To find one, try the [Cosmos Chain Registry](https://github.com/cosmos/chain-registry) or more friendly UI of the [Cosmos Directory](https://cosmos.directory/).

    ./query_valset_remote.sh junod juno 14047318 https://rpc.cosmos.directory:443/juno

NB: Again, the first argument should be the binary you have available locally. The second argument is just the chain name, for file naming. The final argument should be a valid RPC for the target chain. Due to the interoperability of public-facing APIs that run on top of the Cosmos SDK, it is likely that one chain binary will be able to query all the others.

## Using the data

To look at current voting power, check the `tokens` field of every validator data structure.

In order to sort by largest delegation use:

    cat data/juno_block_info_formatted_14047318.json | jq '[.valset[]] | sort_by(.tokens | tonumber) | reverse'

To convert the dataset into a series of pairs of moniker and voting power, use:

    cat data/juno_block_info_formatted_14047318.json | jq '[.valset[]] | sort_by(.tokens | tonumber) | reverse | map([(.description.moniker), (.tokens | tonumber)])'

This command was used to assemble the files in the `voting_power` folder.

NB: Not every validator in the valset will have signed every block, as it might be down (deliberately, for maintenance, or accidentally) at a given height. Thus we should always expect less than the full valset in the data. The screenshots from the Mintscan UI will thus slightly differ, as a validator won't be removed from the set unless it unbonds or is jailed.

## Snapshot heights:

- [akash: 15095285](https://mintscan.io/akash/blocks/15095285)
- [cosmos hub: 19237098](https://mintscan.io/cosmos/blocks/19237098)
- [dydx: 9001705](https://mintscan.io/dydx/blocks/9001705)
- [evmos: 19110795](https://mintscan.io/evmos/blocks/19110795)
- [juno: 14047318](https://mintscan.io/juno/blocks/14047318)
- [osmosis: 13899126](https://mintscan.io/osmosis/blocks/13899126)
- [sei: 59029917](https://mintscan.io/sei/blocks/59029917)

# CVPTST

Or, Cosmos Voting Power Time Series Tools to you and I.

```
RUST_BACKTRACE=1 cargo run \
  ./path/to/validators_data.json \
  true \

```

The third argument is whether the metadata is included in the file. Different networks' RPCs include it or don't. Thus:

```
RUST_BACKTRACE=1 cargo run ../sei/ ./output false
```

```
RUST_BACKTRACE=1 cargo run ../dydx/ ./output true
```

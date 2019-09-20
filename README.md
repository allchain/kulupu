# Kulupu

Proof-of-work blockchain built on
[Substrate](https://github.com/paritytech/substrate).

## Overview

Kulupu is a sister project related to [Solri](https://solri.org). Kulupu's goal
is to create a working proof-of-work blockchain built using unmodified Substrate
blockchain framework. Compared with Solri, Kulupu aims to take a more practical
approach of an on-chain governed self-updating blockchain, while Solri maintains
the ideal minimalist blockchain design.

The consensus engine for Kulupu is the CPU mining algorithm RandomX. For initial
launch, the emission rate is fixed at one coin per second. This, however can be
changed using hard fork or on-chain governance in the future.

## Network Launch

The first launch attempt is on! We currently do not provide any official binary
release, so please compile the node by yourself, using the instructions below.

Launch attempt means that it's an experimental launch. We relaunch the network
when bugs are found. Otherwise, the current network becomes the mainnet.

Substrate contains a variety of features including smart contracts and
democracy. However, for initial launch of Kulupu, we plan to only enable basic
balance and transfer module. This is to keep the network focused, and reduce
risks in terms of stability and safety. Also note that initially the democracy
module is also disabled, meaning we'll be updating runtime via hard fork until
that part is enabled.

## Build

Install Rust:

```bash
curl https://sh.rustup.rs -sSf | sh
```

Install required tools:

```bash
./scripts/init.sh
```

Build Wasm and native code:

```bash
cargo build
```

## Run

### Single node development chain

You can start a development chain with:

```bash
cargo run -- --dev
```

Detailed logs may be shown by running the node with the following environment variables set: `RUST_LOG=debug RUST_BACKTRACE=1 cargo run -- --dev`.

### Multi-node local testnet

If you want to see the multi-node consensus algorithm in action locally, then you can create a local testnet with two validator nodes for Alice and Bob, who are the initial authorities of the genesis chain that have been endowed with testnet units.

Optionally, give each node a name and expose them so they are listed on the Polkadot [telemetry site](https://telemetry.polkadot.io/#/Local%20Testnet).

You'll need two terminal windows open.

We'll start Alice's substrate node first on default TCP port 30333 with her chain database stored locally at `/tmp/alice`. The bootnode ID of her node is `QmRpheLN4JWdAnY7HGJfWFNbfkQCb6tFf4vvA6hgjMZKrR`, which is generated from the `--node-key` value that we specify below:

```bash
cargo run -- \
  --base-path /tmp/alice \
  --chain=local \
  --alice \
  --node-key 0000000000000000000000000000000000000000000000000000000000000001 \
  --telemetry-url ws://telemetry.polkadot.io:1024 \
  --validator
```

In the second terminal, we'll start Bob's substrate node on a different TCP port of 30334, and with his chain database stored locally at `/tmp/bob`. We'll specify a value for the `--bootnodes` option that will connect his node to Alice's bootnode ID on TCP port 30333:

```bash
cargo run -- \
  --base-path /tmp/bob \
  --bootnodes /ip4/127.0.0.1/tcp/30333/p2p/QmRpheLN4JWdAnY7HGJfWFNbfkQCb6tFf4vvA6hgjMZKrR \
  --chain=local \
  --bob \
  --port 30334 \
  --telemetry-url ws://telemetry.polkadot.io:1024 \
  --validator
```

Additional CLI usage options are available and may be shown by running `cargo run -- --help`.

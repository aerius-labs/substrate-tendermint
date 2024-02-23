# Substrate Tendermint

Welcome to the code library for the Substrate-based Tendermint Consensus!

## Introduction

This repository implements the Tendermint Consensus algorithm for the substrate blockchain framework.

This repository includes:

- The main implementation of Tendermint Consensus in ```tendermint/consensus``` and ```tendermint/finality-tendermint/```.
- And an Intergration example of node with Tendermint Consensus.

You can find more detailed descriptions in their individual READMEs.

## Build

### Setting Up the Stable Toolchain

```bash
rustup default stable
rustup update
```

### Adding the Nightly Toolchain and wasm Target

```bash
rustup update nightly
rustup target add wasm32-unknown-unknown --toolchain nightly
```

### Compile the node template

```bash
cargo build --release
```

### Single-Node Development Chain 

```bash
./target/release/node-template --dev
```

## Project Structure

This project is divided into two parts first is **node** and other is **tendermint**. Node includes runtime and node in it and Tendermint include consensus specific logic. 

### Node

A blockchain node is an application that allows users to participate in a blockchain network.
Substrate-based blockchain nodes expose a number of capabilities:

- Networking: Substrate nodes use the [`libp2p`](https://libp2p.io/) networking stack to allow the
  nodes in the network to communicate with one another.
- Consensus: Blockchains must have a way to come to [consensus](https://docs.substrate.io/fundamentals/consensus/) on the state of the network.
  Substrate makes it possible to supply custom consensus engines and also ships with several consensus mechanisms that have been built on top of [Web3 Foundation research](https://research.web3.foundation/en/latest/polkadot/NPoS/index.html).
- RPC Server: A remote procedure call (RPC) server is used to interact with Substrate nodes.

There are several files in the `node` directory.
Take special note of the following:

- [`chain_spec.rs`](./node-template/node/src/chain_spec.rs): A [chain specification](https://docs.substrate.io/build/chain-spec/) is a source code file that defines a Substrate chain's initial (genesis) state.
  Chain specifications are useful for development and testing, and critical when architecting the launch of a production chain.
  Take note of the `development_config` and `testnet_genesis` functions,.
  These functions are used to define the genesis state for the local development chain configuration.
  These functions identify some [well-known accounts](https://docs.substrate.io/reference/command-line-tools/subkey/) and use them to configure the blockchain's initial state.
- [`service.rs`](./node-template/node/node/src/service.rs): This file defines the node implementation.
  Take note of the libraries that this file imports and the names of the functions it invokes.
  In particular, there are references to consensus-related topics, such as the [block finalization and forks](https://docs.substrate.io/fundamentals/consensus/#finalization-and-forks) and other [consensus mechanisms](https://docs.substrate.io/fundamentals/consensus/#default-consensus-models) such as Aura for block authoring and Tendermint for finality.


## Runtime

In Substrate, the concepts of "runtime" and "state transition function" are used interchangeably. These terms describe the fundamental mechanism of the blockchain that validates blocks and implements the state modifications they prescribe.

This project uses [FRAME](https://docs.substrate.io/learn/runtime-development/#frame) to construct a blockchain runtime. Also tendermint pallet is added into the runtime. 

Review the [FRAME runtime implementation](./node-template/runtime/src/lib.rs) included in this node and note the following:

- This file configures several pallets to include in the runtime.
  Each pallet configuration is defined by a code block that begins with `impl $PALLET_NAME::Config for Runtime`.
- The pallets are composed into a single runtime by way of the [`construct_runtime!`](https://paritytech.github.io/substrate/master/frame_support/macro.construct_runtime.html) macro, which is part of the [core FRAME pallet library](https://docs.substrate.io/reference/frame-pallets/#system-pallets).


## Pallets

The runtime in this project is constructed using many FRAME pallets that ship with [the Substrate repository](https://github.com/paritytech/substrate/tree/master/frame) and a tendermint pallet that is [defined in the `pallets`](./tendermint/pallets/tendermint/src/lib.rs) directory.

A FRAME pallet is comprised of a number of blockchain primitives, including:

- Storage: FRAME defines a rich set of powerful [storage abstractions](https://docs.substrate.io/build/runtime-storage/) that makes it easy to use Substrate's efficient key-value database to manage the evolving state of a blockchain.
- Dispatchables: FRAME pallets define special types of functions that can be invoked (dispatched) from outside of the runtime in order to update its state.
- Events: Substrate uses [events](https://docs.substrate.io/build/events-and-errors/) to notify users of significant state changes.
- Errors: When a dispatchable fails, it returns an error.

Each pallet has its own `Config` trait which serves as a configuration interface to generically define the types and parameters it depends on.


## References

- [Substrate-hotstuff](https://github.com/generative-Labs/Substrate-HotStuff/tree/main)
- [Substrate-MCA](https://github.com/fky2015/substrate-MCA/tree/main)
- [Polkadot-SDK](https://github.com/paritytech/polkadot-sdk)


# Introduction
A Tendermint-based block finality tool implemented for substrate. It can replace Grandpa. 

# Usage
Tendermint consensus is divided into four crates: 

1. sp-finality-tendermint: Defines some common types and traits.
2. sc-finality-tendermint: This includes client. 
3. finality-tendermint: This defines the tendermint consensus.
4. pallet-tendermint: On-chain state management pallet for Tendermint consensus.

If you want to use the Tendermint consensus from this repository in substrate, all three of the above crates must be integrated.

## Runtime
First, we need to integrate `pallet-tendermint` into the runtime.

```

...
    // Import Tendermint Authoritylist type and primitives from pallet_tendermint
    use pallet_tendermint::{
        fp_primitives, AuthorityList as TendermintAuthorityList,
    };
...

    // Config pallet for runtime
    impl pallet_tendermint::Config for Runtime {
        type MaxAuthorities = ConstU32<32>;
    }

    // Install pallet_tendermint in the runtime
    construct_runtime!(
    pub struct Runtime {
        ....
        // Include tendermint consensus pallet
		Tendermint: pallet_tendermint,
        ....
	});

    // Implement the `pallet-tendermint` API for runtime
    impl_runtime_apis! {
        ...
        impl fp_primitives::TendermintApi<Block> for Runtime {
            fn tendermint_authorities() -> TendermintAuthorityList {
                Tendermint::authorities()
            }

            fn current_set_id() -> fp_primitives::SetId {
                Tendermint::current_set_id()
            }
        }
        ...
    }

    // Configure the session key
    impl_opaque_keys! {
        pub struct SessionKeys {
            ...
            pub tendermint: Tendermint,
            ...
        }
    }
```

## Client service

`tendermint-consensus` must be launched within the node service for the tendermint consensus to function. In the typical Substrate node service startup process, `new_partial` and `new_full` are core functions. 

```
pub fn new_partial(
	config: &Configuration,
) -> Result<
	sc_service::PartialComponents<
		FullClient,
		FullBackend,
		FullSelectChain,
		sc_consensus::DefaultImportQueue<Block, FullClient>,
		sc_transaction_pool::FullPool<Block, FullClient>,
		(   
            // TendermintBlockImport will send imported block to tendermint consensus
            sc_finality_tendermint::TendermintBlockImport<
                FullBackend,
                Block,
                FullClient,
                FullSelectChain,
            >,
            // LinkHalf is something like `client`, `backend`, `network`, and other components needed by the tendermint consensus.
            sc_finality_tendermint::LinkHalf<Block, FullClient, FullSelectChain>,
            Option<Telemetry>,
		),
	>,
	ServiceError,
> {
    ... 
    let (tendermint_block_import, tendermint_link) = sc_finality_tendermint::block_import(
        client.clone(),
        &client,
        select_chain.clone(),
        telemetry.as_ref().map(|x| x.handle()),
    )?;
    ...

    return 	Ok(sc_service::PartialComponents {
		...
		other: (tendermint_block_import, tendermint_link, telemetry),
	})
}

```

```
   pub fn new_full(config: Configuration) -> Result<TaskManager, ServiceError>{
      ...
      	let tendermint_protocol_name = sc_finality_tendermint::protocol_standard_name(
            &client
                .block_hash(0)
                .ok()
                .flatten()
                .expect("Genesis block exists; qed"),
            &config.chain_spec,
        );

	   net_config.add_notification_protocol(sc_finality_tendermint::tendermint_peers_set_config(
        tendermint_protocol_name.clone(),
    ));
      ...  
        //Configure tendermint 
        let tendermint_config = sc_finality_tendermint::TendermintParams {
            config: tendermint_config,
            link: tendermint_link,
            network,
            sync: Arc::new(sync_service),
            prometheus_registry,
            shared_voter_state: SharedVoterState::empty(),
            telemetry: telemetry.as_ref().map(|x| x.handle()),
        };

        // Start tendermint consensus network
	    task_manager.spawn_essential_handle().spawn_blocking(
            "tendermint-voter",
            None,
            sc_finality_tendermint::run_tendermint_voter(tendermint_config)?,
        );
       ...
   }
```

## Chain spec
Tendermint pallet must be intialized in the ChainSpec, or it will result in an incorrect genesis.

```
    RuntimeGenesisConfig {
            ...
            tendermint: TendermintConfig {
                authorities: initial_authorities.iter().map(|x| (x.1.clone())).collect(),
                ..Default::default()
            },
            ...
    }
```

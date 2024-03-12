use core::pin::Pin;

use futures::channel::mpsc::{self, UnboundedSender};
use futures::{channel::mpsc::UnboundedReceiver, Sink};
use futures::{Future, Stream};

use parking_lot::Mutex;
use std::sync::Arc;

use crate::environment::{Environment, RoundData, VoterData};
use crate::messages::{
    BlockFinalizationData, GlobalMessageIn, GlobalMessageOut, Message, SignedMessage,
};
use crate::testing::network::VoterState;
use crate::{Error, VoterSet};

/// A implementation of `Environment`.
use super::{chain::DummyChain, network::Network, *};

pub struct DummyEnvironment {
    local_id: Id,
    network: Network,
    listeners: Mutex<Vec<UnboundedSender<(Hash, BlockNumber)>>>,
    chain: Mutex<DummyChain>,
    _voters: Arc<Mutex<VoterSet<Id>>>,
}

impl DummyEnvironment {
    pub fn new(network: Network, local_id: Id, voters: Arc<Mutex<VoterSet<Id>>>) -> Self {
        DummyEnvironment {
            _voters: voters,
            network,
            local_id,
            chain: Mutex::new(DummyChain::new()),
            listeners: Mutex::new(Vec::new()),
        }
    }

    pub fn with_chain<F, U>(&self, f: F) -> U
    where
        F: FnOnce(&mut DummyChain) -> U,
    {
        let mut chain = self.chain.lock();
        f(&mut chain)
    }

    pub fn finalized_stream(&self) -> UnboundedReceiver<(Hash, BlockNumber)> {
        let (tx, rx) = mpsc::unbounded();
        self.listeners.lock().push(tx);
        rx
    }
}

impl Environment for DummyEnvironment {
    type Timer = Box<dyn Future<Output = Result<(), Error>> + Unpin + Send>;
    type Id = Id;
    type Signature = Signature;
    type BestChain =
        Box<dyn Future<Output = Result<Option<(Self::Number, Self::Hash)>, Error>> + Unpin + Send>;
    type In = Box<
        dyn Stream<Item = Result<SignedMessage<BlockNumber, Hash, Signature, Id>, Error>>
            + Unpin
            + Send,
    >;
    type Out = Pin<Box<dyn Sink<Message<BlockNumber, Hash>, Error = Error> + Send>>;
    type Error = Error;
    type Hash = Hash;
    type Number = BlockNumber;
    type GlobalIn = Box<
        dyn Stream<
                Item = Result<
                    GlobalMessageIn<Self::Hash, Self::Number, Self::Signature, Self::Id>,
                    Self::Error,
                >,
            > + Unpin
            + Send,
    >;
    type GlobalOut = Pin<
        Box<
            dyn Sink<
                    GlobalMessageOut<Self::Hash, Self::Number, Self::Signature, Self::Id>,
                    Error = Error,
                > + Send,
        >,
    >;

    fn init_voter(&self) -> VoterData<Self::Id> {
        let _globals = self.network.make_global_comms(self.local_id);
        VoterData {
            local_id: self.local_id,
        }
    }

    fn init_round(&self, round: u64) -> RoundData<Self::Id, Self::In, Self::Out> {
        tracing::trace!("{:?} round_data view: {}", self.local_id, round);

        let (incomming, outgoing) = self.network.make_round_comms(round, self.local_id);
        RoundData {
            local_id: self.local_id,
            incoming: Box::new(incomming),
            outgoing: Box::pin(outgoing),
        }
    }

    fn finalize_block(
        &self,
        data: BlockFinalizationData<Self::Number, Self::Hash, Self::Signature, Self::Id>,
    ) -> Result<(), Self::Error> {
        tracing::trace!("{:?} finalize_block", self.local_id);
        self.chain.lock().finalize_block(data.target_hash);
        self.listeners.lock().retain(|s| {
            s.unbounded_send((data.target_hash, data.target_number))
                .is_ok()
        });

        // Update Network RoutingRule's state.
        self.network.rule.lock().update_state(
            self.local_id,
            VoterState {
                _view_number: data.round,
                last_finalized: data.target_number,
            },
        );

        Ok(())
    }

    fn propose(&self, _round: u64, _block: Self::Hash) -> Self::BestChain {
        Box::new(futures::future::ok(Some(self.with_chain(|chain| {
            chain
                .next_to_be_finalized()
                .unwrap_or_else(|_| chain.last_finalized())
        }))))
    }

    // fn preprepare(&self, _view: u64, _block: Self::Hash) -> Self::BestChain {
    //     Box::new(futures::future::ok(Some(self.with_chain(|chain| {
    //         chain
    //             .next_to_be_finalized()
    //             .unwrap_or_else(|_| chain.last_finalized())
    //     }))))
    // }

    // fn complete_f_commit(
    //     &self,
    //     _view: u64,
    //     _state: crate::leader::State<Self::Number, Self::Hash>,
    //     _base: (Self::Number, Self::Hash),
    //     _f_commit: FinalizedCommit<Self::Number, Self::Hash, Self::Signature, Self::Id>,
    // ) -> Result<(), Self::Error> {
    //     Ok(())
    // }
}

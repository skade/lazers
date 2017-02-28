```rust
use super::Replicator;
use lazers_traits::prelude::*;

use futures::Future;
use futures::BoxFuture;
use futures::failed;
use futures::finished;

use std::sync::Arc;
use backtrace;
use super::PeersVerified as ReplicatorPeersVerified;

use std::convert::From as TransitionFrom;
```

## Get peers information

We implement the peer information retrieval as described
[here](http://docs.couchdb.org/en/2.0.0/replication/protocol.html#get-peers-information).

```rust
pub trait State {}

pub struct VerifiedPeers;
impl State for VerifiedPeers {}

pub struct GotSourceInformation;
impl State for GotSourceInformation {}

impl TransitionFrom<VerifiedPeers> for GotSourceInformation {
    fn from(_: VerifiedPeers) -> GotSourceInformation {
        GotSourceInformation
    }
}

pub struct GotTargetInformation;
impl State for GotTargetInformation {}

impl TransitionFrom<GotSourceInformation> for GotTargetInformation {
    fn from(_: GotSourceInformation) -> GotTargetInformation {
        GotTargetInformation
    }
}

pub struct GotPeersInformation;
impl State for GotPeersInformation {}

impl TransitionFrom<GotTargetInformation> for GotPeersInformation {
    fn from(_: GotTargetInformation) -> GotPeersInformation {
        GotPeersInformation
    }
}
```

We then define a `GetPeersInformation` struct to define the flow used in the first few steps. `GetPeersInformation` wraps the replicator struct for the duration of the process.

```rust
pub struct GetPeersInformation<From: Client + Send, To: Client + Send, S: State> {
    pub replicator: Replicator<From, To, ReplicatorPeersVerified>,
    #[allow(dead_code)]
    state: S
}

impl<From: Client + Send, To: Client + Send, T: State> GetPeersInformation<From, To, T> {
    fn transition<X: State + TransitionFrom<T>>(self, state: X) -> GetPeersInformation<From, To, X> {
        GetPeersInformation { replicator: self.replicator, state: state }
    }
}

/// TODO: definitely need to change the PeersVerified (Replicator state) <> VerfiedPeers (local state) confusion
impl<From: Client + Send + 'static, To: Client + Send + 'static> GetPeersInformation<From, To, VerifiedPeers> {
    pub fn new(replicator: Replicator<From, To, ReplicatorPeersVerified>) -> Self {
        GetPeersInformation { replicator: replicator, state: VerifiedPeers }
    }

    fn get_source_information(mut self) -> BoxFuture<GetPeersInformation<From, To, GotSourceInformation>, Error> {
        let future_db_info = self.replicator.from_db.as_ref().unwrap().info();

        future_db_info.and_then(|db_info| {
            self.replicator.from_db_info = Some(db_info);
            finished(self.transition(GotSourceInformation))
        }).boxed()
    }
}

impl<From: Client + Send + 'static, To: Client + Send + 'static> GetPeersInformation<From, To, GotSourceInformation> {
    fn get_target_information(mut self) -> BoxFuture<GetPeersInformation<From, To, GotTargetInformation>, Error> {
        let future_db_info = self.replicator.to_db.as_ref().unwrap().info();

        future_db_info.and_then(|db_info| {
            self.replicator.to_db_info = Some(db_info);
            finished(self.transition(GotTargetInformation))
        }).boxed()
    }
}
```

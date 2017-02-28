```rust
use super::Replicator;
use lazers_traits::prelude::*;

use futures::Future;
use futures::BoxFuture;
use futures::finished;

use super::PeerInformationReceived;
use super::ReplicatorState;

use std::convert::From as TransitionFrom;
use lazers_traits::SimpleKey;

use super::documents::ReplicationLog;
```

## Find common Ancestry

We implement the find common ancestry algorithm as described
[here](http://docs.couchdb.org/en/2.0.0/replication/protocol.html#find-common-ancestry).

```rust
pub trait State {}

pub struct Start;
impl State for Start {}

pub struct GeneratedReplicationId;
impl State for GeneratedReplicationId {}

impl TransitionFrom<Start> for GeneratedReplicationId {
    fn from(_: Start) -> GeneratedReplicationId {
        GeneratedReplicationId
    }
}

pub struct GotSourceReplicationLog;
impl State for GotSourceReplicationLog {}

impl TransitionFrom<GeneratedReplicationId> for GotSourceReplicationLog {
    fn from(_: GeneratedReplicationId) -> GotSourceReplicationLog {
        GotSourceReplicationLog
    }
}

pub struct GotTargetReplicationLog;
impl State for GotTargetReplicationLog {}

impl TransitionFrom<GotSourceReplicationLog> for GotTargetReplicationLog {
    fn from(_: GotSourceReplicationLog) -> GotTargetReplicationLog {
        GotTargetReplicationLog
    }
}

pub struct ComparedReplicationLog;
impl State for ComparedReplicationLog {}

impl TransitionFrom<GotTargetReplicationLog> for ComparedReplicationLog {
    fn from(_: GotTargetReplicationLog) -> ComparedReplicationLog {
        ComparedReplicationLog
    }
}
```

We then define a `GetPeersInformation` struct to define the flow used in the first few steps. `GetPeersInformation` wraps the replicator struct for the duration of the process.

```rust
pub struct FindCommonAncestry<From: Client + Send, To: Client + Send, S: State> {
    pub replicator: Replicator<From, To, PeerInformationReceived>,
    pub source_replication_log: Option<ReplicationLog>,
    pub target_replication_log: Option<ReplicationLog>,
    #[allow(dead_code)]
    state: S
}

impl<From: Client + Send, To: Client + Send, T: State> FindCommonAncestry<From, To, T> {
    fn transition<X: State + TransitionFrom<T>>(self, state: X) -> FindCommonAncestry<From, To, X> {
        FindCommonAncestry { replicator: self.replicator, source_replication_log: self.source_replication_log, target_replication_log: self.target_replication_log, state: state }
    }
}

impl<From: Client + Send + 'static, To: Client + Send + 'static> FindCommonAncestry<From, To, Start> {
    pub fn new(replicator: Replicator<From, To, PeerInformationReceived>) -> Self {
        FindCommonAncestry { replicator: replicator, source_replication_log: None, target_replication_log: None, state: Start }
    }

    pub fn generate_replication_id(mut self) -> BoxFuture<FindCommonAncestry<From, To, GeneratedReplicationId>, Error> {
        let replication_id = self.replicator.replication_id("yihaaaaw");

        finished(self.transition(GeneratedReplicationId)).boxed()
    }

}

impl<From: Client + Send + 'static, To: Client + Send + 'static> FindCommonAncestry<From, To, GeneratedReplicationId> {
    pub fn get_source_replication_log(mut self) -> BoxFuture<FindCommonAncestry<From, To, GotSourceReplicationLog>, Error> {
        let future_doc = {
            let db = self.replicator.from_db.as_ref().unwrap();
            let replication_id = self.replicator.replication_id.as_ref().unwrap();
            let key = SimpleKey::from(format!("_local/{}", replication_id));
            db.doc::<SimpleKey, ReplicationLog>(key)
        };

        future_doc.and_then(move |doc| {
            self.source_replication_log = None;
            finished(self.transition(GotSourceReplicationLog))
        }).boxed()
    }
}
```

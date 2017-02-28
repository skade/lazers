```rust
use super::Replicator;
use lazers_traits::prelude::*;

use futures::Future;
use futures::BoxFuture;
use futures::failed;
use futures::finished;

use super::Unconnected as ReplicatorUnconnected;

use std::convert::From as TransitionFrom;
```

## Verify peers

We implement the peer verification as described
[here](http://docs.couchdb.org/en/2.0.0/replication/protocol.html#verify-peers).

We follow a state-machine like pattern here and name all possible states
first. We label all states by using zero sized structs. They only serve as
information for the type system.

Connections between states are implemented using the `From` trait, aliased as `TransitionFrom`.

```rust
pub trait State {}

pub struct Unconnected;
impl State for Unconnected {}

pub struct SourceExisting;
impl State for SourceExisting {}

impl TransitionFrom<Unconnected> for SourceExisting {
    fn from(_: Unconnected) -> SourceExisting {
        SourceExisting
    }
}

pub struct TargetAbsent;
impl State for TargetAbsent {}

impl TransitionFrom<SourceExisting> for TargetAbsent {
    fn from(_: SourceExisting) -> TargetAbsent {
        TargetAbsent
    }
}

pub struct TargetExisting;
impl State for TargetExisting {}

impl TransitionFrom<SourceExisting> for TargetExisting {
    fn from(_: SourceExisting) -> TargetExisting {
        TargetExisting
    }
}

impl TransitionFrom<TargetAbsent> for TargetExisting {
    fn from(_: TargetAbsent) -> TargetExisting {
        TargetExisting
    }
}
```

We then define a `VerifyPeers` struct to define the flow used in the first
few steps. `VerifyPeers` wraps the replicator struct for the duration of the process.

```rust
pub struct VerifyPeers<From: Client + Send, To: Client + Send, S: State> {
    pub replicator: Replicator<From, To, ReplicatorUnconnected>,
    #[allow(dead_code)]
    state: S
}

impl<From: Client + Send, To: Client + Send, T: State> VerifyPeers<From, To, T> {
    fn transition<X: State + TransitionFrom<T>>(self, state: X) -> VerifyPeers<From, To, X> {
        VerifyPeers { replicator: self.replicator, state: state }
    }
}

impl<From: Client + Send + 'static, To: Client + Send + 'static> VerifyPeers<From, To, Unconnected> {
    pub fn new(replicator: Replicator<From, To, ReplicatorUnconnected>) -> Self {
        VerifyPeers { replicator: replicator, state: Unconnected }
    }

    pub fn verify_source(mut self) -> BoxFuture<VerifyPeers<From, To, SourceExisting>, Error> {
        let database = self.replicator.from_db_name.clone();

        let future_db_state = self.replicator.from.find_database(database);
        future_db_state.and_then(|db_state| {
            match db_state {
                DatabaseState::Existing(db) => {
                    self.replicator.from_db = Some(db);
                    finished(self.transition(SourceExisting)).boxed()
                }
                _ => {
                    failed(Error::from("Source doesn't exist")).boxed()
                }
            }
        }).boxed()
    }
}

impl<From: Client + Send + 'static, To: Client + Send + 'static> VerifyPeers<From, To, SourceExisting> {
    pub fn verify_target(mut self) -> BoxFuture<TargetBranch<From, To>, Error> {
        let database = self.replicator.to_db_name.clone();

        let future_db_state = self.replicator.to.find_database(database);
        future_db_state.and_then(|db_state| {
            match db_state {
                DatabaseState::Existing(db) => {
                    self.replicator.to_db = Some(db);
                    finished(TargetBranch::Existing(self.transition(TargetExisting))).boxed()
                }
                _ => {
                    finished(TargetBranch::Absent(self.transition(TargetAbsent))).boxed()
                }
            }
        }).boxed()
    }
}

impl<From: Client + Send + 'static, To: Client + Send + 'static> TargetBranch<From, To> {
    pub fn create_if_absent(self) -> BoxFuture<VerifyPeers<From, To, TargetExisting>, Error> {
        match self {
            TargetBranch::Existing(s) => finished(s).boxed(),
            TargetBranch::Absent(s) => {
                s.create_target()
            }
        }
    }

    pub fn fail_if_absent(self) -> BoxFuture<VerifyPeers<From, To, TargetExisting>, Error> {
        match self {
            TargetBranch::Existing(s) => finished(s).boxed(),
            TargetBranch::Absent(_) => {
                failed(Error::from("Target doesn't exist")).boxed()
            }
        }
    }
}

impl<From: Client + Send + 'static, To: Client + Send + 'static> VerifyPeers<From, To, TargetAbsent> {
    pub fn create_target(mut self) -> BoxFuture<VerifyPeers<From, To, TargetExisting>, Error> {
        let database = self.replicator.to_db_name.clone();

        let future_db_state = self.replicator.to.find_database(database);
        future_db_state.or_create().and_then(|db_state| {
            match db_state {
                DatabaseState::Existing(db) => {
                    self.replicator.to_db = Some(db);
                    finished(self.transition(TargetExisting)).boxed()
                }
                _ => {
                    failed(Error::from("Creation of target database failed")).boxed()
                }
            }
        }).boxed()
    }
}

pub enum TargetBranch<From: Client + Send, To: Client + Send> {
    Existing(VerifyPeers<From, To, TargetExisting>),
    Absent(VerifyPeers<From, To, TargetAbsent>)
}
```

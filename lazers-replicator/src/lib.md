# lazers-replicator

A replicator that takes a lazers DB and syncs couchdb data into it. It is
an implementation of the algorithm described here:
[here](http://docs.couchdb.org/en/1.6.1/replication/protocol.
html#replication-protocol-algorithm).

```rust
extern crate lazers_traits;
extern crate futures;
extern crate backtrace;
extern crate crypto;
#[macro_use]
extern crate derive_builder;

#[macro_use]
extern crate error_chain;

extern crate serde;
extern crate serde_json;

#[macro_use]
extern crate serde_derive;

use lazers_traits::prelude::*;

use futures::Future;
use futures::BoxFuture;
use futures::finished;

use std::convert::From as TransitionFrom;

mod utils;

pub mod errors;
```

## Document definitions

Document structures used throughout the process are wrapped in the [`documents`](/lazers-replicator/src/documents) module.

```rust
pub mod documents;
```

## Replicator

The standard replicator struct is just a pair of clients to sync from and to, along with the databases to use.

The two clients don't need to be of the same kind.

The Replicator itself has a high-level state machine.

```rust
pub struct Replicator<From: Client + Send, To: Client + Send, State: ReplicatorState> {
    from: From,
    to: To,
    from_db_name: DatabaseName,
    to_db_name: DatabaseName,
    from_db: Option<From::Database>,
    to_db: Option<To::Database>,
    from_db_info: Option<DatabaseInfo>,
    to_db_info: Option<DatabaseInfo>,
    replication_id: Option<String>,
    #[allow(dead_code)]
    state: State
}

impl<From: Client + Send, To: Client + Send> Replicator<From, To, Unconnected> {
    pub fn new(from: From, to: To, from_db: DatabaseName, to_db: DatabaseName) -> Replicator<From, To, Unconnected> {
        Replicator {
            from: from,
            to: to,
            from_db_name: from_db,
            to_db_name: to_db,
            from_db: None,
            to_db: None,
            from_db_info: None,
            to_db_info: None,
            replication_id: None,
            state: Unconnected
        }
    }
}
```

### The state machine

The Replicator state machine encodes all high-level steps descriped in the [CouchDB replication protocol](http://docs.couchdb.org/en/2.0.0/replication/protocol.html).

```rust
pub trait ReplicatorState {}

pub struct Unconnected;
impl ReplicatorState for Unconnected {}

pub struct PeersVerified;
impl ReplicatorState for PeersVerified {}

impl TransitionFrom<Unconnected> for PeersVerified {
    fn from(_: Unconnected) -> PeersVerified {
        PeersVerified
    }
}

pub struct PeerInformationReceived;
impl ReplicatorState for PeerInformationReceived {}

impl TransitionFrom<PeersVerified> for PeerInformationReceived {
    fn from(_: PeersVerified) -> PeerInformationReceived {
        PeerInformationReceived
    }
}

impl<From: Client + Send, To: Client + Send, T: ReplicatorState> Replicator<From, To, T> {
    fn transition<X: ReplicatorState + TransitionFrom<T>>(self, state: X) -> Replicator<From, To, X> {
        Replicator { state: state, from: self.from, to: self.to, from_db: self.from_db, to_db: self.to_db, from_db_name: self.from_db_name, to_db_name: self.to_db_name, from_db_info: self.from_db_info, to_db_info: self.to_db_info, replication_id: self.replication_id }
    }
}
```

## The replication process

The replication process is implemented in state machines wrapping the steps outlined in the CouchDB documentation, each implemented in a seperate module:

[`verify_peers`](/lazers-replicator/src/verify_peers) implements peer verification.

[`get_peers_information`](/lazers-replicator/src/get_peers_information) implements getting all important info from both peers.

[`find_common_ancestry`](/lazers-replicator/src/find_common_ancestry) implements resolution of the common ancestry between to databases.

```rust
mod verify_peers;
mod get_peers_information;
mod find_common_ancestry;
```

All these steps wrap the replicator type.

Finally, they are glued to the replicator as its public interface.

```rust
impl<From: Client + Send + 'static, To: Client + Send + 'static> Replicator<From, To, Unconnected> {
    pub fn verify_peers(self) -> BoxFuture<Replicator<From, To, PeersVerified>, Error> {
        self.setup_peers(false)
    }

    pub fn setup_peers(self, create_target: bool) -> BoxFuture<Replicator<From, To, PeersVerified>, Error> {
        let verifier = verify_peers::VerifyPeers::new(self);
        verifier.verify_source().and_then(|state| {
            state.verify_target()
        }).and_then(move |state| {
            if create_target {
                state.create_if_absent()
            } else {
                state.fail_if_absent()
            }
        }).and_then(|verifier| {
            finished(verifier.replicator.transition(PeersVerified))
        }).boxed()
    }
}

impl<From: Client + Send + 'static, To: Client + Send + 'static> Replicator<From, To, PeersVerified> {
    pub fn get_peers_information(self) -> BoxFuture<Replicator<From, To, PeerInformationReceived>, Error> {
        let steps = get_peers_information::GetPeersInformation::new(self);
        steps.get_source_information().and_then(|steps| {
            steps.get_target_information()
        }).and_then(|steps| {
            finished(steps.replicator.transition(PeerInformationReceived))
        }).boxed()
    }
}
```

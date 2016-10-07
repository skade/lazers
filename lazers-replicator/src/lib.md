# lazers-replicator

A replicator that takes a lazers DB and syncs couchdb data into it. It is
an implementation of the algorithm described here:
[here](http://docs.couchdb.org/en/1.6.1/replication/protocol.
html#replication-protocol-algorithm).

```rust
extern crate lazers_traits;
extern crate futures;
extern crate backtrace;
#[macro_use]
extern crate error_chain;

use lazers_traits::prelude::*;

use futures::Future;
use futures::BoxFuture;
use futures::finished;

use std::convert::From as TransitionFrom;

pub mod errors;
```


## Replicator

The standard replicator struct is just a pair of clients to sync from and to, along with the databases to use.

The two clients don't need to be of the same kind.

The Replicator itself has a high-level state machine.

```rust
pub struct Replicator<From: Client + Send, To: Client + Send, State: ReplicatorState> {
    from: From,
    to: To,
    from_db: DatabaseName,
    to_db: DatabaseName,
    #[allow(dead_code)]
    state: State
}

impl<From: Client + Send, To: Client + Send> Replicator<From, To, Unconnected> {
    pub fn new(from: From, to: To, from_db: DatabaseName, to_db: DatabaseName) -> Replicator<From, To, Unconnected> {
        Replicator {
            from: from,
            to: to,
            from_db: from_db,
            to_db: to_db,
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

impl<From: Client + Send, To: Client + Send, T: ReplicatorState> Replicator<From, To, T> {
    fn transition<X: ReplicatorState + TransitionFrom<T>>(self, state: X) -> Replicator<From, To, X> {
        Replicator { state: state, from: self.from, to: self.to, from_db: self.from_db, to_db: self.to_db }
    }
}
```

## The replication process

The replication process is implemented in state machines wrapping the steps outlined in the CouchDB documentation, each implemented in a seperate module:

[`verify_peers`](/lazers-traits/src/verify_peers) implements peer verification.

[`get_peers_information`](/lazers-traits/src/get_peers_information) implements getting all important info from both peers.

```rust
mod verify_peers;
//mod get_peers_information;
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
```

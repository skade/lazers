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

pub mod errors;
mod verify_peers;
```

## Replicator

The standard replicator struct is just a pair of clients to sync from and to, along with the databases to use.

The two clients don't need to be of the same kind.

```rust
pub struct Replicator<From: Client + Send, To: Client + Send> {
    from: From,
    to: To,
    from_db: DatabaseName,
    to_db: DatabaseName,
    create_target: bool
}

impl<From: Client + Send, To: Client + Send> Replicator<From, To> {
    pub fn new(from: From, to: To, from_db: DatabaseName, to_db: DatabaseName, create_target: bool) -> Replicator<From, To> {
        Replicator {
            from: from,
            to: to,
            from_db: from_db,
            to_db: to_db,
            create_target: create_target
        }
    }
}

impl<From: Client + Send + 'static, To: Client + Send + 'static> Replicator<From, To> {
    pub fn verify_peers(self) -> BoxFuture<(), Error> {
        self.setup_peers(false)
    }

    pub fn setup_peers(self, create_target: bool) -> BoxFuture<(), Error> {
        let verifier = verify_peers::VerifyPeers::new(self);
        verifier.verify_source().and_then(|state| {
            state.verify_target()
        }).and_then(move |state| {
            if create_target {
                state.create_if_absent()
            } else {
                state.fail_if_absent()
            }
        }).and_then(|_| {
            finished(())
        }).boxed()
    }
}
```

## Get Peers information

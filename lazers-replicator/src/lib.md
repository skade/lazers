# lazers-replicator

A replicator that takes a lazers DB and syncs couchdb data into it. It is
an implementation of the algorithm described here:
[here](http://docs.couchdb.org/en/1.6.1/replication/protocol.
html#replication-protocol-algorithm).

```rust
extern crate lazers_traits;
extern crate futures;
extern crate backtrace;

use lazers_traits::prelude::*;

use futures::Future;
use futures::BoxFuture;
use futures::failed;
use futures::finished;

use std::sync::Arc;

use std::marker::PhantomData;
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
        let verifier = VerifyPeers::new(self);
        verifier.verify_source().and_then(|state| {
            state.verify_target()
        }).and_then(|state| {
            state.fail_if_absent()
        }).and_then(|state| {
            finished(())
        }).boxed()
    }

    pub fn setup_peers(self, create_target: bool) -> BoxFuture<(), Error> {
        let verifier = VerifyPeers::new(self);
        verifier.verify_source().and_then(|state| {
            state.verify_target()
        }).and_then(move |state| {
            if create_target {
                state.create_if_absent()
            } else {
                state.fail_if_absent()
            }
        }).and_then(|state| {
            finished(())
        }).boxed()
    }
}
```

## Verify peers

We implement the peer verification as described
[here](http://docs.couchdb.org/en/1.6.1/replication/protocol.html#verify-peers).

We follow a state-machine like pattern here and name all possible states
first. We label all states by using zero sized structs. They only serve as
information for the type system.

```rust
trait State {}

struct Unconnected;
impl State for Unconnected {}

struct SourceExisting;
impl State for SourceExisting {}

struct TargetExisting;
impl State for TargetExisting {}

struct TargetAbsent;
impl State for TargetAbsent {}

type VerifyError = String;
struct Abort(VerifyError);
```

We then define a `VerifyPeers` struct to define the flow used in the first
few steps. `VerifyPeers` also wraps an instance of a `CouchDB` client.

```rust
struct VerifyPeers<From: Client + Send, To: Client + Send, State> {
    replicator: Replicator<From, To>,
    state: State
}

impl<From: Client + Send, To: Client + Send, T> VerifyPeers<From, To, T> {
    fn transition<X: State>(self, state: X) -> VerifyPeers<From, To, X> {
        VerifyPeers { replicator: self.replicator, state: state }
    }
}

impl<From: Client + Send + 'static, To: Client + Send + 'static> VerifyPeers<From, To, Unconnected> {
    fn new(replicator: Replicator<From, To>) -> Self {
        VerifyPeers { replicator: replicator, state: Unconnected }
    }

    fn verify_source(self) -> BoxFuture<VerifyPeers<From, To, SourceExisting>, Error> {
        let database = self.replicator.from_db.clone();

        let future_db_state = self.replicator.from.find_database(database);
        future_db_state.and_then(|db_state| {
            if db_state.existing() {
                finished(self.transition(SourceExisting)).boxed()
            } else {
                failed(error("Source doesn't exist".into(), backtrace::Backtrace::new())).boxed()
            }
        }).boxed()
    }
}

impl<From: Client + Send + 'static, To: Client + Send + 'static> VerifyPeers<From, To, SourceExisting> {
    fn verify_target(self) -> BoxFuture<TargetBranch<From, To>, Error> {
        let database = self.replicator.to_db.clone();

        let future_db_state = self.replicator.to.find_database(database);
        future_db_state.and_then(|db_state| {
            if db_state.existing() {
                finished(TargetBranch::Existing(self.transition(TargetExisting))).boxed()
            } else {
                finished(TargetBranch::Absent(self.transition(TargetAbsent))).boxed()
            }
        }).boxed()
    }
}

impl<From: Client + Send + 'static, To: Client + Send + 'static> TargetBranch<From, To> {
    fn create_if_absent(self) -> BoxFuture<VerifyPeers<From, To, TargetExisting>, Error> {
        match self {
            TargetBranch::Existing(s) => finished(s).boxed(),
            TargetBranch::Absent(s) => {
                s.create_target()
            }
        }
    }

    fn fail_if_absent(self) -> BoxFuture<VerifyPeers<From, To, TargetExisting>, Error> {
        match self {
            TargetBranch::Existing(s) => finished(s).boxed(),
            TargetBranch::Absent(_) => {
                failed(error("Target doesn't exist".into(), backtrace::Backtrace::new())).boxed()
            }
        }
    }
}

impl<From: Client + Send + 'static, To: Client + Send + 'static> VerifyPeers<From, To, TargetAbsent> {
    fn create_target(self) -> BoxFuture<VerifyPeers<From, To, TargetExisting>, Error> {
        let database = self.replicator.to_db.clone();

        let future_db_state = self.replicator.to.find_database(database);
        future_db_state.or_create().and_then(|_| {
            finished(self.transition(TargetExisting))
        }).boxed()
    }
}

enum TargetBranch<From: Client + Send, To: Client + Send> {
    Existing(VerifyPeers<From, To, TargetExisting>),
    Absent(VerifyPeers<From, To, TargetAbsent>)
}

fn error(message: String, backtrace: backtrace::Backtrace) -> Error {
    Error(ErrorKind::ClientError(message), (None, Arc::new(backtrace)))
}
```

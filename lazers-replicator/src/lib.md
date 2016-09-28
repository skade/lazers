# lazers-replicator

A replicator that takes a lazers DB and syncs couchdb data into it. It is
an implementation of the algorithm described here:
[here](http://docs.couchdb.org/en/1.6.1/replication/protocol.
html#replication-protocol-algorithm).

```rust
extern crate hyper;
use std::marker::PhantomData;
```

## Verify peers

We implement the peer verification as described
[here](http://docs.couchdb.org/en/1.6.1/replication/protocol.
html#verify-peers).

We follow a state-machine like pattern here and name all possible states
first. We label all states by using zero sized structs. They only serve as
information for the type system.

```rust
struct Unconnected;
struct CheckedSourceExistence;
struct CheckTargetExistence;
struct CheckCreateTarget;
struct CreateTarget;
struct GetSourceInformation;
type VerifyError = String;
struct Abort(VerifyError);
```

We then define a `VerifyPeers` struct to define the flow used in the first
few steps. `VerifyPeers` also wraps an instance of a `CouchDB` client.

```rust
trait Client: Default {
    fn verify_existence<Url: hyper::client::IntoUrl>(&self, url: Url) -> Result<bool, String>;
}

struct HyperClient {
    inner: hyper::client::Client,
}

impl Default for HyperClient {
    fn default() -> HyperClient {
        HyperClient { inner: hyper::client::Client::new() }
    }
}

impl Client for HyperClient {
    fn verify_existence<Url: hyper::client::IntoUrl>(&self, url: Url) -> Result<bool, String> {
        self.inner
            .head(url)
            .send()
            .map(|_| true)
            .map_err(|e| e.to_string())
    }
}

trait Storage {
    fn check_database(&self, name: &str) -> Result<bool, String>;
}

impl<C: Client> Storage for RemoteCouchDB<C> {
    fn check_database(&self, name: &str) -> Result<bool, String> {
        self.client.verify_existence(&format!("{}/{}", self.base_url, name))
    }
}

struct RemoteCouchDB<C: Client = HyperClient> {
    client: C,
    base_url: String,
}

impl<C: Client> RemoteCouchDB<C> {
    fn new(base_url: String) -> RemoteCouchDB<C> {
        RemoteCouchDB {
            client: C::default(),
            base_url: base_url,
        }
    }
}

struct VerifyPeers<State> {
    source: Box<Storage>,
    marker: PhantomData<State>,
}

impl VerifyPeers<Unconnected> {
    fn check_source_existence
        (self,
         db_name: &str)
         -> Result<VerifyPeers<CheckedSourceExistence>, VerifyPeers<Abort>> {
        let source = match self {
            VerifyPeers { source: s, marker: _ } => s,
        };
        let res = source.check_database(db_name);

        match res {
            Ok(present) if present == true => {
                Ok(VerifyPeers {
                    source: source,
                    marker: PhantomData::<CheckedSourceExistence>,
                })
            }
            _ => {
                Err(VerifyPeers {
                    source: source,
                    marker: PhantomData::<Abort>,
                })
            }

        }
    }
}
```

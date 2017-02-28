# Replication utilities

This module contains utilities necessary to implement the CouchDB replication protocol.

## CouchDB Replication ID

The CouchDB replication protocol documents the following as inputs into the replication ID:

* Persistent Peer UUID value. For CouchDB, the local Server UUID is used
* Source and Target URI and if Source or Target are local or remote Databases
* If Target needed to be created
* If Replication is Continuous
* OAuth headers if any
* Any custom headers
* Filter function code if used
* Changes Feed query parameters, if any

As laze RS is not necessarily a server, all these parameters need to be passed to the replication protocol.

```rust
use super::ReplicatorState;
use super::Replicator;
use lazers_traits::Client;
use crypto::md5::Md5;
use crypto::digest::Digest;

impl<From: Client + Send, To: Client + Send, T: ReplicatorState> Replicator<From, To, T> {
    pub fn replication_id(self, peer_uuid: String, ) -> String {
        let mut md5 = Md5::new();
        md5.input_str("hello!");
        md5.result_str()
    }
}
```

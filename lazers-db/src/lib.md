# lazers - a CouchDB syncing library for Rust

lazers is a local database syncing with CouchDB.

It is developed in literal Rust, using [tango](https://github.com/pnkflx/tango).



```rust
extern crate lazers_traits;
extern crate lazers_hashmap;

use lazers_traits::Backend;

pub struct Database<B: Backend>{
    backend: B,
}


#[test]
fn some_db() {
    use lazers_hashmap::HashMapBackend;

    let hash = HashMapBackend::new();
    let mut db = Database { backend: hash };
    db.backend.set(1,2);
}
```

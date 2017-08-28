# Result Decorations

These implementations make it easier to work with the results given by the
traits described by the main module.

They decorate the respective results with generic operations while
propagating
previously occuring errors.

The pattern is described in detail
[here](http://yakshav.es/decorating-results).

## Imports

All types to be decorated and types necessary for interaction with them.

```rust
use super::DatabaseState;
use super::Database;
use super::DatabaseCreator;
use super::DatabaseEntry;
use super::Document;
use super::Key;

use result::Error;

use futures::BoxFuture;
use futures::future::{FutureResult,Either};
use futures::future::{ok, err};
use futures::Future;
use futures::finished;
use futures::done;
```

### Results of finding a Database

`FindDatabaseResult` decorates the result returned from finding a database.
The
operations provided are `or_create` and `and_delete`.

`or_create` creates the database if it was not present, otherwise, it just
returns the already-existing database. If and error occured in a previous
step,
the error is passed through and no attempt to create the database is
undertaken.

`and_delete` delete the database if it is present, otherwise, it just
returns
the already absent state. If and error occured in a previous step, the
error is
passed through and no attempt to create the database is undertaken.


```rust
type FindDatabaseFuture<D: Database> = Box<Future<Item=DatabaseState<D, D::Creator>, Error=Error>>;

pub trait FindDatabaseResult {
    type D: Database;

    fn or_create(self) -> FindDatabaseFuture<Self::D>;
    fn and_delete(self) -> FindDatabaseFuture<Self::D>;
}

impl<D: Database + 'static> FindDatabaseResult for FindDatabaseFuture<D> {
    type D = D;

    fn or_create(self) -> Self {
        let future = self.and_then({ |state|
            match state {
                DatabaseState::Existing(_) => Either::A(ok(state)),
                DatabaseState::Absent(creator) => Either::B(creator.create().and_then(|d| ok(DatabaseState::Existing(d)))),
            }
        });
        
        Box::new(future)
    }

    fn and_delete(self) -> Self {
        let future = self.and_then({ |state|
            match state {
                DatabaseState::Absent(c) => Either::A(ok(DatabaseState::Absent(c))),
                DatabaseState::Existing(d) => Either::B(d.destroy().and_then(|c| finished(DatabaseState::Absent(c)))),
            }
        });

        Box::new(future)
    }
}
```

### Results of retrieving documents

`DocumentResult` decorates the result returned from retrieving a document
from.
The operations provided are `get`, `set` and `delete`. If the result is
already
describing an error, that error is propagated.

`get` retrieves the document from the result and passes ownership to the
caller. It consumes the result. Getting an absent document or a collided
document is an error.

`set` changes the document stored under the given key. It consumes the
result
and returns another one instead, describing the new state of the document or
possibly an error.

`delete` deletes the document stored under the given key. It consumes the
result and returns another one instead, describing the new state of the
document or possibly an error.

```rust
pub trait DocumentResult {
    type K: Key;
    type D: Document;

    fn get(self) -> BoxFuture<Self::D, Error>;
    fn set(self, doc: Self::D) -> Self;
    fn delete(self) -> Self;
}

impl<K: Key + 'static, D: Document + 'static, DB: Database + 'static> DocumentResult for BoxFuture<DatabaseEntry<K, D, DB>, Error> {
    type K = K;
    type D = D;

    fn get(self) -> BoxFuture<Self::D, Error> {
        self.and_then(|entry| {
            let res = match entry {
                DatabaseEntry::Present { doc: d, .. } => Ok(d),
                DatabaseEntry::Absent { key, .. } => Err(key.id().to_string().into()),
                _ => panic!("conflicts are unimplemented"),
            };
            done(res).boxed()
        }).boxed()
    }

    fn set(self, doc: D) -> Self {
        self.and_then(|entry| {
            match entry {
                DatabaseEntry::Absent { key, database: db, .. } |
                DatabaseEntry::Present { key, database: db, .. } => {
                    db.insert(key, doc).and_then(|(key, doc)| {
                        let new_entry = DatabaseEntry::Present {
                            key: key,
                            doc: doc,
                            database: db,
                        };
                        finished(new_entry)
                    })
                }
                DatabaseEntry::Conflicted { .. } => panic!("unimplemented"),
            }
        }).boxed()
    }

    fn delete(self) -> Self {
        self.and_then(|entry| {
            match entry {
                DatabaseEntry::Present { key, database: db, .. } => {
                    // ignoring here is fine, the OK value is ()
                    db.delete(key.clone()).and_then( |_| {
                        let new_entry = DatabaseEntry::Absent {
                            key: key,
                            database: db,
                        };
                        finished(new_entry)
                    }).boxed()
                }
                a @ DatabaseEntry::Absent { .. } => finished(a).boxed(),
                DatabaseEntry::Conflicted { .. } => panic!("unimplemented"),
            }
        }).boxed()
    }
}
```

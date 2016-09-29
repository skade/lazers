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

use result::Result;
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
pub trait FindDatabaseResult {
    type D: Database;

    fn or_create(self) -> Self;
    fn and_delete(self) -> Self;
}

impl<D: Database> FindDatabaseResult for Result<DatabaseState<D, D::Creator>> {
    type D = D;

    fn or_create(self) -> Self {
        let state = try!(self);

        match state {
            DatabaseState::Existing(d) => Ok(DatabaseState::Existing(d)),
            DatabaseState::Absent(creator) => creator.create().map(|d| DatabaseState::Existing(d)),
        }
    }

    fn and_delete(self) -> Self {
        let state = try!(self);

        match state {
            DatabaseState::Absent(c) => Ok(DatabaseState::Absent(c)),
            DatabaseState::Existing(d) => d.destroy().map(|c| DatabaseState::Absent(c)),
        }
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

    fn get(self) -> Result<Self::D>;
    fn set(self, doc: Self::D) -> Self;
    fn delete(self) -> Self;
}

impl<'a, K: Key, D: Document, DB: Database> DocumentResult for Result<DatabaseEntry<'a, K, D, DB>> {
    type K = K;
    type D = D;

    fn get(self) -> Result<Self::D> {
        let entry = try!(self);

        match entry {
            DatabaseEntry::Present { doc: d, .. } => Ok(d),
            DatabaseEntry::Absent { key, .. } => Err(key.id().to_string().into()),
            _ => panic!("conflicts are unimplemented"),
        }
    }

    fn set(self, doc: D) -> Self {
        let entry = try!(self);

        match entry {
            DatabaseEntry::Absent { key, database: db, .. } |
            DatabaseEntry::Present { key, database: db, .. } => {
                match db.insert(key, doc) {
                    Ok((key, doc)) => {
                        Ok(DatabaseEntry::Present {
                            key: key,
                            doc: doc,
                            database: db,
                        })
                    }
                    Err(e) => Err(e),
                }
            }
            DatabaseEntry::Conflicted { .. } => panic!("unimplemented"),
        }
    }

    fn delete(self) -> Self {
        let entry = try!(self);

        match entry {
            DatabaseEntry::Present { key, database: db, .. } => {
                // ignoring here is fine, the OK value is ()
                let _ = try!{ db.delete(key.clone()) };
                Ok(DatabaseEntry::Absent {
                    key: key,
                    database: db,
                })
            }
            a @ DatabaseEntry::Absent { .. } => Ok(a),
            DatabaseEntry::Conflicted { .. } => panic!("unimplemented"),
        }
    }
}
```

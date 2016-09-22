# Result Decorations

```rust
use super::DatabaseState;
use super::Database;
use super::DatabaseCreator;
use super::DatabaseEntry;
use super::Document;
use super::Key;

use result::Result;

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
            DatabaseState::Absent(creator) => {
                creator.create().map(|d|
                    DatabaseState::Existing(d)
                )
            }
        }
    }

    fn and_delete(self) -> Self {
        let state = try!(self);
    
        match state {
            DatabaseState::Absent(c) => Ok(DatabaseState::Absent(c)),
            DatabaseState::Existing(d) => {
                d.destroy().map(|c|
                    DatabaseState::Absent(c)
                )
            }
        }
    }
}

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
            _ => panic!("collisions are unimplemented")
        }
    }

    fn set(self, doc: D) -> Self {
        let entry = try!(self);

        match entry {
            DatabaseEntry::Present { key, database: db, .. } => {
                match db.insert(key, doc) {
                    Ok((key, doc)) => {
                        Ok(DatabaseEntry::Present { key: key, doc: doc, database: db })
                    }
                    Err(e) => Err(e)
                }
            },
            DatabaseEntry::Absent { key, database: db, .. } => {
                match db.insert(key, doc) {
                    Ok((key, doc)) => {
                        Ok(DatabaseEntry::Present { key: key, doc: doc, database: db })
                    }
                    Err(e) => Err(e)
                }
            },
            DatabaseEntry::Conflicted { .. } => panic!("unimplemented")
        }
    }

    fn delete(self) -> Self {
        let entry = try!(self);

        match entry {
            DatabaseEntry::Present { key, database: db, .. } => {
                match db.delete(key.clone()) {
                    Ok(()) => {
                        Ok(DatabaseEntry::Absent { key: key, database: db })
                    }
                    Err(e) => Err(e)
                }
            },
            a @ DatabaseEntry::Absent { .. } => {
                Ok(a)
            },
            DatabaseEntry::Conflicted { .. } => panic!("unimplemented")
        }
    }
}
```

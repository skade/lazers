# lazers-traits - important traits for the library

```rust
use std::hash::Hash;
use std::borrow::Borrow;

pub type DatabaseName = String;
pub type Error = String;

pub trait Backend {
    type K: Eq;
    type V;

    fn get<Q: ?Sized>(&self, k: &Q) -> Option<&Self::V> where Self::K: Borrow<Q>, Q: Eq + Hash, Self : Sized;
    fn set(&mut self, k: Self::K, v: Self::V) -> Option<Self::V>;
    fn delete<Q: ?Sized>(&mut self, k: &Q) -> Option<Self::V> where Self::K: Borrow<Q>, Q: Eq + Hash, Self : Sized;
}

pub trait DatabaseCreator where Self: Sized {
    type D: Database;

    fn create(self) -> Result<Self::D, Error>;
}

pub trait Database where Self: Sized {
    type Creator: DatabaseCreator<D = Self>;

    fn delete(self) -> Result<Self::Creator, Error>;
}

pub enum DatabaseState<D: Database, C: DatabaseCreator> {
    Existing(D),
    Absent(C)
}

pub trait DatabaseResult {
    type D: Database;

    fn or_create(self) -> Self;
    fn and_delete(self) -> Self;
}

impl<D: Database> DatabaseResult for Result<DatabaseState<D, D::Creator>, Error> {
    type D = D;

    fn or_create(self) -> Self {
        self.and_then(|state|
            match state {
                DatabaseState::Existing(d) => Ok(DatabaseState::Existing(d)),
                DatabaseState::Absent(creator) => {
                    creator.create().map(|d|
                        DatabaseState::Existing(d)
                    )
                }
            }
        )
    }

    fn and_delete(self) -> Self {
        self.and_then(|state|
            match state {
                DatabaseState::Absent(c) => Ok(DatabaseState::Absent(c)),
                DatabaseState::Existing(d) => {
                    d.delete().map(|c|
                        DatabaseState::Absent(c)
                    )
                }
            }
        )
    }
}

impl<D: Database, C: DatabaseCreator> DatabaseState<D,C> {
    pub fn absent(&self) -> bool {
        match self {
            &DatabaseState::Absent(_) => true,
            _                         => false
        }
    }

    pub fn existing(&self) -> bool {
        !self.absent()
    }
}

pub trait Client : Default {
    type Database : Database;

    fn find_database(&self, name: DatabaseName) -> Result<DatabaseState<Self::Database, <<Self as Client>::Database as Database>::Creator>, Error>;
}
```

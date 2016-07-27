# lazers-traits - important traits for the library

```rust
extern crate serde;

use std::hash::Hash;
use std::borrow::Borrow;
use serde::de::Deserialize;
use serde::ser::Serialize;


pub type DatabaseName = String;
pub type Error = String;

pub type Result<T> = std::result::Result<T, Error>;

pub trait Backend where Self: Sized {
    type K: Eq;
    type V;

    fn get<Q: ?Sized>(&self, k: &Q) -> Option<&Self::V> where Self::K: Borrow<Q>, Q: Eq + Hash, Self : Sized;
    fn set(&mut self, k: Self::K, v: Self::V) -> Option<Self::V>;
    fn delete<Q: ?Sized>(&mut self, k: &Q) -> Option<Self::V> where Self::K: Borrow<Q>, Q: Eq + Hash, Self : Sized;
}

pub trait Document : Deserialize + Serialize where Self : Sized {

}

impl<D: Deserialize + Serialize + Sized> Document for D {

}

pub trait Key : Eq where Self: Sized {
    fn id(&self) -> &str;
    fn rev(&self) -> Option<&str>;
    fn from_id_and_rev(id: String, rev: Option<String>) -> Self;
}

#[derive(Clone,PartialEq,Eq)]
pub struct SimpleKey {
    pub id: String,
    pub rev: Option<String>
}

impl Key for SimpleKey {
    fn id(&self) -> &str {
        &self.id
    }
    
    fn rev(&self) -> Option<&str> {
        match self.rev {
            Some(ref string) => Some(string),
            None => None
        }
    }
    fn from_id_and_rev(id: String, rev: Option<String>) -> Self {
        SimpleKey { id: id, rev: rev }
    }
}

impl From<String> for SimpleKey {
    fn from(string: String) -> SimpleKey {
        SimpleKey { id: string, rev: None }
    }
}

pub trait DatabaseCreator where Self: Sized {
    type D: Database;

    fn create(self) -> Result<Self::D>;
}

pub enum DatabaseEntry<'a, K: Key, D: Document, DB: Database + 'a> {
    Present { key: K, doc: D, database: &'a DB },
    Absent  { key: K, database: &'a DB},
    Collided { key: K, documents: Vec<D>, database: &'a DB },
}

impl<'a, K: Key, D: Document, DB: Database> DatabaseEntry<'a, K, D, DB> {
    pub fn present(key: K, doc: D, database: &'a DB) -> DatabaseEntry<'a, K, D, DB> {
        DatabaseEntry::Present { key: key, doc: doc, database: database }
    }

    pub fn absent(key: K, database: &'a DB) -> DatabaseEntry<'a, K, D, DB> {
        DatabaseEntry::Absent { key: key, database: database}
    }

    pub fn exists(&self) -> bool {
        match self {
            &DatabaseEntry::Present { .. } | &DatabaseEntry::Collided { .. } => true,
            _ => false
        }
    }
}

pub trait DocumentResult {
    type K: Key;
    type D: Document;

    fn get(self) -> Result<Self::D>;
    fn set(self, doc: Self::D) -> Self;
}

impl<'a, K: Key, D: Document, DB: Database> DocumentResult for Result<DatabaseEntry<'a, K, D, DB>> {
    type K = K;
    type D = D;

    fn get(self) -> Result<Self::D> {
        match self {
            Ok(DatabaseEntry::Present { doc: d, .. }) => Ok(d),
            Ok(_) => Err("Document not available".into()),
            Err(e) => Err(e)
        }
    }

    fn set(self, doc: D) -> Self {
        match self {
            Ok(DatabaseEntry::Present { key: key, database: db, .. }) => {
                match db.insert(key, doc) {
                    Ok((key, doc)) => {
                        Ok(DatabaseEntry::Present { key: key, doc: doc, database: db })
                    }
                    Err(e) => Err(e)
                }
            },
            Ok(DatabaseEntry::Absent { key: key, database: db, .. }) => {
                match db.insert(key, doc) {
                    Ok((key, doc)) => {
                        Ok(DatabaseEntry::Present { key: key, doc: doc, database: db })
                    }
                    Err(e) => Err(e)
                }
            },
            Ok(DatabaseEntry::Collided { .. }) => panic!("unimplemented"),
            Err(e) => Err(e),
        }
    }
}

pub trait Database where Self: Sized {
    type Creator: DatabaseCreator<D = Self>;

    fn destroy(self) -> Result<Self::Creator>;
    fn doc<'a, K: Key, D: Document>(&'a self, key: K) -> Result<DatabaseEntry<'a, K, D, Self>>;
    fn insert<K: Key, D: Document>(&self, key: K, doc: D) -> Result<(K, D)>;
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

impl<D: Database> DatabaseResult for Result<DatabaseState<D, D::Creator>> {
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
                    d.destroy().map(|c|
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

    fn find_database(&self, name: DatabaseName) -> Result<DatabaseState<Self::Database, <<Self as Client>::Database as Database>::Creator>>;
}
```

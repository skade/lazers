# lazers-traits - important traits for the library

```rust
extern crate serde;

use std::hash::Hash;
use std::borrow::Borrow;
use serde::de::Deserialize;

use std::marker::PhantomData;

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

pub trait Document : Deserialize {

}

pub struct PresentDocument<K: Key, D: Document> {
    key: K,
    doc: D
}

impl<K: Key, D: Document> PresentDocument<K, D> {
    pub fn new(key: K, doc: D) -> PresentDocument<K, D> {
        PresentDocument { key: key, doc: doc}
    }
}

pub struct AbsentDocument<K: Key, D: Document> {
    key: K,
    doc: PhantomData<D>
}

impl<K: Key, D: Document> AbsentDocument<K, D> {
    pub fn new(key: K) -> AbsentDocument<K, D> {
        AbsentDocument { key: key, doc: PhantomData::default() }
    }
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

pub enum DatabaseEntry<K: Key, D: Document> {
    Present(PresentDocument<K, D>),
    Absent(AbsentDocument<K, D>),
    Collided(Vec<PresentDocument<K, D>>),
}

pub trait Database where Self: Sized {
    type Creator: DatabaseCreator<D = Self>;

    fn destroy(self) -> Result<Self::Creator>;
    fn doc<K: Key, D: Document>(&self, key: K) -> Result<DatabaseEntry<K, D>>;
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

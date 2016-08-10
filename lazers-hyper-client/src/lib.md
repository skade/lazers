# lazers-hyper-client

A CouchDB client implemented using hyper.

```rust
extern crate hyper;
extern crate url;
extern crate lazers_traits;
extern crate serde;
extern crate serde_json;
extern crate mime;
extern crate backtrace;

mod types;
use types::document_created::DocumentCreated;
use types::error;

use lazers_traits::Client;
use lazers_traits::DatabaseName;
use lazers_traits::Database;
use lazers_traits::DatabaseState;
use lazers_traits::DatabaseCreator;
use lazers_traits::Document;
use lazers_traits::DatabaseEntry;
use lazers_traits::Key;
use lazers_traits::result::Result;
use lazers_traits::result::Error;
use lazers_traits::result::ErrorKind;
use lazers_traits::result::ChainErr;
use serde_json::de::from_reader;
use serde_json::ser::to_string;

use lazers_traits::DatabaseResult;

use hyper::header::ETag;
use hyper::header::ContentType;

use hyper::status::StatusCode;
use std::sync::Arc;

use url::{Url};

pub struct HyperClient {
    inner: hyper::client::Client,
    base_url: Url,
}

impl Default for HyperClient {
    fn default() -> HyperClient {
        HyperClient {
            inner: hyper::client::Client::new(),
            base_url: Url::parse("http://localhost:5984")
                          .expect("this is a valid URL")
        }
    }
}

pub struct RemoteDatabaseCreator {
    name: DatabaseName,
    base_url: Url
}

pub struct RemoteDatabase {
    name: DatabaseName,
    base_url: Url
}

impl DatabaseCreator for RemoteDatabaseCreator {
    type D = RemoteDatabase;

    fn create(self) -> Result<RemoteDatabase> {
        let mut url = self.base_url.clone();
        url.set_path(self.name.as_ref());
        let client = hyper::client::Client::new();
        let res = client.put(url)
                        .send();
        try!(res.chain_err(|| self.name.clone() ));

        Ok(RemoteDatabase { name: self.name, base_url: self.base_url })
    }
}

impl Database for RemoteDatabase {
    type Creator = RemoteDatabaseCreator;

    fn destroy(self) -> Result<RemoteDatabaseCreator> {
        let mut url = self.base_url.clone();
        url.set_path(self.name.as_ref());
        let client = hyper::client::Client::new();
        let res = client.delete(url)
                        .send();

        try!(res.chain_err(|| self.name.clone() ));

        Ok(RemoteDatabaseCreator { name: self.name, base_url: self.base_url })
    }

    fn doc<'a, K: Key, D: Document>(&'a self, key: K) -> Result<DatabaseEntry<'a, K, D, RemoteDatabase>> {
        let mut url = self.base_url.clone();
        url.set_path(format!("{}/{}", self.name, key.id()).as_ref());
        let client = hyper::client::Client::new();
        let res = client.get(url)
                        .send();

        match res {
            Ok(r) => {
                match r.status {
                    StatusCode::Ok => {
                        let rev = r.headers.get::<ETag>().unwrap().clone();
                        let key_with_rev = <K as Key>::from_id_and_rev(key.id().to_owned(), Some(rev.tag().to_owned()));
                        let doc = from_reader(r).unwrap();
                        Ok(DatabaseEntry::present(key_with_rev, doc, self))
                    },
                    StatusCode::NotFound => Ok(DatabaseEntry::absent(key, self)),
                    _ => Err(Error(ErrorKind::ClientError(format!("Unexpected status: {}", r.status)), (None, Arc::new(backtrace::Backtrace::new()))))
                }
            },
            Err(e) => { Err(Error(ErrorKind::ClientError(format!("Unexpected HTTP error")), (Some(Box::new(e)), Arc::new(backtrace::Backtrace::new())))) }
        }
    }

    // this should probably be &doc, as Doc won't be changed, but might
    // get a new key
    fn insert<K: Key, D: Document>(&self, key: K, doc: D) -> Result<(K, D)> {
        println!("{:?}", key);
        let mut url = self.base_url.clone();
        url.set_path(format!("{}/{}", self.name, key.id()).as_ref());

        if let Some(rev) = key.rev() {
            url.query_pairs_mut().append_pair("rev", rev);
        }

        let client = hyper::client::Client::new();
        let body = match to_string(&doc) {
                     Ok(s) => s,
                     Err(e) => { return Err(Error(ErrorKind::ClientError(format!("Unexpected HTTP error")), (Some(Box::new(e)), Arc::new(backtrace::Backtrace::new())))) }
                   };

        let mime: mime::Mime = "application/json".parse().unwrap();
        let res = client.put(url)
                        .header(ContentType(mime))
                        .body(&body)
                        .send();
        match res {
            Ok(r) => {
                match r.status {
                    StatusCode::Created => {
                        let response_data: DocumentCreated = from_reader(r).unwrap();

                        let k = K::from_id_and_rev(response_data.id, Some(response_data.rev));

                        Ok((k, doc))
                    }
                    StatusCode::Conflict => {
                        let response_data: error::Error = from_reader(r).unwrap();
                        match response_data {
                            error::Error::Conflict(reason) => { Err(Error(ErrorKind::UpdateConflict(format!("Document update conflict: {}", reason)), (None, Arc::new(backtrace::Backtrace::new())))) },
                            error::Error::BadRequest(reason) => { Err(Error(ErrorKind::ClientError(format!("Bad Request: {}", reason)), (None, Arc::new(backtrace::Backtrace::new())))) },
                        }
                    }
                    _ => Err(Error(ErrorKind::ClientError(format!("Unexpected status: {}", r.status)), (None, Arc::new(backtrace::Backtrace::new()))))
                }
            },
            Err(e) => { Err(Error(ErrorKind::ClientError(format!("Unexpected HTTP error")), (Some(Box::new(e)), Arc::new(backtrace::Backtrace::new())))) }
        }
    }

    fn delete<K: Key>(&self, key: K) -> Result<()> {
        let mut url = self.base_url.clone();
        url.set_path(format!("{}/{}", self.name, key.id()).as_ref());
        url.query_pairs_mut().append_pair("rev", key.rev().unwrap());
        let client = hyper::client::Client::new();
        let res = client.delete(url)
                        .send();

        match res {
            Ok(r) => {
                match r.status {
                    StatusCode::Ok => { Ok(()) },
                    _ => Err(Error(ErrorKind::ClientError(format!("Unexpected status: {}", r.status)), (None, Arc::new(backtrace::Backtrace::new()))))
                }
            },
            Err(e) => { Err(Error(ErrorKind::ClientError(format!("Unexpected HTTP error")), (Some(Box::new(e)), Arc::new(backtrace::Backtrace::new())))) }
        }
    }
}

impl Client for HyperClient {
    type Database = RemoteDatabase;

    fn find_database(&self, name: DatabaseName) -> Result<DatabaseState<RemoteDatabase, RemoteDatabaseCreator>> {
        let mut url = self.base_url.clone();
        url.set_path(name.as_ref());
        let res = self.inner
                      .head(url)
                      .send();

        match res {
            Ok(r) => {
                match r.status {
                    StatusCode::Ok => Ok(DatabaseState::Existing(RemoteDatabase { name: name, base_url: self.base_url.clone() })),
                    StatusCode::NotFound => Ok(DatabaseState::Absent(RemoteDatabaseCreator { name: name, base_url: self.base_url.clone() })),
                    _ => Err(Error(ErrorKind::ClientError(format!("Unexpected status: {}", r.status)), (None, Arc::new(backtrace::Backtrace::new()))))
                }
            },
            Err(e) => { Err(Error(ErrorKind::ClientError(format!("Unexpected HTTP error")), (Some(Box::new(e)), Arc::new(backtrace::Backtrace::new())))) }
        }
    }
}
```

Let's write some tests! \o/


```rust
#[test]
fn test_database_lookup() {
    let client = HyperClient::default();
    let res = client.find_database("absent".to_string());
    assert!(res.is_ok());
}

#[test]
fn test_database_absent() {
    let client = HyperClient::default();
    let res = client.find_database("absent".to_string());
    assert!(res.is_ok());
    assert!(res.unwrap().absent())
}

#[test]
fn test_database_create() {
    let client = HyperClient::default();
    let res = client.find_database("to_be_created".to_string())
                    .or_create();
    assert!(res.is_ok());
    assert!(res.unwrap().existing())
}

#[test]
fn test_database_create_and_delete() {
    let client = HyperClient::default();
    let res = client.find_database("to_be_deleted".to_string())
                    .or_create()
                    .and_delete();
    assert!(res.is_ok());
    assert!(res.unwrap().absent())
}

#[test]
fn test_database_get_document() {
    use lazers_traits::SimpleKey;
    use serde_json::Value;

    let client = HyperClient::default();
    let res = client.find_database("empty_test_db".to_string())
                    .or_create();
    assert!(res.is_ok());
    let db = res.unwrap();
    assert!(db.existing());

    if let DatabaseState::Existing(db) = db {
        let key = SimpleKey::from("test".to_owned());
        let doc_res = db.doc::<SimpleKey, Value>(key);
        assert!(doc_res.is_ok());
    } else {
        panic!("database not existing!")
    }
}

#[test]
fn test_database_create_document() {
    use lazers_traits::SimpleKey;
    use serde_json::Value;
    use lazers_traits::DocumentResult;

    let client = HyperClient::default();
    let res = client.find_database("empty_test_db".to_string())
                    .and_delete()
                    .or_create();
    assert!(res.is_ok());
    let db = res.unwrap();
    assert!(db.existing());

    if let DatabaseState::Existing(db) = db {
        let key = SimpleKey::from("test-will-be-created".to_owned());
        let s = "{\"x\": 1.0, \"y\": 2.0}";
        let value: Value = serde_json::from_str(s).unwrap();
        let doc_res = db.doc(key);
        assert!(doc_res.is_ok());

        let del_res = doc_res.delete();
        assert!(del_res.is_ok());

        let set_res = del_res.set(value);

        match set_res {
            Err(e) => {println!("{}", e); panic!()},
            _ => { }
        };

        assert!(set_res.is_ok());
    } else {
        panic!("database not existing!")
    }
}
```

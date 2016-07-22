# lazers-hyper-client

A CouchDB client implemented using hyper.

```rust
extern crate hyper;
extern crate url;
extern crate lazers_traits;

use lazers_traits::Client;
use lazers_traits::DatabaseName;
use lazers_traits::Database;
use lazers_traits::DatabaseState;
use lazers_traits::DatabaseCreator;
use lazers_traits::DatabaseResult;
use lazers_traits::Error;

use hyper::status::StatusCode;

use url::{Url, Host};

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

    fn create(self) -> Result<RemoteDatabase, Error> {
        let mut url = self.base_url.clone();
        url.set_path(self.name.as_ref());
        let client = hyper::client::Client::new();
        let res = client.put(url)
                        .send();

        match res {
            Ok(_) => Ok(RemoteDatabase { name: self.name, base_url: self.base_url }),
            Err(e) => Err(e.to_string())
        }
    }
}

impl Database for RemoteDatabase {
    type Creator = RemoteDatabaseCreator;

    fn delete(self) -> Result<RemoteDatabaseCreator, Error> {
        let mut url = self.base_url.clone();
        url.set_path(self.name.as_ref());
        let client = hyper::client::Client::new();
        let res = client.delete(url)
                        .send();

        match res {
            Ok(_) => Ok(RemoteDatabaseCreator { name: self.name, base_url: self.base_url }),
            Err(e) => Err(e.to_string())
        }
    }
}

impl Client for HyperClient {
    type Database = RemoteDatabase;

    fn find_database(&self, name: DatabaseName) -> Result<DatabaseState<RemoteDatabase, RemoteDatabaseCreator>, Error> {
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
                    _ => Err(format!("unexpected status: {}", r.status))
                }
            },
            Err(e) => { Err(e.to_string()) }
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
    let res = client.find_database("to_be_created".to_string())
                    .or_create()
                    .and_delete();
    assert!(res.is_ok());
    assert!(res.unwrap().absent())
}
```

# lazers-hyper-client

A CouchDB client implemented using hyper.

This is currently a draft implementation that suffers from a few problems,
mainly that generating the errors hooking into lazers-traits a bit noisy.

This crate itself holds no logic outside of HTTP handling, the description
of
all workflows is in lazers-traits.


```rust
extern crate hyper;
extern crate url;
extern crate lazers_traits;
extern crate serde;
extern crate serde_json;
extern crate mime;
extern crate backtrace;
extern crate futures;

mod types;
use types::document_created::DocumentCreated;
use types::error;

use lazers_traits::prelude::*;

use serde_json::de::from_reader;
use serde_json::ser::to_string;

use hyper::header::ETag;
use hyper::header::ContentType;

use hyper::client::IntoUrl;

use hyper::status::StatusCode;
use std::sync::Arc;

use url::{Url, ParseError};

use futures::BoxFuture;
use futures::Future;
use futures::finished;
use futures::failed;
use futures::done;

pub struct HyperClient {
    inner: hyper::client::Client,
    base_url: Url,
}

impl HyperClient {
    pub fn new<T: IntoUrl>(url: T) -> std::result::Result<HyperClient, ParseError> {
        Ok(HyperClient {
            inner: hyper::client::Client::new(),
            base_url: try!(url.into_url()),
        })
    }
}

impl Default for HyperClient {
    fn default() -> HyperClient {
        HyperClient {
            inner: hyper::client::Client::new(),
            base_url: Url::parse("http://localhost:5984").expect("this is a valid URL"),
        }
    }
}

#[derive(Clone)]
pub struct RemoteDatabaseCreator {
    name: DatabaseName,
    base_url: Url,
}

#[derive(Clone)]
pub struct RemoteDatabase {
    name: DatabaseName,
    base_url: Url,
}

impl DatabaseCreator for RemoteDatabaseCreator {
    type D = RemoteDatabase;

    fn create(self) -> BoxFuture<RemoteDatabase, Error> {
        let mut url = self.base_url.clone();
        url.set_path(self.name.as_ref());
        let client = hyper::client::Client::new();
        let res = client.put(url)
            .send();

        let res2 = res.chain_err(|| self.name.clone());

        match res2 {
            Ok(_) => {
                finished(RemoteDatabase {
                    name: self.name,
                    base_url: self.base_url,
                }).boxed()
            }
            Err(e) => failed(e).boxed(),
        }
    }
}

impl Database for RemoteDatabase {
    type Creator = RemoteDatabaseCreator;

    fn destroy(self) -> BoxFuture<RemoteDatabaseCreator, Error> {
        let mut url = self.base_url.clone();
        url.set_path(self.name.as_ref());
        let client = hyper::client::Client::new();
        let res = client.delete(url)
            .send();

        let res2 = res.chain_err(|| self.name.clone());

        match res2 {
            Ok(_) => {
                finished(RemoteDatabaseCreator {
                    name: self.name,
                    base_url: self.base_url,
                }).boxed()
            }
            Err(e) => failed(e).boxed(),
        }
    }

    fn doc<K: Key + 'static, D: Document + 'static>(&self,
                                    key: K)
                                    -> BoxFuture<DatabaseEntry<K, D, RemoteDatabase>, Error> {
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
                        let key_with_rev = <K as Key>::from_id_and_rev(key.id().to_owned(),
                                                                       Some(rev.tag().to_owned()));
                        let doc = from_reader(r).unwrap();
                        finished(DatabaseEntry::present(key_with_rev, doc, self.clone())).boxed()
                    }
                    StatusCode::NotFound => finished(DatabaseEntry::absent(key, self.clone())).boxed(),
                    _ => {
                        failed(error(format!("Unexpected status: {}", r.status),
                              backtrace::Backtrace::new())).boxed()
                    }
                }
            }
            Err(e) => {
                failed(hyper_error(format!("Unexpected HTTP error"),
                            e,
                            backtrace::Backtrace::new())).boxed()
            }
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
            Err(e) => {
                return Err(hyper_error(format!("Unexpected HTTP error"),
                                   e,
                                   backtrace::Backtrace::new()))
            }
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
                            error::Error::Conflict(reason) => {
                                conflict(format!("Document update conflict: {}", reason),
                                         backtrace::Backtrace::new())
                            }
                            error::Error::BadRequest(reason) => {
                                Err(error(format!("Bad request: {}", reason),
                                      backtrace::Backtrace::new()))
                            }
                        }
                    }
                    _ => {
                        Err(error(format!("Unexpected status: {}", r.status),
                              backtrace::Backtrace::new()))
                    }
                }
            }
            Err(e) => {
                Err(hyper_error(format!("Unexpected HTTP error"),
                            e,
                            backtrace::Backtrace::new()))
            }
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
                    StatusCode::Ok => Ok(()),
                    _ => {
                        Err(error(format!("Unexpected status: {}", r.status),
                              backtrace::Backtrace::new()))
                    }
                }
            }
            Err(e) => {
                Err(hyper_error(format!("Unexpected HTTP error"),
                            e,
                            backtrace::Backtrace::new()))
            }
        }
    }
}

impl Client for HyperClient {
    type Database = RemoteDatabase;

    fn find_database(&self,
                     name: DatabaseName)
                     -> BoxFuture<DatabaseState<RemoteDatabase, RemoteDatabaseCreator>, Error> {
        let mut url = self.base_url.clone();
        url.set_path(name.as_ref());
        let res = self.inner
            .head(url)
            .send();

        match res {
            Ok(r) => {
                match r.status {
                    StatusCode::Ok => {
                        finished(DatabaseState::Existing(RemoteDatabase {
                            name: name,
                            base_url: self.base_url.clone(),
                        })).boxed()
                    }
                    StatusCode::NotFound => {
                        finished(DatabaseState::Absent(RemoteDatabaseCreator {
                            name: name,
                            base_url: self.base_url.clone(),
                        })).boxed()
                    }
                    _ => {
                        failed(error(format!("Unexpected status: {}", r.status),
                                     backtrace::Backtrace::new())).boxed()
                    }
                }
            }
            Err(e) => {
                failed(hyper_error(format!("Unexpected HTTP error"),
                            e,
                            backtrace::Backtrace::new())).boxed()
            }
        }
    }
}

fn hyper_error<E: std::error::Error + Send + 'static>(message: String,
                                                         error: E,
                                                         backtrace: backtrace::Backtrace)
                                                         -> Error {
    Error(ErrorKind::ClientError(message),
              (Some(Box::new(error)), Arc::new(backtrace)))
}

fn error(message: String, backtrace: backtrace::Backtrace) -> Error {
    Error(ErrorKind::ClientError(message), (None, Arc::new(backtrace)))
}

fn conflict<T>(message: String, backtrace: backtrace::Backtrace) -> Result<T> {
    Err(Error(ErrorKind::ClientError(message), (None, Arc::new(backtrace))))
}
```

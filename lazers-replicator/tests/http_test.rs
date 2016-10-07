extern crate lazers_replicator;
extern crate lazers_hyper_client;
extern crate lazers_traits;
extern crate futures;

use futures::Future;

use lazers_traits::prelude::*;
use lazers_hyper_client::*;
use lazers_replicator::*;


fn ensure_database_present(database: &str) {
    let client = HyperClient::default();
    client.find_database(database.to_string())
                         .or_create().wait().unwrap();
    wait(1000)
}

fn ensure_database_absent(database: &str) {
    let client = HyperClient::default();
    client.find_database(database.to_string())
                       .and_delete().wait().unwrap();
}

fn wait(millis: u64) {
  use std::{thread, time};

  let duration = time::Duration::from_millis(millis);

  thread::sleep(duration);
}

#[test]
fn test_present_databases() {
  let from = HyperClient::default();
  let to = HyperClient::default();
  ensure_database_present("present-source");
  ensure_database_present("present-target");

  let from_db = "present-source".to_string();
  let to_db = "present-target".to_string();

  let replicator = Replicator::new(from, to, from_db, to_db, false);

  let res = replicator.verify_peers().wait();

  assert!(res.is_ok())
}

#[test]
fn test_absent_from_database() {
  let from = HyperClient::default();
  let to = HyperClient::default();
  ensure_database_absent("absent-source");
  ensure_database_absent("absent-target");

  let from_db = "absent-source".to_string();
  let to_db = "absent-target".to_string();

  let replicator = Replicator::new(from, to, from_db, to_db, false);

  let res = replicator.verify_peers().wait();

  assert!(res.is_err())
}

#[test]
fn test_absent_target_database_without_create() {
  let from = HyperClient::default();
  let to = HyperClient::default();
  ensure_database_present("present-source");
  ensure_database_absent("absent-target");

  let from_db = "present-source".to_string();
  let to_db = "absent-target".to_string();

  let replicator = Replicator::new(from, to, from_db, to_db, false);

  let res = replicator.verify_peers().wait();

  assert!(res.is_err())
}

#[test]
fn test_absent_target_database_with_create() {
  let from = HyperClient::default();
  let to = HyperClient::default();
  ensure_database_present("present-source");
  ensure_database_absent("absent-target");

  let from_db = "present-source".to_string();
  let to_db = "absent-target".to_string();

  let replicator = Replicator::new(from, to, from_db, to_db, true);

  let res = replicator.setup_peers(true).wait();

  println!("{:?}", res);

  assert!(res.is_ok())
}

#[test]
fn test_absent_source_database_with_create() {
  let from = HyperClient::default();
  let to = HyperClient::default();
  ensure_database_absent("absent-source");
  ensure_database_absent("absent-target");

  let from_db = "absent-source".to_string();
  let to_db = "absent-target".to_string();

  let replicator = Replicator::new(from, to, from_db, to_db, true);

  let res = replicator.setup_peers(true).wait();

  assert!(res.is_err())
}
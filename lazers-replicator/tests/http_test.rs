extern crate lazers_replicator;
extern crate lazers_hyper_client;
extern crate lazers_traits;
extern crate futures;

use futures::Future;
use futures::done;

use lazers_traits::prelude::*;
use lazers_hyper_client::*;
use lazers_replicator::*;


fn ensure_database_present(database: &str) {
    let client = HyperClient::default();
    client.find_database(database.to_string())
                         .or_create().wait().unwrap();
}

fn ensure_database_absent(database: &str) {
    let client = HyperClient::default();
    client.find_database(database.to_string())
                       .and_delete().wait().unwrap();
}

#[test]
fn test_present_database() {
  let from = HyperClient::default();
  let to = HyperClient::default();
  ensure_database_present("from");
  ensure_database_present("to");

  let from_db = "from".to_string();
  let to_db = "to".to_string();

  let replicator = Replicator::new(from, to, from_db, to_db, false);

  let res = replicator.verify_peers().wait();

  assert!(res.is_ok())
}

#[test]
fn test_absent_database() {
  let from = HyperClient::default();
  let to = HyperClient::default();
  ensure_database_present("from");
  ensure_database_absent("to");

  let from_db = "from".to_string();
  let to_db = "to".to_string();

  let replicator = Replicator::new(from, to, from_db, to_db, false);

  let res = replicator.verify_peers().wait();

  assert!(res.is_err())
}
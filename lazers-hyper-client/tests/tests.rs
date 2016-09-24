extern crate lazers_hyper_client;
extern crate lazers_traits;
extern crate serde_json;

use lazers_hyper_client::*;
use lazers_traits::prelude::*;

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
        
        let get_res = set_res.get();
        assert!(get_res.is_ok());
    } else {
        panic!("database not existing!")
    }
}

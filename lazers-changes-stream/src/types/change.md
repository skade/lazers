```rust
use serde;
use serde::de::Deserialize;
use std::marker::PhantomData;
use super::revision::Revision;

#[derive(Debug)]
pub struct Change<T: Deserialize> {
    pub seq: i64,
    id: String,
    changes: Vec<Revision>,
    doc: Option<T>,
    deleted: bool
}

impl<T: Deserialize> Deserialize for Change<T> {
    fn deserialize<D>(deserializer: &mut D) -> Result<Change<T>, D::Error>
        where D: serde::Deserializer,
    {
        deserializer.deserialize(ChangeVisitor { phantom: PhantomData } )
    }
}

enum ChangeField {
    Seq,
    Id,
    Changes,
    Doc,
    Deleted
}

impl serde::Deserialize for ChangeField {
    fn deserialize<D>(deserializer: &mut D) -> Result<ChangeField, D::Error>
        where D: serde::de::Deserializer
    {
        struct ChangeFieldVisitor;

        impl serde::de::Visitor for ChangeFieldVisitor {
            type Value = ChangeField;

            fn visit_str<E>(&mut self, value: &str) -> Result<ChangeField, E>
                where E: serde::de::Error
            {
                match value {
                    "seq" => Ok(ChangeField::Seq),
                    "id" => Ok(ChangeField::Id),
                    "changes" => Ok(ChangeField::Changes),
                    "doc" => Ok(ChangeField::Doc),
                    "deleted" => Ok(ChangeField::Deleted),
                    _ => Err(serde::de::Error::unknown_field(format!("expected seq, id, changes or deleted field, got: {}", value).as_ref())),
                }
            }
        }

        deserializer.deserialize(ChangeFieldVisitor)
    }
}

pub struct ChangeVisitor<T: Deserialize> {
    phantom: PhantomData<T>
}

impl<T: Deserialize> serde::de::Visitor for ChangeVisitor<T> {
    type Value = Change<T>;

    fn visit_map<V>(&mut self, mut visitor: V) -> Result<Change<T>, V::Error>
        where V: serde::de::MapVisitor
    {
        let mut seq = None;
        let mut id = None;
        let mut changes = None;
        let mut doc = None;
        let mut deleted = None;

        loop {
            match try!(visitor.visit_key()) {
                Some(ChangeField::Seq) => { seq = Some(try!(visitor.visit_value())); }
                Some(ChangeField::Id) => { id = Some(try!(visitor.visit_value())); }
                Some(ChangeField::Changes) => { changes = Some(try!(visitor.visit_value())); }
                Some(ChangeField::Doc) => { doc = Some(try!(visitor.visit_value())); }
                Some(ChangeField::Deleted) => { deleted = Some(try!(visitor.visit_value())); }
                None => { break; }
            }
        }

        let seq = match seq {
            Some(seq) => seq,
            None => try!(visitor.missing_field("seq")),
        };

        let id = match id {
            Some(id) => id,
            None => try!(visitor.missing_field("id")),
        };

        let changes = match changes {
            Some(changes) => changes,
            None => try!(visitor.missing_field("changes")),
        };

        let doc = match doc {
            Some(doc) => doc,
            None => try!(visitor.missing_field("doc")),
        };

        let deleted = match deleted {
            Some(deleted) => deleted,
            None => false,
        };

        try!(visitor.end());

        Ok(Change { seq: seq, id: id, changes: changes, doc: doc, deleted: deleted})
    }
}
```

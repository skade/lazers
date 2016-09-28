```rust
use serde;
use serde::de::Deserialize;

#[derive(Debug)]
pub struct DocumentCreated {
    pub ok: bool,
    pub id: String,
    pub rev: String,
}

enum DocumentCreatedField {
    Ok,
    Id,
    Rev,
}

impl Deserialize for DocumentCreated {
    fn deserialize<D>(deserializer: &mut D) -> Result<DocumentCreated, D::Error>
        where D: serde::Deserializer
    {
        deserializer.deserialize(DocumentCreatedVisitor)
    }
}

impl serde::Deserialize for DocumentCreatedField {
    fn deserialize<D>(deserializer: &mut D) -> Result<DocumentCreatedField, D::Error>
        where D: serde::de::Deserializer
    {
        struct DocumentCreatedFieldVisitor;

        impl serde::de::Visitor for DocumentCreatedFieldVisitor {
            type Value = DocumentCreatedField;

            fn visit_str<E>(&mut self, value: &str) -> Result<DocumentCreatedField, E>
                where E: serde::de::Error
            {
                match value {
                    "ok" => Ok(DocumentCreatedField::Ok),
                    "id" => Ok(DocumentCreatedField::Id),
                    "rev" => Ok(DocumentCreatedField::Rev),
                    _ => {
                        Err(serde::de::Error::unknown_field(format!("expected ok, id or rev \
                                                                     field, got: {}",
                                                                    value)
                            .as_ref()))
                    }
                }
            }
        }

        deserializer.deserialize(DocumentCreatedFieldVisitor)
    }
}

struct DocumentCreatedVisitor;

impl serde::de::Visitor for DocumentCreatedVisitor {
    type Value = DocumentCreated;

    fn visit_map<V>(&mut self, mut visitor: V) -> Result<DocumentCreated, V::Error>
        where V: serde::de::MapVisitor
    {
        let mut ok = None;
        let mut id = None;
        let mut rev = None;

        loop {
            match try!(visitor.visit_key()) {
                Some(DocumentCreatedField::Ok) => {
                    ok = Some(try!(visitor.visit_value()));
                }
                Some(DocumentCreatedField::Id) => {
                    id = Some(try!(visitor.visit_value()));
                }
                Some(DocumentCreatedField::Rev) => {
                    rev = Some(try!(visitor.visit_value()));
                }
                None => {
                    break;
                }
            }
        }

        let ok = match ok {
            Some(ok) => ok,
            None => try!(visitor.missing_field("ok")),
        };

        let id = match id {
            Some(id) => id,
            None => try!(visitor.missing_field("seq")),
        };

        let rev = match rev {
            Some(rev) => rev,
            None => try!(visitor.missing_field("rev")),
        };

        try!(visitor.end());

        Ok(DocumentCreated {
            ok: ok,
            id: id,
            rev: rev,
        })
    }
}

#[cfg(test)]
mod tests {
    use serde_json as json;
    use super::DocumentCreated;

    #[test]
    fn parses_document_created() {
        json::from_str::<DocumentCreated>("{\"ok\": true, \"id\": \"213123\", \"rev\": \
                                           \"1-cd90201763f897aa0178b7ff05eb80cb\"}")
            .unwrap();
    }
}
```

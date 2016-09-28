```rust
use serde;
use serde::de::Deserialize;

#[derive(Debug)]
pub struct LastSeq {
    last_seq: i64,
}

impl Deserialize for LastSeq {
    fn deserialize<D>(deserializer: &mut D) -> Result<LastSeq, D::Error>
        where D: serde::Deserializer
    {
        deserializer.deserialize(LastSeqVisitor)
    }
}

enum LastSeqField {
    LastSeq,
}

impl serde::Deserialize for LastSeqField {
    fn deserialize<D>(deserializer: &mut D) -> Result<LastSeqField, D::Error>
        where D: serde::de::Deserializer
    {
        struct LastSeqFieldVisitor;

        impl serde::de::Visitor for LastSeqFieldVisitor {
            type Value = LastSeqField;

            fn visit_str<E>(&mut self, value: &str) -> Result<LastSeqField, E>
                where E: serde::de::Error
            {
                match value {
                    "last_seq" => Ok(LastSeqField::LastSeq),
                    _ => {
                        Err(serde::de::Error::unknown_field(format!("expected last_seq field, \
                                                                     got: {}",
                                                                    value)
                            .as_ref()))
                    }
                }
            }
        }

        deserializer.deserialize(LastSeqFieldVisitor)
    }
}

pub struct LastSeqVisitor;

impl serde::de::Visitor for LastSeqVisitor {
    type Value = LastSeq;

    fn visit_map<V>(&mut self, mut visitor: V) -> Result<LastSeq, V::Error>
        where V: serde::de::MapVisitor
    {
        let mut last_seq = None;

        loop {
            match try!(visitor.visit_key()) {
                Some(LastSeqField::LastSeq) => {
                    last_seq = Some(try!(visitor.visit_value()));
                }
                None => {
                    break;
                }
            }
        }

        let last_seq = match last_seq {
            Some(last_seq) => last_seq,
            None => try!(visitor.missing_field("last_seq")),
        };

        try!(visitor.end());

        Ok(LastSeq { last_seq: last_seq })
    }
}

#[cfg(test)]
mod tests {
    use serde_json as json;
    use super::LastSeq;

    #[test]
    fn parses_last_seq_line() {
        json::from_str::<LastSeq>("{\"last_seq\":3}").unwrap();
    }
}
```

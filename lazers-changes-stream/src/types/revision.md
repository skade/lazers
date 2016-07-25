```rust
use serde;
use serde::de::Deserialize;

#[derive(Debug)]
pub struct Revision {
    rev: String,
}

impl Deserialize for Revision {
    fn deserialize<D>(deserializer: &mut D) -> Result<Revision, D::Error>
        where D: serde::de::Deserializer,
    {
        deserializer.deserialize(RevisionVisitor)
    }
}

enum RevisionField {
    Rev,
}

impl serde::Deserialize for RevisionField {
    fn deserialize<D>(deserializer: &mut D) -> Result<RevisionField, D::Error>
        where D: serde::de::Deserializer
    {
        struct RevisionFieldVisitor;

        impl serde::de::Visitor for RevisionFieldVisitor {
            type Value = RevisionField;

            fn visit_str<E>(&mut self, value: &str) -> Result<RevisionField, E>
                where E: serde::de::Error
            {
                match value {
                    "rev" => Ok(RevisionField::Rev),
                    _ => Err(serde::de::Error::missing_field("expected rev field"))
                }
            }
        }

        deserializer.deserialize(RevisionFieldVisitor)
    }
}

struct RevisionVisitor;

impl serde::de::Visitor for RevisionVisitor {
    type Value = Revision;

    fn visit_map<V>(&mut self, mut visitor: V) -> Result<Revision, V::Error>
        where V: serde::de::MapVisitor
    {
        let mut rev = None;

        loop {
            match try!(visitor.visit_key()) {
                Some(RevisionField::Rev) => { rev = Some(try!(visitor.visit_value())); }
                None => { break; }
            }
        }

        let rev = match rev {
            Some(rev) => rev,
            None => try!(visitor.missing_field("rev")),
        };

        try!(visitor.end());

        Ok(Revision { rev: rev })
    }
}
```

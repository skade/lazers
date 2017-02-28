```rust
use serde::de::Deserialize;
use super::revision::Revision;

#[derive(Debug,Deserialize)]
pub struct Change<T: Deserialize> {
    pub seq: i64,
    id: String,
    changes: Vec<Revision>,
    doc: Option<T>,
    deleted: bool,
}
```

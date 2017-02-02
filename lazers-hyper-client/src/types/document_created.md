```rust
#[derive(Debug, Deserialize)]
pub struct DocumentCreated {
    pub ok: bool,
    pub id: String,
    pub rev: String,
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

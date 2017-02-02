```rust
#[derive(Debug, Deserialize)]
pub struct LastSeq {
    last_seq: i64,
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

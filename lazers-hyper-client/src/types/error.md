```rust
use serde;
use std::fmt;

#[derive(Debug, Deserialize)]
#[serde(tag = "error", content = "reason")]
pub enum Error {
    #[serde(rename = "conflict")]
    Conflict(String),
    #[serde(rename = "bad_request")]
    BadRequest(String),
}

#[cfg(test)]
mod tests {
    use serde_json as json;
    use super::Error;

    #[test]
    fn parses_error() {
        json::from_str::<Error>("{\"error\":\"conflict\",\"reason\":\"Document update \
                                 conflict.\"}")
            .unwrap();
        json::from_str::<Error>("{\"error\":\"bad_request\",\"reason\":\"Referer header \
                                 required.\"}")
            .unwrap();

    }
}
```

```rust
use serde;
use serde::de::Deserialize;
use std::fmt;

#[derive(Debug)]
pub enum Error {
    Conflict(String),
    BadRequest(String),
}


enum ErrorField {
    Error,
    Reason,
}

impl Deserialize for Error {
    fn deserialize<D>(deserializer: D) -> Result<Error, D::Error>
        where D: serde::Deserializer
    {
        deserializer.deserialize(ErrorVisitor)
    }
}

impl serde::Deserialize for ErrorField {
    fn deserialize<D>(deserializer: D) -> Result<ErrorField, D::Error>
        where D: serde::de::Deserializer
    {
        struct ErrorFieldVisitor;

        impl serde::de::Visitor for ErrorFieldVisitor {
            type Value = ErrorField;

            fn visit_str<E>(self, value: &str) -> Result<ErrorField, E>
                where E: serde::de::Error
            {
                match value {
                    "error" => Ok(ErrorField::Error),
                    "reason" => Ok(ErrorField::Reason),
                    _ => {
                        Err(serde::de::Error::unknown_field(format!("expected error or reason \
                                                                     field, got: {}",
                                                                    value)
                            .as_ref()))
                    }
                }
            }

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                write!(formatter, "a map with an `error` and a `reason` field")
            }
        }

        deserializer.deserialize(ErrorFieldVisitor)
    }
}

struct ErrorVisitor;

impl serde::de::Visitor for ErrorVisitor {
    type Value = Error;

    fn visit_map<V>(self, mut visitor: V) -> Result<Error, V::Error>
        where V: serde::de::MapVisitor
    {
        let mut error: Option<String> = None;
        let mut reason = None;

        loop {
            match try!(visitor.visit_key()) {
                Some(ErrorField::Error) => {
                    error = Some(try!(visitor.visit_value()));
                }
                Some(ErrorField::Reason) => {
                    reason = Some(try!(visitor.visit_value()));
                }
                None => {
                    break;
                }
            }
        }

        let error = match error {
            Some(error) => error,
            None => try!(visitor.missing_field("error")),
        };

        let reason = match reason {
            Some(reason) => reason,
            None => try!(visitor.missing_field("reason")),
        };

        try!(visitor.end());

        match error.as_ref() {
            "conflict" => Ok(Error::Conflict(reason)),
            "bad_request" => Ok(Error::BadRequest(reason)),
            _ => {
                Err(serde::de::Error::invalid_value(format!("Unknown error type: {}", error)
                    .as_ref()))
            }
        }
    }

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        write!(formatter, "a map with an `error` and a `reason` field")
    }
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

```rust
use serde_json as json;
use serde::de::Deserialize;
use std::convert::AsRef;
use super::change::{Change};
use super::last_seq::{LastSeq};

#[derive(Debug)]
pub enum ChangesLines<T: Deserialize> {
    Change(Change<T>),
    LastSeq(LastSeq)
}

impl<T: Deserialize> ChangesLines<T> {
    pub fn parse<'a, Line: AsRef<str>>(line: Line) -> Result<ChangesLines<T>, json::error::Error>{
        json::from_str::<Change<T>>(line.as_ref())
             .map(|c| {
            ChangesLines::Change(c)
        }).or_else(|e| {
            json::from_str::<LastSeq>(line.as_ref())
                 .map(|seq| {
                ChangesLines::LastSeq(seq)
            }).or(Err(e))
        })
    }

    pub fn change(&self) -> bool {
        match *self {
            ChangesLines::Change(_) => true,
            _ => false
        }
    }

    pub fn to_change(self) -> Option<Change<T>> {
        match self {
            ChangesLines::Change(c) => Some(c),
            _ => None
        }
    }

    pub fn to_last_seq(self) -> Option<LastSeq> {
        match self {
            ChangesLines::LastSeq(l) => Some(l),
            _ => None
        }
    }
}
```

# ChangesStream structure

This library only handles the streaming changes stream, as specified in
[TODO:link]().

There are two kinds of forms this steam can take: The full stream and just
changes. This module implements both of them.

It works by consuming any input stream that implements `Read`, so it can be
used on top of HTTP requests or just File input.

## Imports

The module abstracts of raw streams that implement `Read`, but works linewise.
This set of traits is needed to make that convenient.

```rust
use std::io::{BufRead,BufReader,Read,Lines};
```

The module abstracts over types as parsing information. They are not stored as
data itself, so we need PhantomData fields here.

```rust
use std::marker::PhantomData;
```

We decide to expect all types to be deserializable through Serde. We're not
fully sure if simpler json libs could be allowed for the payload data.

```rust
use serde::de::Deserialize;
```

We use both out own `Change` and `ChangesLines` types. `Change` is any document
change, `ChangesLines` holds _all_ lines of the changes stream.

```rust
use types::change::{Change};
use types::changes_lines::{ChangesLines};
```

## Definitions

### `ChangesStream`

Provides reading of the CouchDB wire protocol from any stream that implements
`Read`.

It is generic over the kinds of documents included in the changes stream, as
long as they implement "Deserialize".

```rust
/// A handle on a changes stream. Provides reading of events from a source of /// type and holds type information about the documents expected.
pub struct ChangesStream<Source: Read, Documents: Deserialize> {
    source: Lines<BufReader<Source>>,
    documents: PhantomData<Documents>
}
```
### `Full`

The `Full` interface gives raw access to all events happening in the changes
stream. This includes `LastSeq` documents, that are intended for internal
tracking.

```rust
/// Wrapper for a ChangesStream with full access.
pub struct Full<Source: Read, Documents: Deserialize> {
    stream: ChangesStream<Source, Documents>
}
```

### `Changes`

`Changes` only includes actual document changes and no protocol information.
Most notably, it filters out `LastSeq` messages.

```rust
/// Wrapper for a ChangesStream only returning `Change` documents.
pub struct Changes<Source: Read, Documents: Deserialize> {
    stream: Full<Source, Documents>
}
```

The implementation of the `ChangesStream` is intended as a proxy only, it is
constructed and then the user selects if the full stream or only changes are
wanted. Folding `Full` and `ChangesStream` into one was considered, but not
used as this provides a symmetric interface, even though `Changes` internally
relies on full.

```rust
impl<Source: Read, Documents: Deserialize> ChangesStream<Source,Documents> {
    /// Construct a new changes stream out of every `read` source.
    /// `Documents` needs to be any deserializable type.
    pub fn new(source: Source) -> ChangesStream<Source,Documents> {
        ChangesStream { source: BufReader::new(source).lines(), documents: PhantomData }
    }

    /// Get an iterator to iterate over the full changes stream, including
    /// control events. 
    pub fn full(self) -> Full<Source, Documents> {
        Full { stream: self }
    }

    /// Get an iterator to just read the changes out of the stream, without
    /// control events.
    pub fn changes(self) -> Changes<Source, Documents> {
        Changes { stream: self.full() }
    }
}
```

### Iterator implementations

The iterator implementations are rather straight forward, with `Full` delegating
to `ChangesLines` for parsing and unwrapping its results.

Note that this implementation silently eats errors (including connection
errors!) currently.

```rust
impl<Source: Read, Documents: Deserialize> Iterator for Full<Source, Documents> {
    type Item = ChangesLines<Documents>;

    #[inline]
    fn next(&mut self) -> Option<ChangesLines<Documents>> {
        if let Some(elem) = self.stream.source.next() {
            elem.ok().iter()
                .filter_map(|line| {
                    ChangesLines::parse(line).ok()
                }).nth(0)
        } else {
            None
        }

    }
}

impl<Source: Read, Documents: Deserialize> Iterator for Changes<Source, Documents> {
    type Item = Change<Documents>;

    #[inline]
    fn next(&mut self) -> Option<Change<Documents>> {
        if let Some(next) = self.stream.next() {
            next.to_change()
        } else {
            None
        }
    }
}
```

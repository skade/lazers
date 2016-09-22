# lazers-changes-stream

lazers-changes-stream is an implementation of the streaming couchdb changes protocol. It is standalone, and independent of the stream kind. This means lazers-changes-stream doesn't require HTTP interaction. lazers-changes-stream works with both in `include_docs=true` mode and without.

lazers-changes-steam uses serde for serialization and deserialization of data. It is generic over the output data, so it deserializes into any user-requested data format, including simple JSON.

## Dependencies

`serde` and `serde_json` are used for deserialisation.

```rust
extern crate serde;
extern crate serde_json;
```

## Exposed modules

### `types`

This module defines all types used for reading the CouchDB changes protocol. 
Examples for this are the Change type that wraps a changed document and or the `LastSeq` type, which describes the last sequence number read.

```rust
pub mod types;
```

### `changes_stream`

The general interface into the library, most notably the ChangesStream buffer implementation.

```rust
pub mod changes_stream;
```

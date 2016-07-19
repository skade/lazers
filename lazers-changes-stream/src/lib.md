# lazers-changes-stream

lazers-changes-stream is an implementation of the streaming couchdb sync protocol. It is standalone, and independent of the stream kind. This means lazers-changes-stream doesn't require HTTP interaction.

lazers-changes-steam uses serde for serialization and deserialization of data. It is generic over the output data, so it deserializes into any user-requested data format, including simple JSON.

## Dependencies

`serde` and `serde_json` are used for deserialisation.

```rust
extern crate serde;
extern crate serde_json;
```

## Exposed modules

### `types`

These are the types the module uses for marking things like Changes and the shared structural types that the CouchDB replication protocol brings with it (like the `last_seq` marker).

```rust
pub mod types;
```

### `changes_stream`

The general interface into the library, most notably the ChangesStream buffer implementation.

```rust
pub mod changes_stream;
```

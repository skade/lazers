# Laze RS

What could become a CouchDB-client in Rust.

Uses literate programming through [https://github.com/pnkfelix/tango](tango) and tries hard to build nice APIs.

[https://en.wiktionary.org/wiki/laze](It is pronouced "Laze RS").

## Project state

This is currently mostly API experiments. I'm searching for help
with many of them.

Contributors welcome!

Find a list of issues [here](https://github.com/skade/lazers/issues).

## Notable specialties

### Use of Tango

Tango is used to provide richer documentation by using literate programming to make writing descriptions the default.

### Documentation-driven development

Features, as experimental as they may be, should never be committed without extensive documentation.

This hasn't been followed through in the past and has been fixed, but there might be spots. These spots shouldn't happen again.

### Use of decorated results

Decorated results allow a form of result chaining that hides error handling until the very last step.

See [TODO] about how decorated results work.

## Current Setup

The project currently consists of the following crates:

* [lazers-traits](lazers-traits): A set of traits without implementations that describe the general interface towards a CouchDB(-like) database. It defines error and result types and interactions for writing and reading documents.

* [lazers-hyper-client](lazers-hyper-client): A client implementing lazers-traits for a remote CouchDB using hyper. Currently also serves as an implementation example for the interface described in lazers-traits.

* [lazers-changes-stream](lazers-changes-stream): An implementation of the CouchDB changes stream protocol. It is generic over any `Read` interface, so it depends on no http client.

* [lazers-replicator](lazers-replicator): A (currently unfinished) implementation of the CouchDB replication protocol. Important features of the base libraries were missing, but it is still around as a placeholder or as a way to start.

* [lazers-liblazers](lazers-liblazers): A C interface to the library. Still figuring a good way to do this. This interface is important, see [Long-Term Goals]

## Notably missing

* A second implementation of the lazers-traits interface for a local database
* A good way to compile the md source documents into a doc site
* A switch to tokio/futures-rs. I think this is the... future, so at some point the whole interface would need to be switched over.

## Long-term goals

* Provide a way to use Laze RS on iOS and Android similar to CouchBase lite.
* Provide bindings towards several programming languages like Ruby/Python

## Constraints

The library itself uses plain Serde for serialisation and deserialisation, this means some boilerplate work is required.

## Credits

Hat tip to https://lobste.rs/u/gsquire for the final name idea.

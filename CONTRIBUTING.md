# Contributing

## Getting started

If you don't have it installed already, install Rust and CouchDB:

* [Rust](https://doc.rust-lang.org/book/getting-started.html)
* [CouchDB](http://couchdb.apache.org/)

Make sure CouchDB is running.

Clone [the repository](https://github.com/skade/lazers).

Navigate the subdirectory (e.g. `lazers-hyper-client`) you want to work in and type:

```bash
$ cargo build
$ cargo test
```

If projects rely on each other (e.g. lazers-hyper-client depends on lazers-traits), the depending project will be built automatically.

That is all, you are ready to go!

## State of the project

The project is currently experimental. Expect API changes.

## Opportunities

The project is currently experimental. Think something is not a good idea? Change it!

## Do I need to know Rust?

No, but you should obviously bring interest. I'll happily walk you through the issues you might experience on the way to your first patch.

## Do I need to know CouchDB?

No. CouchDB is well-documented and currently, most of the work is in implementing the public interface.

## Expected quality of patches

I'm fine with anything that compiles. Even if you break the tests, make sure you submit the patch, we can get started from there.

## Using Tango

I'm rather convinced that I want to stick to tango. Here's how to work with it:

Tango is working in a bidirectional fashion. Whenever you run `cargo build`, it generates Rust sourcecode from the markdown input. You can then edit _any of both files_, as long as you keep the other untouched. On the next `cargo build`, Tango will update the unchanged file. So you can work with whatever file you want.

Just make sure you later commit the right one.

The benefit of tango is the extensive long-form documentation it provides.
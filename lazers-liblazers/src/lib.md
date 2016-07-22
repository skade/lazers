# liblazers

The C interface to laze.rs.


```rust
extern crate lazers_hyper_client;
extern crate lazers_traits;
extern crate libc;

use std::any::Any;
use lazers_traits::Client;
use lazers_hyper_client::HyperClient;

#[repr(C)]
pub struct lzrs_client(Box<Any>);

#[no_mangle]
pub extern "C" fn lzrs_new_hyper_client() -> *mut lzrs_client {
    let boxed = Box::new(lzrs_client(Box::new(HyperClient::default())));
    Box::into_raw(boxed)
}

#[no_mangle]
pub extern "C" fn lzrs_inspect_client(client: *mut lzrs_client) {
    unsafe {
        if (*client).0.is::<HyperClient>() {
            println!("true");
        };
    }
}

#[no_mangle]
pub extern "C" fn lzrs_close(client: *mut lzrs_client) {
    unsafe {
        drop(Box::from_raw(client));
    }
}

#[test]
fn sizes() {
    use std::mem::size_of;
    assert_eq!(size_of::<Box<HyperClient>>(), 8);
    assert_eq!(size_of::<Box<Any>>(), 16);
}
```

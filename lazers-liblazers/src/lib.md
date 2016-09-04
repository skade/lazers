# liblazers

The C interface to laze.rs.


```rust
extern crate lazers_hyper_client;
extern crate lazers_traits;
extern crate libc;

use std::any::Any;
use std::ffi::CStr;
use lazers_traits::Client;
use lazers_hyper_client::HyperClient;

#[repr(C)]
pub struct CClient {
    inspect: unsafe extern "C" fn(*mut std::os::raw::c_void) -> (),
    close: unsafe extern "C" fn(*mut std::os::raw::c_void) -> (),
    get: unsafe extern "C" fn(*mut std::os::raw::c_void, *mut std::os::raw::c_char) -> (),
    client: *mut std::os::raw::c_void
}

pub trait CClientInterface {
    fn inspect(&self);
    fn get(&self, key: &str);
}

impl<C: CClientInterface> From<Box<C>> for CClient {
    fn from(i: Box<C>) -> CClient {
        CClient {
            inspect: inspect::<C>,
            close: close::<C>,
            get: get::<C>,
            client: Box::into_raw(i) as *mut std::os::raw::c_void
        }
    }
}

impl CClientInterface for HyperClient {
    fn inspect(&self) {
        println!("hey from hyperclient");
    }
    fn get(&self, key: &str) {
        println!("hey from get: {}", key);
    }
}

unsafe extern "C" fn inspect<C: CClientInterface>(client: *mut std::os::raw::c_void) {
    let client : Box<C> = Box::from_raw(client as *mut C);
    client.inspect();
    std::mem::forget(client);
}

unsafe extern "C" fn get<C: CClientInterface>(client: *mut std::os::raw::c_void, raw_key: *mut std::os::raw::c_char) {
    let client : Box<C> = Box::from_raw(client as *mut C);
    let key = CStr::from_ptr(raw_key).to_str().unwrap();
    client.get(key);
    std::mem::forget(client);
}

unsafe extern "C" fn close<C: CClientInterface>(client: *mut std::os::raw::c_void) {
    let client = client as *mut CClient;
    Box::from_raw((*client).client as *mut C);
}

#[no_mangle]
pub extern "C" fn lzrs_new_hyper_client() -> *mut CClient {
    let client = Box::new(HyperClient::default());
    let client_structure = Box::new(client.into());
    Box::into_raw(client_structure)
}


#[test]
fn sizes() {
    use std::mem::size_of;
    assert_eq!(size_of::<Box<HyperClient>>(), 8);
    assert_eq!(size_of::<Box<Any>>(), 16);
}
```

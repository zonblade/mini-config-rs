pub mod arc;

use std::collections::HashMap;
use std::sync::{Mutex, Once};

static mut MY_HASHMAP: Option<Mutex<HashMap<&'static str, &'static str>>> = None;
static INIT: Once = Once::new();

pub fn init_memory() {
    let hashmap = HashMap::<&str, &str>::new();
    let mutex = Mutex::new(hashmap);
    unsafe {
        MY_HASHMAP = Some(mutex);
    }
}

pub fn get_value(key: &str) -> Option<&'static str> {
    INIT.call_once(init_memory);
    unsafe {
        MY_HASHMAP
            .as_ref()
            .unwrap()
            .lock()
            .unwrap()
            .get(key)
            .map(|&s| s)
    }
}

pub fn set_value(key: &'static str, value: &'static str) {
    INIT.call_once(init_memory);
    unsafe {
        MY_HASHMAP
            .as_ref()
            .unwrap()
            .lock()
            .unwrap()
            .insert(key, value);
    }
}

#[cfg(feature = "mini-config-derive")]
#[allow(unused_imports)]
#[macro_use]
extern crate mini_config_derive;
#[cfg(feature = "mini-config-derive")]
#[doc(hidden)]
pub use mini_config_derive::*;

#[allow(dead_code)]
fn main() {}
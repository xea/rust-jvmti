extern crate libc;

use std::ptr;
use std::ffi::CStr;
use self::native::MutString;

/// We're exporting native here, so that client code can access native types until this crate
/// provides a stable interface to all JVM functions
pub mod native;

///
/// Turns a C-style string pointer into a String instance. If the string pointer points to NULL,
/// then a "(NULL)" string will be returned.
///
#[allow(dead_code)] // TODO remove this once it's not needed any more
pub fn stringify(input: MutString) -> String {
    let null_string = "(NULL)".to_string();
    let utf8_error = "(UTF8-ERROR)".to_string();

    unsafe {
        if input != ptr::null_mut() {
            match CStr::from_ptr(input).to_str() {
                Ok(string) => string.to_string(),
                Err(_) => utf8_error
            }
        } else {
            null_string
        }
    }
}

use super::native::MutString;
use std::ffi::CStr;
use std::ptr;

///
/// Turns a C-style string pointer into a String instance. If the string pointer points to NULL,
/// then a "(NULL)" string will be returned.
///
pub fn stringify(input: MutString) -> String {
    unsafe {
        if input != ptr::null_mut() {
            match CStr::from_ptr(input).to_str() {
                Ok(string) => string.to_string(),
                Err(_) => "(UTF8-ERROR)".to_string()
            }
        } else {
            "(NULL)".to_string()
        }
    }
}

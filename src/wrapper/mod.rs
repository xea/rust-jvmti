use std::ptr;
use std::ffi::CStr;
use self::native::MutString;

pub mod agent_capabilities;
pub mod class;
pub mod environment;
pub mod error;
pub mod event;
pub mod method;
pub mod native;
pub mod thread;

pub fn stringify(input: MutString) -> String {
    unsafe {
        if input != ptr::null_mut() {
            CStr::from_ptr(input).to_str().unwrap().to_string()
        } else {
            "".to_string()
        }
    }
}

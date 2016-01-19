extern crate libc;

pub mod jvm_agent;
mod jvmti_native;
mod error;

pub use self::error::*;
use self::jvmti_native::jvmti_native::*;

pub type JavaVMPtr = *mut JavaVM;
/// Standard return value type for JVMTI functions
pub type ReturnValue = jint;

/// A type-safe representation of possible errors
///
pub enum NativeError {
    NoError = 0
}

pub fn translate_error(error: NativeError) -> String {
    return error_code(error as u32);
}

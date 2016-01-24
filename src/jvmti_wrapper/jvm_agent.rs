use super::jvmti_native::jvmti_native::*;
use super::jvmti_environment::JvmtiEnvironment;
use super::{JavaVMPtr, NativeError};
use super::error::wrap_error;
use libc::c_void;
use std::ptr;

const JVM_AGENT_VERSION: u32 = 0x00000001;

/// Encapsulates a native JVMTI JVM environment structure for more Rust-idiomatic functionality
pub struct JvmAgent {
    version: u32,
    vm: JavaVMPtr
}

impl JvmAgent {
    /// Create a new JvmAgent instance.
    pub fn new(jvm_ptr: JavaVMPtr) -> JvmAgent {
        JvmAgent {
            version: JVM_AGENT_VERSION,
            vm: jvm_ptr
        }
    }

    // TODO consider if we should hide EnvPtr instead
    pub fn get_environment(&self) -> Result<JvmtiEnvironment, NativeError> {
        unsafe {
            let mut void_ptr: *mut c_void = ptr::null_mut() as *mut c_void;
            let penv_ptr: *mut *mut c_void = &mut void_ptr as *mut *mut c_void;
            let result = (**self.vm).GetEnv.unwrap()(self.vm, penv_ptr, JVMTI_VERSION);

            if result == 0 {
                let env_ptr: *mut jvmtiEnv = *penv_ptr as *mut jvmtiEnv;
                let env = JvmtiEnvironment::new(env_ptr);
                return Result::Ok(env);
            }

            return Result::Err(wrap_error(result as u32));
        }
    }

    /// Return a string representation of this instance
    pub fn to_string(&self) -> String {
        return format!("JVM Agent v{}", self.version);
    }
}

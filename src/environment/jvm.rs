use super::super::native::{JavaVMPtr, JVMTIEnvPtr};
use super::super::native::jvmti_native::JVMTI_VERSION;
use super::super::environment::jvmti::{JVMTI, JVMTIEnvironment};
use super::super::error::{wrap_error, NativeError};
use libc::c_void;
use std::ptr;


///
/// `JVMAgent` represents a binding to the JVM.
///
pub struct JVMAgent {
    vm: JavaVMPtr
}

impl JVMAgent {

    /// Create a new `JVMAgent` instance
    pub fn new(vm: JavaVMPtr) -> JVMAgent {
        JVMAgent { vm: vm }
    }

    /// Return the native JVMTI environment if available (ie. the current thread is attached to it)
    /// otherwise return an error message.
    pub fn get_environment(&self) -> Result<JVMTIEnvironment, NativeError> {
        unsafe {
            let mut void_ptr: *mut c_void = ptr::null_mut() as *mut c_void;
            let penv_ptr: *mut *mut c_void = &mut void_ptr as *mut *mut c_void;
            let result = wrap_error((**self.vm).GetEnv.unwrap()(self.vm, penv_ptr, JVMTI_VERSION) as u32);

            match result {
                NativeError::NoError => {
                    let env_ptr: JVMTIEnvPtr = *penv_ptr as JVMTIEnvPtr;
                    let env = JVMTIEnvironment::new(env_ptr);
                    return Result::Ok(env);
                },
                err @ _ => Result::Err(wrap_error(err as u32))
            }
        }
    }
}

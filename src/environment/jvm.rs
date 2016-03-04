use super::super::native::{JavaVMPtr, JVMTIEnvPtr};
use super::super::native::jvmti_native::JVMTI_VERSION;
use super::super::environment::jvmti::{JVMTI, JVMTIEnvironment};
use super::super::error::{wrap_error, NativeError};
use libc::c_void;
use std::ptr;

pub trait JVMF {
    fn get_environment(&self) -> Result<Box<JVMTI>, NativeError>;
    fn destroy(&self) -> Result<(), NativeError>;
}
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
}

impl JVMF for JVMAgent {

    /// Return the native JVMTI environment if available (ie. the current thread is attached to it)
    /// otherwise return an error message.
    fn get_environment(&self) -> Result<Box<JVMTI>, NativeError> {
        unsafe {
            let mut void_ptr: *mut c_void = ptr::null_mut() as *mut c_void;
            let penv_ptr: *mut *mut c_void = &mut void_ptr as *mut *mut c_void;
            let result = wrap_error((**self.vm).GetEnv.unwrap()(self.vm, penv_ptr, JVMTI_VERSION) as u32);

            match result {
                NativeError::NoError => {
                    let env_ptr: JVMTIEnvPtr = *penv_ptr as JVMTIEnvPtr;
                    let env = JVMTIEnvironment::new(env_ptr);
                    return Result::Ok(Box::new(env));
                },
                err @ _ => Result::Err(wrap_error(err as u32))
            }
        }
    }

    fn destroy(&self) -> Result<(), NativeError> {
        unsafe {
            let error = (**self.vm).DestroyJavaVM.unwrap()(self.vm) as u32;

            if error == 0 {
                Ok(())
            } else {
                Err(wrap_error(error))
            }
        }
    }
}

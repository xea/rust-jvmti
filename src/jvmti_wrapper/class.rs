use super::jvmti_native::jvmti_native::*;
use super::jvmti_environment::JvmtiEnvironment;
use super::EnvPtr;
use super::error::translate_error;
use std::ptr;

/// Represents a Java class
pub struct Class<'a> {
    pub id: jclass,
    env: &'a JvmtiEnvironment
}

impl<'a> Class<'a> {
    pub fn new(env_ptr: &JvmtiEnvironment, class_id: jclass) -> Class {
        Class {
            id: class_id,
            env: env_ptr
        }
    }

    pub fn get_signature(&self) -> String {
        //fn(env: *mut jvmtiEnv, klass: jclass, signature_ptr: *mut *mut ::libc::c_char, generic_ptr: *mut *mut ::libc::c_char)

        match self.env.get_class_signature(self) {
            Ok(signature) => signature,
            Err(error) => "ERROR: Can't get class signature".to_string()
        }
    }

    pub fn is_interface(&self) -> bool {
        match self.env.is_interface(self) {
            Ok(result) => result,
            Err(error) => { println!("Error volt: {}", translate_error(&error)); false}
        }
    }

    pub fn to_string(&self) -> String {
        "CLASS".to_string()
    }
}

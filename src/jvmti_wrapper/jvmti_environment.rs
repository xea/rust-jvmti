extern crate libc;

//use super::jvmti_native::jvmti_native::*;
use super::{EnvPtr, ObjectPtr};
use super::error::{wrap_error, NativeError};
use super::agent_capabilities::AgentCapabilities;
use super::event_callbacks::{EventCallbacks, VMEvent, CALLBACK_TABLE};
use super::jvmti_native::jvmti_native::*;
use super::class::Class;
use super::method::Method;
use std::ffi::CStr;
use std::mem::size_of;
use std::ptr;

pub struct JvmtiEnvironment {
    env: EnvPtr
}

impl JvmtiEnvironment {

    /// Create a new JvmtiEnvironment instance
    pub fn new(env_ptr: EnvPtr) -> JvmtiEnvironment {
        JvmtiEnvironment {
            env: env_ptr
        }
    }

    pub fn add_capabilities(&self, new_caps: AgentCapabilities) -> Result<AgentCapabilities, NativeError> {
        unsafe {
            match wrap_error((**self.env).AddCapabilities.unwrap()(self.env, &new_caps.to_native())) {
                // TODO Implement actual capability re-read
                NativeError::NoError => Ok(new_caps),
                err @ _ => Err(err)
            }
        }
    }

    pub fn set_event_callbacks(&self, callbacks: EventCallbacks) -> Option<NativeError> {
        unsafe {
            CALLBACK_TABLE.vm_object_alloc = callbacks.vm_object_alloc;
            CALLBACK_TABLE.vm_init = callbacks.vm_init;
            CALLBACK_TABLE.method_entry = callbacks.method_entry;

            match wrap_error((**self.env).SetEventCallbacks.unwrap()(self.env, &callbacks.to_native(), size_of::<jvmtiEventCallbacks>() as i32)) {
                NativeError::NoError => None,
                err @ _ => Some(err)
            }
        }
    }

    pub fn set_event_notification_mode(&self, event: VMEvent, mode: bool) -> Option<NativeError> {
        unsafe {
            let mode_i = match mode { true => 1, false => 0 };
            let sptr: ObjectPtr = ptr::null_mut();

            match wrap_error((**self.env).SetEventNotificationMode.unwrap()(self.env, mode_i, event as u32, sptr)) {
                NativeError::NoError => None,
                err @ _ => Some(err)
            }
        }
    }

//  pub GetMethodDeclaringClass: fn(env: *mut jvmtiEnv, method: jmethodID, declaring_class_ptr: *mut jclass)
    pub fn get_method_declaring_class(&self, method: &Method) -> Result<Class, NativeError> {
        unsafe {
            let mut jstruct: Struct__jobject = Struct__jobject { _hacky_hack_workaround: 0 };
            let mut jclass_instance: jclass = &mut jstruct;
            let meta_ptr: *mut jclass = &mut jclass_instance;

            match wrap_error((**self.env).GetMethodDeclaringClass.unwrap()(self.env, method.id(), meta_ptr)) {
                NativeError::NoError => Ok(Class::new(self, *meta_ptr)),
                err @ _ => Err(err)
            }
        }
    }
// pub GetClassSignature: fn(env: *mut jvmtiEnv, klass: jclass, signature_ptr: *mut *mut ::libc::c_char, generic_ptr: *mut *mut ::libc::c_char)
    pub fn get_class_signature(&self, class: &Class) -> Result<String, NativeError> {
        unsafe {
            let mut native_sig: *mut libc::c_char = ptr::null_mut();
            let mut sig: *mut libc::c_char = ptr::null_mut();
            let p1: *mut *mut libc::c_char = &mut sig;
            let p2: *mut *mut libc::c_char = &mut native_sig;

            let result = match wrap_error((**self.env).GetClassSignature.unwrap()(self.env, class.id, p1, p2)) {
                NativeError::NoError => Ok(CStr::from_ptr(sig).to_str().unwrap().to_string()),
                err @ _ => Err(err)
            };

            result
        }
    }

    pub fn is_interface(&self, class: &Class) -> Result<bool, NativeError> {
        unsafe {
            let interface:u8 = 0;

            match wrap_error((**self.env).IsInterface.unwrap()(self.env, class.id, Box::into_raw(Box::new(interface)))) {
                NativeError::NoError => Ok(interface > 0),
                err@_ => Err(err)
            }
        }
    }
}

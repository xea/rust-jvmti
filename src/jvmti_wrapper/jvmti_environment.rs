//use super::jvmti_native::jvmti_native::*;
use super::{EnvPtr, ObjectPtr};
use super::error::{wrap_error, NativeError};
use super::agent_capabilities::AgentCapabilities;
use super::event_callbacks::{EventCallbacks, VMEvent, CALLBACK_TABLE};
use super::jvmti_native::jvmti_native::*;
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

            //match wrap_error((**self.env).SetEventCallbacks.unwrap()(self.env, &callbacks.to_native(), size_of::<jvmtiEventCallbacks>() as i32)) {
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
}

extern crate libc;

//use super::jvmti_native::jvmti_native::*;
use super::{EnvPtr, ObjectPtr};
use super::error::{wrap_error, NativeError};
use super::agent_capabilities::AgentCapabilities;
use super::event_callbacks::{EventCallbacks, VMEvent, CALLBACK_TABLE};
use super::jvmti_native::jvmti_native::*;
use super::class::Class;
use super::method::Method;
use super::method_signature::MethodSignature;
use super::thread::{Thread, ThreadInfo};
use std::ffi::CStr;
use std::mem::size_of;
use std::ptr;
use libc::c_char;

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

    /// Set new capabilities by adding the capabilities whose values are set to true in new_caps.
    /// All previous capabilities are retained.
    /// Some virtual machines may allow a limited set of capabilities to be added in the live phase.
    pub fn add_capabilities(&self, new_caps: AgentCapabilities) -> Result<AgentCapabilities, NativeError> {
        unsafe {
            match wrap_error((**self.env).AddCapabilities.unwrap()(self.env, &new_caps.to_native())) {
                // TODO Implement actual capability re-read
                NativeError::NoError => Ok(new_caps),
                err @ _ => Err(err)
            }
        }
    }

    /// Set the functions to be called for each event. The callbacks are specified by supplying a
    /// replacement function table. The function table is copied--changes to the local copy of the
    /// table have no effect. This is an atomic action, all callbacks are set at once. No events
    /// are sent before this function is called. When an entry is None no event is sent.
    /// An event must be enabled and have a callback in order to be sent--the order in which this
    /// function and set_event_notification_mode are called does not affect the result.
    pub fn set_event_callbacks(&self, callbacks: EventCallbacks) -> Option<NativeError> {
        unsafe {
            CALLBACK_TABLE.vm_object_alloc = callbacks.vm_object_alloc;
            CALLBACK_TABLE.vm_init = callbacks.vm_init;
            CALLBACK_TABLE.method_entry = callbacks.method_entry;
            CALLBACK_TABLE.method_exit = callbacks.method_exit;
            CALLBACK_TABLE.exception = callbacks.exception;
            CALLBACK_TABLE.exception_catch = callbacks.exception_catch;

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

            match wrap_error((**self.env).GetClassSignature.unwrap()(self.env, class.id, p1, p2)) {
                NativeError::NoError => Ok(CStr::from_ptr(sig).to_str().unwrap().to_string()),
                err @ _ => Err(err)
            }
        }
    }

    pub fn get_method_name(&self, method: &Method) -> Result<MethodSignature, NativeError> {
        unsafe {
            let mut method_name = ptr::null_mut();
            let mut method_ptr = &mut method_name;

            let mut signature: *mut c_char = ptr::null_mut();
            let mut signature_ptr = &mut signature;

            let mut generic_sig: *mut c_char = ptr::null_mut();
            let mut generic_sig_ptr = &mut generic_sig;

            match wrap_error((**self.env).GetMethodName.unwrap()(self.env, method.id(), method_ptr, signature_ptr, generic_sig_ptr)) {
                NativeError::NoError => Ok(MethodSignature::new(self.stringify(*method_ptr), self.stringify(*signature_ptr), self.stringify(*generic_sig_ptr))),
                err @ _ => Err(err)
            }
        }
    }

    #[allow(dead_code)]
    pub fn is_interface(&self, class: &Class) -> Result<bool, NativeError> {
        unsafe {
            let interface:u8 = 0;

            match wrap_error((**self.env).IsInterface.unwrap()(self.env, class.id, Box::into_raw(Box::new(interface)))) {
                NativeError::NoError => Ok(interface > 0),
                err@_ => Err(err)
            }
        }
    }

    //pub GetThreadInfo: ::std::option::Option<unsafe extern "C" fn(env: *mut jvmtiEnv, thread: jthread, info_ptr: *mut jvmtiThreadInfo)
    pub fn get_thread_info(&self, thread: jthread) -> Result<ThreadInfo, NativeError> {
        unsafe {
            let mut info = Struct__jvmtiThreadInfo { name: ptr::null_mut(), priority: 0, is_daemon: 0, thread_group: ptr::null_mut(), context_class_loader: ptr::null_mut()};
            let mut info_ptr = &mut info;

            match wrap_error((**self.env).GetThreadInfo.unwrap()(self.env, thread, info_ptr)) {
                NativeError::NoError => Ok(ThreadInfo {
                    name: self.stringify((*info_ptr).name),
                    priority: (*info_ptr).priority as u32,
                    is_daemon: if (*info_ptr).is_daemon > 0 { true } else { false }
                }),
                err@_ => Err(err)
            }
        }
    }

    fn stringify(&self, input: *mut c_char) -> String {
        unsafe {
            if input != ptr::null_mut() {
                CStr::from_ptr(input).to_str().unwrap().to_string()
            } else {
                "".to_string()
            }
        }
    }
}

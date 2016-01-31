use libc::{c_char, c_void};
use std::mem::size_of;
use std::ptr;
use wrapper::native::jvmti_native::*;
use wrapper::native::{JVMTIEnvPtr, JNIEnvPtr, JavaVMPtr, JavaObjectPtr, JavaThread};
use wrapper::agent_capabilities::AgentCapabilities;
use wrapper::event::{EventCallbacks, VMEvent, CALLBACK_TABLE};
use wrapper::method::{MethodId, MethodSignature};
use wrapper::thread::{Thread, ThreadId};
use super::error::{NativeError, wrap_error};
use super::stringify;

pub trait JVMTI {
    /// Set new capabilities by adding the capabilities whose values are set to true in new_caps.
    /// All previous capabilities are retained.
    /// Some virtual machines may allow a limited set of capabilities to be added in the live phase.
    fn add_capabilities(&self, new_capabilities: AgentCapabilities) -> Result<AgentCapabilities, NativeError>;

    /// Set the functions to be called for each event. The callbacks are specified by supplying a
    /// replacement function table. The function table is copied--changes to the local copy of the
    /// table have no effect. This is an atomic action, all callbacks are set at once. No events
    /// are sent before this function is called. When an entry is None no event is sent.
    /// An event must be enabled and have a callback in order to be sent--the order in which this
    /// function and set_event_notification_mode are called does not affect the result.
    fn set_event_callbacks(&self, callbacks: EventCallbacks) -> Option<NativeError>;
    fn set_event_notification_mode(&self, event: VMEvent, mode: bool) -> Option<NativeError>;
    fn get_method_name(&self, method_id: &MethodId) -> Result<MethodSignature, NativeError>;
    fn get_thread_info(&self, thread_id: &JavaThread) -> Result<Thread, NativeError>;
}

pub trait JNI {
}

/// Unified trait for accessing both the JVMTI and the JNI native APIs. This type does not implement
/// any of the provided behaviours itself, only delegates the calls to their respective handlers.
pub struct Environment {
    jvmti: JVMTIEnvironment,
    jni: JNIEnvironment
}

pub struct JNIEnvironment {
    jni: JNIEnvPtr
}

pub struct JVMTIEnvironment {
    jvmti: JVMTIEnvPtr
}

pub struct JVMAgent {
    vm: JavaVMPtr
}

impl JNI for Environment {

}

impl JVMTI for Environment {

    fn add_capabilities(&self, new_capabilities: AgentCapabilities) -> Result<AgentCapabilities, NativeError> {
        self.jvmti.add_capabilities(new_capabilities)
    }

    fn set_event_callbacks(&self, callbacks: EventCallbacks) -> Option<NativeError> {
        self.jvmti.set_event_callbacks(callbacks)
    }

    fn set_event_notification_mode(&self, event: VMEvent, mode: bool) -> Option<NativeError> {
        self.jvmti.set_event_notification_mode(event, mode)
    }

    fn get_method_name(&self, method_id: &MethodId) -> Result<MethodSignature, NativeError> {
        self.jvmti.get_method_name(method_id)
    }

    fn get_thread_info(&self, thread_id: &JavaThread) -> Result<Thread, NativeError> {
        self.jvmti.get_thread_info(thread_id)
    }
}

impl JNI for JNIEnvironment {
}

impl JVMTI for JVMTIEnvironment {

    fn add_capabilities(&self, new_capabilities: AgentCapabilities) -> Result<AgentCapabilities, NativeError> {
        unsafe {
            match wrap_error((**self.jvmti).AddCapabilities.unwrap()(self.jvmti, &new_capabilities.to_native())) {
                // TODO Implement actual capability re-read
                NativeError::NoError => Ok(new_capabilities),
                err @ _ => Err(err)
            }
        }
    }

    fn set_event_callbacks(&self, callbacks: EventCallbacks) -> Option<NativeError> {
        unsafe {
            CALLBACK_TABLE.vm_object_alloc = callbacks.vm_object_alloc;
//            CALLBACK_TABLE.vm_init = callbacks.vm_init;
            CALLBACK_TABLE.method_entry = callbacks.method_entry;
            CALLBACK_TABLE.method_exit = callbacks.method_exit;
            CALLBACK_TABLE.exception = callbacks.exception;
            CALLBACK_TABLE.exception_catch = callbacks.exception_catch;

            match wrap_error((**self.jvmti).SetEventCallbacks.unwrap()(self.jvmti, &callbacks.to_native(), size_of::<jvmtiEventCallbacks>() as i32)) {
                NativeError::NoError => None,
                err @ _ => Some(err)
            }
        }
    }

    fn set_event_notification_mode(&self, event: VMEvent, mode: bool) -> Option<NativeError> {
        unsafe {
            let mode_i = match mode { true => 1, false => 0 };
            let sptr: JavaObjectPtr = ptr::null_mut();

            match wrap_error((**self.jvmti).SetEventNotificationMode.unwrap()(self.jvmti, mode_i, event as u32, sptr)) {
                NativeError::NoError => None,
                err @ _ => Some(err)
            }
        }
    }

    fn get_method_name(&self, method_id: &MethodId) -> Result<MethodSignature, NativeError> {
        unsafe {
            let mut method_name = ptr::null_mut();
            let mut method_ptr = &mut method_name;

            let mut signature: *mut c_char = ptr::null_mut();
            let mut signature_ptr = &mut signature;

            let mut generic_sig: *mut c_char = ptr::null_mut();
            let mut generic_sig_ptr = &mut generic_sig;

            match wrap_error((**self.jvmti).GetMethodName.unwrap()(self.jvmti, method_id.native_id, method_ptr, signature_ptr, generic_sig_ptr)) {
                NativeError::NoError => Ok(MethodSignature::new(stringify(*method_ptr), stringify(*signature_ptr))),
                err @ _ => Err(err)
            }
        }
    }

    fn get_thread_info(&self, thread_id: &JavaThread) -> Result<Thread, NativeError> {
        unsafe {
            let mut info = Struct__jvmtiThreadInfo { name: ptr::null_mut(), priority: 0, is_daemon: 0, thread_group: ptr::null_mut(), context_class_loader: ptr::null_mut()};
            let mut info_ptr = &mut info;

            match wrap_error((**self.jvmti).GetThreadInfo.unwrap()(self.jvmti, *thread_id, info_ptr)) {
                NativeError::NoError => Ok(Thread {
                    id: ThreadId { native_id: *thread_id },
                    name: stringify((*info_ptr).name),
                    priority: (*info_ptr).priority as u32,
                    is_daemon: if (*info_ptr).is_daemon > 0 { true } else { false }
                }),
                err@_ => Err(err)
            }
        }
    }
}

impl Environment {

    pub fn new(jvmti: JVMTIEnvironment, jni: JNIEnvironment) -> Environment {
        Environment { jvmti: jvmti, jni: jni }
    }
}

impl JNIEnvironment {

    pub fn new(jni: JNIEnvPtr) -> JNIEnvironment {
        JNIEnvironment { jni: jni }
    }
}

impl JVMTIEnvironment {

    pub fn new(jvmti: JVMTIEnvPtr) -> JVMTIEnvironment {
        JVMTIEnvironment { jvmti: jvmti }
    }

}

impl JVMAgent {

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

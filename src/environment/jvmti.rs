use super::super::capabilities::Capabilities;
use super::super::error::{wrap_error, NativeError};
use super::super::event::{EventCallbacks, VMEvent};
use super::super::event_handler::*;
use super::super::version::VersionNumber;
use super::super::native::{JavaObject, JVMTIEnvPtr};
use super::super::native::jvmti_native::jvmtiCapabilities;
use std::ptr;

pub trait JVMTI {

    ///
    /// Return the JVM TI version number, which includes major, minor and micro version numbers.
    ///
    fn get_version_number(&self) -> VersionNumber;
    /// Set new capabilities by adding the capabilities whose values are set to true in new_caps.
    /// All previous capabilities are retained.
    /// Some virtual machines may allow a limited set of capabilities to be added in the live phase.
    fn add_capabilities(&mut self, new_capabilities: &Capabilities) -> Result<Capabilities, NativeError>;
    fn get_capabilities(&self) -> Capabilities;
    /// Set the functions to be called for each event. The callbacks are specified by supplying a
    /// replacement function table. The function table is copied--changes to the local copy of the
    /// table have no effect. This is an atomic action, all callbacks are set at once. No events
    /// are sent before this function is called. When an entry is None no event is sent.
    /// An event must be enabled and have a callback in order to be sent--the order in which this
    /// function and set_event_notification_mode are called does not affect the result.
    fn set_event_callbacks(&mut self, callbacks: EventCallbacks) -> Option<NativeError>;
    fn set_event_notification_mode(&mut self, event: VMEvent, mode: bool) -> Option<NativeError>;
}

pub struct JVMTIEnvironment {

    jvmti: JVMTIEnvPtr
}

impl JVMTIEnvironment {
    pub fn new(env_ptr: JVMTIEnvPtr) -> JVMTIEnvironment {
        JVMTIEnvironment { jvmti: env_ptr }
    }
}

impl JVMTI for JVMTIEnvironment {

    fn get_version_number(&self) -> VersionNumber {
        unsafe {
            let mut version: i32 = 0;
            let version_ptr = &mut version;
            (**self.jvmti).GetVersionNumber.unwrap()(self.jvmti, version_ptr);
            let uversion = *version_ptr as u32;
            VersionNumber::from_u32(&uversion)
        }
    }

    fn add_capabilities(&mut self, new_capabilities: &Capabilities) -> Result<Capabilities, NativeError> {
        let native_caps = new_capabilities.to_native();
        let caps_ptr:*const jvmtiCapabilities = &native_caps;

        unsafe {
            match wrap_error((**self.jvmti).AddCapabilities.unwrap()(self.jvmti, caps_ptr)) {
                NativeError::NoError => Ok(self.get_capabilities()),
                err @ _ => Err(err)
            }
        }
    }

    fn get_capabilities(&self) -> Capabilities {
        unsafe {
            let caps = Capabilities::new();
            let mut native_caps = caps.to_native();
            {
                let cap_ptr = &mut native_caps;
                (**self.jvmti).GetCapabilities.unwrap()(self.jvmti, cap_ptr);
            }
            Capabilities::from_native(&native_caps)
        }
    }

    fn set_event_callbacks(&mut self, callbacks: EventCallbacks) -> Option<NativeError> {
        register_vm_init_callback(callbacks.vm_init);
        let (native_callbacks, callbacks_size) = registered_callbacks();

        unsafe {
            // (**self.jvmti).SetEventCallbacks.unwrap()(self.jvmti, &callbacks.to_native(), size_of::<jvmtiEventCallbacks>() as i32)
            match wrap_error((**self.jvmti).SetEventCallbacks.unwrap()(self.jvmti, &native_callbacks, callbacks_size)) {
                NativeError::NoError => None,
                err @ _ => Some(err)
            }
        }
    }

    fn set_event_notification_mode(&mut self, event: VMEvent, mode: bool) -> Option<NativeError> {
        unsafe {
            let mode_i = match mode { true => 1, false => 0 };
            let sptr: JavaObject = ptr::null_mut();

            match wrap_error((**self.jvmti).SetEventNotificationMode.unwrap()(self.jvmti, mode_i, event as u32, sptr)) {
                NativeError::NoError => None,
                err @ _ => Some(err)
            }
        }
    }
}

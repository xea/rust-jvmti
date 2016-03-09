use super::capabilities::Capabilities;
use super::error::NativeError;
use super::environment::jvm::JVMF;
use super::environment::jvmti::{JVMTI};
use super::event::{EventCallbacks, VMEvent};
use super::native::JavaThread;
use super::thread::Thread;
use super::version::VersionNumber;
use std::collections::HashMap;

/// Allows testing of JVM and JVMTI-related functions by emulating (mocking) a JVM agent.
pub struct JVMEmulator {
    pub capabilities: Capabilities,
    pub callbacks: EventCallbacks,
    pub events: HashMap<VMEvent, bool>
}

impl JVMEmulator {
    pub fn new() -> JVMEmulator {
        JVMEmulator {
            capabilities: Capabilities::new(),
            callbacks: EventCallbacks::new(),
            events: HashMap::new()
        }
    }
}

impl JVMF for JVMEmulator {
    fn get_environment(&self) -> Result<Box<JVMTI>, NativeError> {
        Ok(Box::new(JVMEmulator::new()))
    }

    fn destroy(&self) -> Result<(), NativeError> {
        Ok(())
    }
}

impl JVMTI for JVMEmulator {

    fn get_version_number(&self) -> VersionNumber {
        VersionNumber::unknown()
    }

    fn add_capabilities(&mut self, new_capabilities: &Capabilities) -> Result<Capabilities, NativeError> {
        let merged = self.capabilities.merge(&new_capabilities);
        self.capabilities = merged;
        Ok(self.capabilities.clone())
    }

    fn get_capabilities(&self) -> Capabilities {
        self.capabilities.clone()
    }

    fn set_event_callbacks(&mut self, callbacks: EventCallbacks) -> Option<NativeError> {
        self.callbacks = callbacks;

        None
    }

    fn set_event_notification_mode(&mut self, event: VMEvent, mode: bool) -> Option<NativeError> {
        self.events.insert(event, mode);
        None
    }

    fn get_thread_info(&self, thread_id: &JavaThread) -> Result<Thread, NativeError> {
        Err(NativeError::NoError)
    }
}

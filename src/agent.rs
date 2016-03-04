use super::capabilities::Capabilities;
use super::environment::jvm::{JVMF, JVMAgent};
use super::environment::jvmti::JVMTI;
use super::event::*;
use super::error::*;
use super::native::JavaVMPtr;
use super::version::VersionNumber;

pub struct Agent {
    jvm: Box<JVMF>,
    pub capabilities: Capabilities,
    callbacks: EventCallbacks,
    environment: Box<JVMTI>
}

impl Agent {

    ///
    /// Create a newly initialised but blank JVM `Agent` instance using the provided Java VM pointer.
    ///
    pub fn new(vm: JavaVMPtr) -> Agent {
        let jvm_agent = JVMAgent::new(vm);

        match jvm_agent.get_environment() {
            Ok(environment) => Agent {
                jvm: Box::new(jvm_agent),
                capabilities: Capabilities::new(),
                callbacks: EventCallbacks::new(),
                environment: environment
            },
            Err(err) => panic!("FATAL: Could not get JVMTI environment: {}", translate_error(&err))
        }

    }

    ///
    /// Create a newly initialised but blank JVM `Agent` instance using the provided JVM agent.
    pub fn new_from(jvm: Box<JVMF>) -> Agent {

        match jvm.get_environment() {
            Ok(environment) => Agent {
                jvm: jvm,
                capabilities: Capabilities::new(),
                callbacks: EventCallbacks::new(),
                environment: environment
            },
            Err(err) => panic!("FATAL: Could not get JVMTI environment: {}", translate_error(&err))
        }
    }

    pub fn get_version(&self) -> VersionNumber {
        self.environment.get_version_number()
    }

    pub fn shutdown(&self) {
        // TODO implement this method
    }

    pub fn destroy(&self) -> Result<(), NativeError> {
        self.jvm.destroy()
    }

    pub fn update(&self) {
        self.environment.add_capabilities(&self.capabilities);
    }

    pub fn on_method_entry(&mut self, handler: Option<FnMethodEntry>) {
        self.callbacks.method_entry = handler;
        self.capabilities.can_generate_method_entry_events = handler.is_some();
    }

    pub fn on_method_exit(&mut self, handler: Option<FnMethodExit>) {
        self.callbacks.method_exit = handler;
        self.capabilities.can_generate_method_exit_events = handler.is_some();
    }

    pub fn on_vm_init(&mut self, handler: Option<FnVMInit>) {
        self.callbacks.vm_init = handler;
    }

    pub fn on_vm_death(&mut self, handler: Option<FnVMDeath>) {
        self.callbacks.vm_death = handler;
    }

    pub fn on_vm_start(&mut self, handler: Option<FnVMStart>) {
        self.callbacks.vm_start = handler;
    }

    pub fn on_vm_object_alloc(&mut self, handler: Option<FnVMObjectAlloc>) {
        self.callbacks.vm_object_alloc = handler;
        self.capabilities.can_generate_vm_object_alloc_events = handler.is_some();
    }

    pub fn on_thread_start(&mut self, handler: Option<FnThreadStart>) {
        self.callbacks.thread_start = handler;
    }

    pub fn on_thread_end(&mut self, handler: Option<FnThreadEnd>) {
        self.callbacks.thread_end = handler;
    }

    pub fn on_exception(&mut self, handler: Option<FnException>) {
        self.callbacks.exception = handler;
        self.capabilities.can_generate_exception_events = handler.or(self.callbacks.exception_catch).is_some();
    }

    pub fn on_exception_catch(&mut self, handler: Option<FnExceptionCatch>) {
        self.callbacks.exception_catch = handler;
        self.capabilities.can_generate_exception_events = handler.or(self.callbacks.exception).is_some();
    }

    pub fn on_monitor_wait(&mut self, handler: Option<FnMonitorWait>) {
        self.callbacks.monitor_wait = handler;

        let has_some = handler
            .or(self.callbacks.monitor_waited)
            .or(self.callbacks.monitor_contended_enter)
            .or(self.callbacks.monitor_contended_entered).is_some();

        self.capabilities.can_generate_monitor_events = has_some;
    }

    pub fn on_monitor_waited(&mut self, handler: Option<FnMonitorWaited>) {
        self.callbacks.monitor_waited = handler;

        let has_some = handler
            .or(self.callbacks.monitor_wait)
            .or(self.callbacks.monitor_contended_enter)
            .or(self.callbacks.monitor_contended_entered).is_some();

        self.capabilities.can_generate_monitor_events = has_some;
    }

    pub fn on_monitor_contended_enter(&mut self, handler: Option<FnMonitorContendedEnter>) {
        self.callbacks.monitor_contended_enter = handler;

        let has_some = handler
            .or(self.callbacks.monitor_wait)
            .or(self.callbacks.monitor_waited)
            .or(self.callbacks.monitor_contended_entered).is_some();

        self.capabilities.can_generate_monitor_events = has_some;
    }

    pub fn on_monitor_contended_entered(&mut self, handler: Option<FnMonitorContendedEntered>) {
        self.callbacks.monitor_contended_entered = handler;

        let has_some = handler
            .or(self.callbacks.monitor_wait)
            .or(self.callbacks.monitor_waited)
            .or(self.callbacks.monitor_contended_enter).is_some();

        self.capabilities.can_generate_monitor_events = has_some;
    }

    pub fn on_field_access(&mut self, handler: Option<FnFieldAccess>) {
        self.callbacks.field_access = handler;
        self.capabilities.can_generate_field_access_events = handler.is_some();
    }

    pub fn on_field_modification(&mut self, handler: Option<FnFieldModitification>) {
        self.callbacks.field_modification = handler;
        self.capabilities.can_generate_field_modification_events = handler.is_some();
    }

    pub fn on_garbage_collection_start(&mut self, handler: Option<FnGarbageCollectionStart>) {
        self.callbacks.garbage_collection_start = handler;
        self.capabilities.can_generate_garbage_collection_events = handler.or(self.callbacks.garbage_collection_finish).is_some();
    }

    pub fn on_garbage_collection_finish(&mut self, handler: Option<FnGarbageCollectionFinish>) {
        self.callbacks.garbage_collection_finish = handler;
        self.capabilities.can_generate_garbage_collection_events = handler.or(self.callbacks.garbage_collection_start).is_some();
    }
}

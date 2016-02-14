use super::wrapper::agent_capabilities::AgentCapabilities;
use super::wrapper::environment::*;
use super::wrapper::event::*;
use super::wrapper::error::{translate_error};
use super::wrapper::native::{JavaVMPtr};

///
/// Provides a type-safe and Rust-idiomatic (I only hope at this point) interface for accessing
/// JVMTI and JNI functionality withouth having to deal with obscure API calls.
///
pub struct Agent {
    jvm: JVMAgent,
    capabilities: AgentCapabilities,
    callbacks: EventCallbacks
}

impl Agent {

    /// Return an empty but initialised JVM agent without any registered event handler callbacks
    pub fn new(jvm: JavaVMPtr) -> Agent {
        Agent {
            jvm: JVMAgent::new(jvm),
            capabilities: AgentCapabilities::new(),
            callbacks: EventCallbacks::new()
        }
    }

    /// Register a handler method that is called when the Java code is about to enter a method
    pub fn on_method_entry(&mut self, handler: Option<FnMethodEntry>) -> () {
        self.callbacks.method_entry = handler;

        if handler.is_some() {
            self.capabilities.can_generate_method_entry_events = true;
        } else {
            self.capabilities.can_generate_method_entry_events = false;
        }
    }

    pub fn on_method_exit(&mut self, handler: Option<FnMethodExit>) -> () {
        self.callbacks.method_exit = handler;

        if handler.is_some() {
            self.capabilities.can_generate_method_exit_events = true;
        } else {
            self.capabilities.can_generate_method_exit_events = false;
        }
    }

    pub fn on_exception(&mut self, handler: Option<FnException>) -> () {
        self.callbacks.exception = handler;

        if handler.is_some() {
            self.capabilities.can_generate_exception_events = true;
        } else {
            self.capabilities.can_generate_exception_events = false;
        }
    }

    pub fn on_exception_catch(&mut self, handler: Option<FnExceptionCatch>) -> () {
        self.callbacks.exception_catch = handler;

        if handler.is_some() {
            self.capabilities.can_generate_exception_events = true;
        } else {
            self.capabilities.can_generate_exception_events = false;
        }
    }

    pub fn on_monitor_wait(&mut self, handler: Option<FnMonitorWait>) -> () {
        self.callbacks.monitor_wait = handler;

        if handler.is_some() {
            self.capabilities.can_generate_monitor_events = true;
        } else {
            self.capabilities.can_generate_monitor_events = false;
        }

    }

    pub fn on_monitor_enter(&mut self, handler: Option<FnMonitorEntered>) -> () {
        self.callbacks.monitor_entered = handler;

        if handler.is_some() {
            self.capabilities.can_generate_monitor_events = true;
        } else {
            self.capabilities.can_generate_monitor_events = false;
        }
    }

    pub fn on_monitor_contended_wait(&mut self, handler: Option<FnMonitorContendedEnter>) -> () {
        self.callbacks.monitor_contended_enter = handler;

        if handler.is_some() {
            self.capabilities.can_generate_monitor_events = true;
        } else {
            self.capabilities.can_generate_monitor_events = false;
        }
    }

    pub fn on_monitor_contended_enter(&mut self, handler: Option<FnMonitorContendedEntered>) -> () {
        self.callbacks.monitor_contended_entered = handler;

        if handler.is_some() {
            self.capabilities.can_generate_monitor_events = true;
        } else {
            self.capabilities.can_generate_monitor_events = false;
        }
    }

    pub fn on_vm_object_alloc(&mut self, handler: Option<FnVMObjectAlloc>) -> () {
        self.callbacks.vm_object_alloc = handler;

        if handler.is_some() {
            self.capabilities.can_generate_vm_object_alloc_events = true;
        } else {
            self.capabilities.can_generate_vm_object_alloc_events = false;
        }
    }

    pub fn start(&self) -> () {
        match self.jvm.get_environment() {
            Result::Ok(env) => self.setup_environment(env),
            Result::Err(err) => {
                println!("Error during obtaining JVMTI Environment: {}", translate_error(&err));
                //return wrap_error(err as u32) as ReturnValue;
            }
        }

    }

    fn setup_environment(&self, env: JVMTIEnvironment) -> () {
        match env.add_capabilities(self.capabilities.clone()) {
            Ok(_) => {
                println!("Agent capabilities were added successfully");
                self.register_callbacks(env);
            },
            Err(err) => println!("Error during adding agent capabilities: {}", translate_error(&err))
        }

        println!("Successfully obtained JVMTI Environment");
    }

    fn register_callbacks(&self, env: JVMTIEnvironment) -> () {
        match env.set_event_callbacks(self.callbacks.clone()) {
            None => {
                env.set_event_notification_mode(VMEvent::VMObjectAlloc, self.callbacks.vm_object_alloc.is_some());
                env.set_event_notification_mode(VMEvent::VMStart, false);
                env.set_event_notification_mode(VMEvent::MethodEntry, self.callbacks.method_entry.is_some());
                env.set_event_notification_mode(VMEvent::MethodExit, self.callbacks.method_exit.is_some());
                env.set_event_notification_mode(VMEvent::Exception, self.callbacks.exception.is_some());
                env.set_event_notification_mode(VMEvent::ExceptionCatch, self.callbacks.exception_catch.is_some());
                env.set_event_notification_mode(VMEvent::ExceptionCatch, self.callbacks.exception_catch.is_some());
                env.set_event_notification_mode(VMEvent::MonitorWait, self.callbacks.monitor_wait.is_some());
                env.set_event_notification_mode(VMEvent::MonitorWaited, self.callbacks.monitor_entered.is_some());
                env.set_event_notification_mode(VMEvent::MonitorContendedEnter, self.callbacks.monitor_contended_enter.is_some());
                env.set_event_notification_mode(VMEvent::MonitorContendedEntered, self.callbacks.monitor_contended_entered.is_some());
            },
            Some(err) => println!("Error during setting event callbacks: {}", translate_error(&err))
        }
    }
}

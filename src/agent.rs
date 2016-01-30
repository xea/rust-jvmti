use super::wrapper::agent_capabilities::AgentCapabilities;
use super::wrapper::environment::*;
use super::wrapper::event::*;
use super::wrapper::error::{translate_error, wrap_error};
use super::wrapper::native::{JavaVMPtr, ReturnValue};
use super::error::Error;

pub struct Agent {
    jvm: JVMAgent,
    capabilities: AgentCapabilities,
    callbacks: EventCallbacks
}

impl Agent {

    pub fn new(jvm: JavaVMPtr) -> Agent {
        Agent {
            jvm: JVMAgent::new(jvm),
            capabilities: AgentCapabilities::new(),
            callbacks: EventCallbacks::new()
        }
    }

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
        /*
        //caps.can_generate_vm_object_alloc_events = true;
        */

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
                env.set_event_notification_mode(VMEvent::VMObjectAlloc, false);
                env.set_event_notification_mode(VMEvent::VMStart, false);
                env.set_event_notification_mode(VMEvent::MethodEntry, true);
                env.set_event_notification_mode(VMEvent::MethodExit, true);
                env.set_event_notification_mode(VMEvent::Exception, true);
                env.set_event_notification_mode(VMEvent::ExceptionCatch, true);
                println!("Setting event callbacks was successful");
            },
            Some(err) => println!("Error during setting event callbacks: {}", translate_error(&err))
        }
    }
}

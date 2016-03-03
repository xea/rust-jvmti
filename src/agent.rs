use super::capabilities::Capabilities;
use super::environment::jvm::{JVMF, JVMAgent};
use super::environment::jvmti::JVMTI;
use super::event::{FnMethodEntry, EventCallbacks};
use super::native::JavaVMPtr;
use super::version::VersionNumber;

pub struct Agent {
    jvm: Box<JVMF>,
    pub capabilities: Capabilities,
    callbacks: EventCallbacks,
}

impl Agent {

    ///
    /// Create a newly initialised but blank JVM `Agent` instance using the provided Java VM pointer.
    ///
    pub fn new(vm: JavaVMPtr) -> Agent {
        Agent {
            jvm: Box::new(JVMAgent::new(vm)),
            capabilities: Capabilities::new(),
            callbacks: EventCallbacks::new(),
        }
    }

    ///
    /// Create a newly initialised but blank JVM `Agent` instance using the provided JVM agent.
    pub fn new_from(jvm: Box<JVMF>) -> Agent {
        Agent {
            jvm: jvm,
            capabilities: Capabilities::new(),
            callbacks: EventCallbacks::new()
        }
    }

    pub fn get_version(&self) -> VersionNumber {
        match self.jvm.get_environment() {
            Ok(env) => env.get_version_number(),
            Err(_) => VersionNumber::unknown()
        }
    }

    pub fn shutdown(&self) {
        // TODO implement this method
    }

    pub fn update(&self) {
    }

    pub fn on_method_entry(&mut self, handler: Option<FnMethodEntry>) -> bool {
        self.callbacks.method_entry = handler;
        true
    }

}

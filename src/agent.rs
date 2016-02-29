use super::capabilities::Capabilities;
use super::environment::jvm::JVMAgent;
use super::event::{FnMethodEntry, EventCallbacks};
use super::native::JavaVMPtr;

pub struct Agent {
    jvm: JVMAgent,
    capabilities: Capabilities,
    callbacks: EventCallbacks
}

impl Agent {

    ///
    /// Create a newly initialised but blank JVM `Agent` instance using the provided Java VM pointer.
    ///
    pub fn new(vm: JavaVMPtr) -> Agent {
        Agent { jvm: JVMAgent::new(vm), capabilities: Capabilities::new(), callbacks: EventCallbacks::new() }
    }

    pub fn get_version(&self) -> u32 {
        0xBABE
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

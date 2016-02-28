use super::capabilities::Capabilities;
use super::event::EventCallbacks;
use super::native::JavaVMPtr;

pub struct Agent {
    capabilities: Capabilities,
    callbacks: EventCallbacks
}

impl Agent {

    pub fn new(vm: JavaVMPtr) -> Agent {
        Agent { capabilities: Capabilities::new(), callbacks: EventCallbacks::new() }
    }

    pub fn get_version(&self) -> u32 {
        0xBABE
    }

    pub fn shutdown(&self) {
        // TODO implement this method
    }
}

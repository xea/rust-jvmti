extern crate jvmti;

use jvmti::capabilities::Capabilities;
use jvmti::native::{JavaVMPtr, MutString, VoidPtr, ReturnValue};
use handler::FnMethodEntry;

mod handler;

///
/// `Agent_OnLoad` is the actual entry point of the agent code and it is directly called by the
/// Java Virtual Machine.
///
#[no_mangle]
#[allow(non_snake_case, unused_variables)]
pub extern fn Agent_OnLoad(vm: JavaVMPtr, options: MutString, reserved: VoidPtr) -> ReturnValue {

    let agent = Agent::new(vm);
    agent.on_method_entry(Some(on_method_entry));

    return 0;
}

///
/// `Agent_OnUnload` is the exit point of the agent code. It is called when the JVM has finished
/// running and the virtual machine is unloading the agent from memory before shutting down.
/// Note: this method is also called when the JVM crashes due to an internal error.
///
#[no_mangle]
#[allow(non_snake_case, unused_variables)]
pub extern fn Agent_OnUnload(vm: JavaVMPtr) {
}

///
pub struct Agent {

    vm: JavaVMPtr,
    capabilities: Capabilities
}

impl Agent {

    pub fn new(vmptr: JavaVMPtr) -> Agent {
        Agent { vm: vmptr }
    }

    pub fn get_version(&self) -> u32 {
        0xBABE
    }

    pub fn on_method_entry(&self, handler: Option<FnMethodEntry>) {

    }
}

fn on_method_entry() -> () {

}

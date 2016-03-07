extern crate libc;
#[macro_use]
extern crate lazy_static;

use agent::Agent;
use native::{JavaVMPtr, MutString, VoidPtr, ReturnValue};

pub mod agent;
pub mod capabilities;
pub mod class;
pub mod emulator;
pub mod environment;
pub mod error;
pub mod event;
pub mod event_handler;
pub mod native;
pub mod util;
pub mod version;

fn on_method_entry() {
    println!("Method entry");
}

fn on_method_exit() {
    println!("Method exit");
}

fn on_thread_start() {
    println!("Thread start");
}

fn on_thread_end() {
    println!("Thread end");
}

///
/// `Agent_OnLoad` is the actual entry point of the agent code and it is directly called by the
/// Java Virtual Machine.
///
#[no_mangle]
#[allow(non_snake_case, unused_variables)]
pub extern fn Agent_OnLoad(vm: JavaVMPtr, options: MutString, reserved: VoidPtr) -> ReturnValue {

    let mut agent = Agent::new(vm);
    agent.on_method_entry(Some(on_method_entry));
    agent.on_method_exit(Some(on_method_exit));
    agent.on_thread_start(Some(on_thread_start));
    agent.on_thread_end(Some(on_thread_end));

    agent.update();

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

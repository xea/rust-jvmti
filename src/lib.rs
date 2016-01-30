extern crate libc;

///
/// This is the main module of the JVMTI native agent.
///
use wrapper::native::{JavaVMPtr, VoidPtr, MutString, ReturnValue};
use wrapper::environment::{JVMTI, JVMTIEnvironment};
use wrapper::error::*;
use agent::Agent;

mod agent;
mod error;
mod wrapper;

#[no_mangle]
#[allow(non_snake_case)]
#[allow(unused_variables)]
pub extern fn Agent_OnLoad(vm: JavaVMPtr, options: MutString, reserved: VoidPtr) -> ReturnValue {
    let mut agent = Agent::new(vm);

    agent.on_method_entry(Some(on_method_entry));
    agent.on_method_exit(Some(on_method_exit));
    agent.on_exception(Some(on_exception));
    agent.on_exception_catch(Some(on_exception_catch));

    agent.start();
    return NativeError::NoError as ReturnValue;
}

fn on_method_entry() -> () {
    println!("Method entry")
}

fn on_method_exit() -> () {
    println!("Method exit")
}

fn on_exception() -> () {
    println!("Exception event")
}

fn on_exception_catch() -> () {
    println!("Phew")
}

fn on_vm_object_alloc(size: u64) -> () {

}

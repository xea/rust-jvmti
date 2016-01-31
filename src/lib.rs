extern crate libc;

///
/// This is the main module of the JVMTI native agent.
///
use wrapper::native::{JavaVMPtr, VoidPtr, MutString, ReturnValue};
//use wrapper::environment::{JVMTI, JVMTIEnvironment};
use wrapper::class::*;
use wrapper::error::*;
use wrapper::method::Method;
use wrapper::thread::Thread;
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
    agent.on_vm_object_alloc(Some(on_vm_object_alloc));

    agent.start();
    return NativeError::NoError as ReturnValue;
}

fn on_method_entry(method: Method, class: Class, thread: Thread) -> () {
    println!("Method entry: {}: {}.{}{}", thread.name, class.signature.signature, method.signature.name, method.signature.signature)
}

fn on_method_exit(method: Method, thread: Thread) -> () {
    println!("Method exit: {}: {}{}", thread.name, method.signature.name, method.signature.signature)
}

fn on_exception(exception_class: Class) -> () {
    println!("Exception event: {}", exception_class.signature.signature)
}

fn on_exception_catch() -> () {
    println!("Phew")
}

fn on_vm_object_alloc(size: u64) -> () {
    println!("Allocated an object of size: {}", size)
}

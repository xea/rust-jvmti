extern crate libc;
#[macro_use]
extern crate lazy_static;
extern crate time;

///
/// This is the main module of the JVMTI native agent.
///
use wrapper::native::{JavaVMPtr, VoidPtr, MutString, ReturnValue};
use wrapper::class::*;
use wrapper::error::*;
use wrapper::method::Method;
use wrapper::thread::Thread;
use agent::Agent;
use benchmark::{BenchmarkKey, MethodTimer, MethodCounter};
use runtime::{MethodInvocation};
use time::now;

pub mod agent;
mod benchmark;
mod data_store;
mod runtime;
pub mod wrapper;

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

#[no_mangle]
#[allow(non_snake_case)]
#[allow(unused_variables)]
pub extern fn Agent_OnUnload(vm: JavaVMPtr) -> () {
    let data = MethodCounter::get_all();
    for counter in data.keys() {
        match data.get(counter) {
            Some(entry) => println!("{} {}", counter.key, entry),
            _ => ()
        }
    }
}

fn on_method_entry(method: Method, class: Class, thread: Thread) -> () {
    let invocation = MethodInvocation { class: class, method: method, thread: thread, at: now() };
    let time_key = BenchmarkKey::new("method-calltime".to_string(), invocation.thread.name.clone());

    MethodTimer::enter(&time_key);
    MethodCounter::enter(&BenchmarkKey::new("method-count".to_string(), invocation.class.signature.fqn() + "." + &invocation.method.signature.name));
}

fn on_method_exit(method: Method, class: Class, thread: Thread) -> () {
    let invocation = MethodInvocation { class: class, method: method, thread: thread, at: now() };
    let time_key = BenchmarkKey::new("method-calltime".to_string(), invocation.thread.name);

    let result = MethodTimer::exit(&time_key);

    match result {
        Some((stack_size, duration)) => {
        },//{ for _ in 1..(ssize) { print!(" "); }; println!("Result: {} {} {} {}.{}s", thread.name, class.signature.fqn(), method.signature.name, duration.num_seconds(), duration.num_milliseconds()) },
        None => println!("Who knows")
    }
//    println!("Method exit: {}: {}.{}{} = {}", thread.name, class.signature.fqn(), method.signature.name, method.signature.signature, no - 1);

}

fn on_exception(exception_class: Class) -> () {
    println!("Exception event: {}", exception_class.signature.signature)
}

fn on_exception_catch() -> () {
    //println!("Phew")
}

fn on_vm_object_alloc(size: u64) -> () {
    println!("Allocated an object of size: {}", size)
}

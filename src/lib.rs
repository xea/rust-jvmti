extern crate libc;
#[macro_use]
extern crate lazy_static;

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
use benchmark::{Benchmark, BenchmarkKey, BenchmarkValue};

mod agent;
mod benchmark;
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
    let key = BenchmarkKey { category: "method".to_string(), id: "TEST".to_string() };
    let r = Benchmark::get(&key);

    let no = match r {
        Some(value) => value.value,
        None => 0
    };
    Benchmark::update(key.clone(), BenchmarkValue { value: no + 1 });
    println!("Method entry: {}: {}.{}{} = {}", thread.name, class.signature.fqn(), method.signature.name, method.signature.signature, no + 1);
}

fn on_method_exit(method: Method, class: Class, thread: Thread) -> () {
    let key = BenchmarkKey { category: "method".to_string(), id: "TEST".to_string() };
    let r = Benchmark::get(&key);

    let no = match r {
        Some(value) => value.value,
        None => 0
    };
    Benchmark::update(key.clone(), BenchmarkValue { value: no - 1 });
    println!("Method exit: {}: {}.{}{} = {}", thread.name, class.signature.fqn(), method.signature.name, method.signature.signature, no - 1);
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

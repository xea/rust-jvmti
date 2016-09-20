extern crate libc;
#[macro_use]
extern crate lazy_static;
extern crate time;

use agent::Agent;
use context::static_context;
use native::{JavaVMPtr, MutString, VoidPtr, ReturnValue};
use options::Options;
use runtime::*;
use thread::Thread;
use util::stringify;

pub mod agent;
pub mod bytecode;
pub mod capabilities;
pub mod class;
pub mod context;
pub mod emulator;
pub mod environment;
pub mod error;
pub mod event;
pub mod event_handler;
pub mod method;
pub mod native;
pub mod options;
pub mod runtime;
pub mod thread;
pub mod util;
pub mod version;

/*
 * TODO The functions below are essentially parts of an actual client implementation. Because this
 * implementation is highly experimental and incomplete they shall remain here for a while but
 * they will have to find a new home, eventually
 */

fn on_method_entry(event: MethodInvocationEvent) {
    //println!("[M-{}.{}::{}]", event.class_sig.package, event.class_sig.name, event.method_sig.name);

    static_context().method_enter(&event.thread.id);
}

fn on_method_exit(event: MethodInvocationEvent) {
    match static_context().method_exit(&event.thread.id) {
        Some(_) => (),
        //Some(duration) => println!("Method {} exited after {}", event.method_sig.name, duration),
        None => println!("Method has no start: {}", event.method_sig.name)
    }
}

fn on_thread_start(thread: Thread) {
    println!("[TS-{}]", thread.name);

    static_context().thread_start(&thread.id);
}

fn on_thread_end(thread: Thread) {
    println!("[TE-{}]", thread.name);

    match static_context().thread_end(&thread.id) {
        Some(duration) => println!("Thread {} lived {}", thread.name, duration),
        None => println!("Thread {} has no start", thread.name)
    }
}

fn on_monitor_wait(thread: Thread) {
    println!("[W1-{}]", thread.name);
}

fn on_monitor_waited(thread: Thread) {
    println!("[W2-{}]", thread.name);
}

fn on_monitor_contended_enter(thread: Thread) {
    println!("[C1-{}]", thread.name);

    static_context().monitor_enter(&thread.id);
}

fn on_monitor_contended_entered(thread: Thread) {
    println!("[C2-{}]", thread.name);

    match static_context().monitor_entered(&thread.id) {
        Some(duration) => println!("Thread {} waited {}", thread.name, duration),
        None => println!("Thread {} has never waited", thread.name)
    }
}

fn on_class_file_load() {
}

///
/// `Agent_OnLoad` is the actual entry point of the agent code and it is called by the
/// Java Virtual Machine directly.
///
#[no_mangle]
#[allow(non_snake_case, unused_variables)]
pub extern fn Agent_OnLoad(vm: JavaVMPtr, options: MutString, reserved: VoidPtr) -> ReturnValue {
    let options = Options::parse(stringify(options));
    println!("Starting up as {}", options.agent_id);

    let mut agent = Agent::new(vm);

    agent.on_method_entry(Some(on_method_entry));
    agent.on_method_exit(Some(on_method_exit));
    agent.on_thread_start(Some(on_thread_start));
    agent.on_thread_end(Some(on_thread_end));
    agent.on_monitor_wait(Some(on_monitor_wait));
    agent.on_monitor_waited(Some(on_monitor_waited));
    agent.on_monitor_contended_enter(Some(on_monitor_contended_enter));
    agent.on_monitor_contended_entered(Some(on_monitor_contended_entered));
    agent.on_class_file_load(Some(on_class_file_load));

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

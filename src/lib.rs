extern crate libc;

///
/// This is the main module of the JVMTI native agent.
///
use libc::c_char;
use wrapper::agent_capabilities::AgentCapabilities;
use wrapper::native::{JavaVMPtr, VoidPtr, ReturnValue};
use wrapper::environment::{JVMTI, JVMAgent, JVMTIEnvironment};
use wrapper::error::*;
use wrapper::event::{VMEvent, EventCallbacks};

mod wrapper;

#[no_mangle]
#[allow(non_snake_case)]
#[allow(unused_variables)]
pub extern fn Agent_OnLoad(vm: JavaVMPtr, options: *mut c_char, reserved: VoidPtr) -> ReturnValue {
    let agent = JVMAgent::new(vm);

    match agent.get_environment() {
        Result::Ok(env) => setup_environment(env),
        Result::Err(err) => {
            println!("Error during obtaining JVMTI Environment: {}", translate_error(&err));
            return wrap_error(err as u32) as ReturnValue;
        }
    }

    return NativeError::NoError as ReturnValue;
}

fn setup_environment(env: JVMTIEnvironment) -> () {
    let mut caps = AgentCapabilities::new();
    caps.can_generate_method_entry_events = true;
    caps.can_generate_method_exit_events = true;
    caps.can_generate_vm_object_alloc_events = true;
    caps.can_generate_exception_events = true;

    match env.add_capabilities(caps) {
        Ok(_) => {
            println!("Agent capabilities were added successfully");
            register_callbacks(env);
        },
        Err(err) => println!("Error during adding agent capabilities: {}", translate_error(&err))
    }

    println!("Successfully obtained JVMTI Environment");
}

fn register_callbacks(env: JVMTIEnvironment) -> () {
    let mut callbacks = EventCallbacks::new();

    /*
    callbacks.vm_object_alloc = Some(on_vm_object_alloc);
    callbacks.method_entry = Some(on_method_entry);
    callbacks.method_exit = Some(on_method_exit);
    callbacks.exception = Some(on_exception);
    callbacks.exception_catch = Some(on_exception_catch);
    */

    match env.set_event_callbacks(callbacks) {

        None => {
            env.set_event_notification_mode(VMEvent::VMObjectAlloc, true);
            env.set_event_notification_mode(VMEvent::VMStart, true);
            env.set_event_notification_mode(VMEvent::MethodEntry, true);
            env.set_event_notification_mode(VMEvent::MethodExit, true);
            env.set_event_notification_mode(VMEvent::Exception, true);
            env.set_event_notification_mode(VMEvent::ExceptionCatch, true);
            println!("Setting event callbacks was successful");
        },
        Some(err) => println!("Error during setting event callbacks: {}", translate_error(&err))
    }
}

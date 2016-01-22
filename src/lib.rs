extern crate libc;

use libc::c_char;
use jvmti_wrapper::{JavaVMPtr, NativeError, ReturnValue, VoidPtr};
use jvmti_wrapper::{translate_error, wrap_error};
use jvmti_wrapper::agent_capabilities::AgentCapabilities;
use jvmti_wrapper::event_callbacks::{EventCallbacks, VMEvent};
use jvmti_wrapper::jvmti_environment::JvmtiEnvironment;
use jvmti_wrapper::jvm_agent::JvmAgent;
use jvmti_wrapper::method::Method;

mod jvmti_wrapper;

#[no_mangle]
#[allow(non_snake_case)]
#[allow(unused_variables)]
pub extern fn Agent_OnLoad(vm: JavaVMPtr, options: *mut c_char, reserved: VoidPtr) -> ReturnValue {
    let agent = JvmAgent::new(vm);
    println!("Loading {}", agent.to_string());

    let result = agent.get_environment();

    match result {
        Result::Ok(env) => setup_environment(env),
        Result::Err(err) => {
            println!("Error during obtaining JVMTI Environment: {}", translate_error(&err));
            return wrap_error(err as u32) as ReturnValue;
        }
    }

    return NativeError::NoError as ReturnValue;
}

#[no_mangle]
#[allow(non_snake_case)]
#[allow(unused_variables)]
pub extern fn Agent_OnUnload(vm: JavaVMPtr) -> () {
}

#[no_mangle]
pub extern fn on_method_entry(method: Method) -> () {
    match method.get_class() {
        Err(err) => println!("Errro fasz {}", translate_error(&err)),
        Ok(class) => println!("signature: {} -> {}", class.get_signature(), method.name())
    }
}

#[no_mangle]
pub extern fn on_vm_object_alloc() -> () {
    //println!("On object alloc");
}

///
fn setup_environment(env: JvmtiEnvironment) -> () {
    let mut caps = AgentCapabilities::new();
    caps.can_generate_method_entry_events = true;
    caps.can_generate_vm_object_alloc_events = true;

    match env.add_capabilities(caps) {
        Ok(_) => {
            println!("Agent capabilities were addedd successfully");
            register_callbacks(&env);
        },
        Err(err) => println!("Error during adding agent capabilities: {}", translate_error(&err))
    }

    println!("Successfully obtained JVMTI Environment");
}

fn register_callbacks(env: &JvmtiEnvironment) -> () {
    let mut callbacks = EventCallbacks::new();

    callbacks.vm_object_alloc = Some(on_vm_object_alloc);
    callbacks.method_entry = Some(on_method_entry);

    match env.set_event_callbacks(callbacks) {

        None => {
            env.set_event_notification_mode(VMEvent::VMObjectAlloc, true);
            env.set_event_notification_mode(VMEvent::VMStart, true);
            env.set_event_notification_mode(VMEvent::MethodEntry, true);
            println!("Setting event callbacks was successful");
        },
        Some(err) => println!("Error during setting event callbacks: {}", translate_error(&err))
    }
}

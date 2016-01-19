extern crate libc;

use libc::c_char;
use libc::c_void;
use jvmti_wrapper::{JavaVMPtr, ReturnValue};
use jvmti_wrapper::jvm_agent::JvmAgent;

mod jvmti_wrapper;

#[no_mangle]
#[allow(non_snake_case)]
#[allow(unused_variables)]
pub extern fn Agent_OnLoad(vm: JavaVMPtr, options: *mut c_char, reserved: *mut c_void) -> ReturnValue {
    let agent = JvmAgent::new(vm);
    println!("{}", agent.to_string());
    return 0;
}

#[no_mangle]
#[allow(non_snake_case)]
#[allow(unused_variables)]
pub extern fn Agent_OnUnload(vm: JavaVMPtr) -> () {
}

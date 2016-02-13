extern crate jvmti;

use jvmti::wrapper::native::JavaVMPtr;
use jvmti::agent::Agent;
use std::ptr;
use std::mem;
use super::JVMTITestbed;


#[test]
fn new_agents_are_fully_initialised() {
    let mut testbed = JVMTITestbed::new();
    let mut test_ptr: *mut JVMTITestbed = &mut testbed;
    let mut test_ptr_ptr: *mut *mut JVMTITestbed = &mut test_ptr;

    let jvm_ptr: JavaVMPtr = unsafe { mem::transmute(test_ptr_ptr) };

    let agent = Agent::new(jvm_ptr);

    //assert_eq!(ptr::null_mut(), jvm_ptr);
}

extern crate jvmti;

use jvmti::native::{JavaVMPtr, MutString, VoidPtr, ReturnValue};

#[no_mangle]
#[allow(non_snake_case, unused_variables)]
pub extern fn Agent_OnLoad(vm: JavaVMPtr, options: MutString, reserved: VoidPtr) -> ReturnValue {
    return 0;
}

#[no_mangle]
#[allow(non_snake_case, unused_variables)]
pub extern fn Agent_OnUnload(vm: JavaVMPtr) {
}

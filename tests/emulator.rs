extern crate jvmti;
extern crate libc;

/*
#[cfg(test)]
mod tests {

    use jvmti::emulator::Emulator;
    use jvmti::native::JavaVMPtr;
    use libc::c_void;
    use std::ptr;

    #[test]
    fn initialize() {
        let emulator = Emulator::new();
        let jvm_ptr: JavaVMPtr = emulator.to_javavmptr();

        let mut env_ptr: *mut c_void = ptr::null_mut();
        let env_ptr_ptr: *mut *mut c_void = &mut env_ptr;

        unsafe {
            assert!((**jvm_ptr).GetEnv.is_some());
            assert_eq!(13, (**jvm_ptr).GetEnv.unwrap()(jvm_ptr, env_ptr_ptr, 13));
        }
    }
}
*/

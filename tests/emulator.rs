extern crate jvmti;
extern crate libc;

#[cfg(test)]
mod tests {

    use jvmti::emulator::Emulator;
    use jvmti::emulator::JVMTIEmulator;
    use libc::c_void;
    use std::ptr;

    #[test]
    fn emulators_can_be_transmuted_into_jvm_ptrs() {
        let mut emulator = Emulator::new();
        let mut eptr: *mut Emulator = &mut emulator;
        let vmptr = Emulator::transmute(&mut eptr);

        assert!(ptr::null_mut() != vmptr);
    }

    #[test]
    fn get_emulated_environment_fills_the_provided_pointer() {
        let mut emulator = Emulator::new();
        let mut eptr: *mut Emulator = &mut emulator;
        let vmptr = Emulator::transmute(&mut eptr);
        let mut env: *mut c_void = ptr::null_mut();
        let env_ptr: *mut *mut c_void = &mut env;

        unsafe {
            assert_eq!(ptr::null_mut(), env);
            (**vmptr).GetEnv.unwrap()(vmptr, env_ptr, 0);
            assert!(ptr::null_mut() != env_ptr);
        }
    }

    #[test]
    fn get_version_number_returns_the_emulator_version() {
        let mut version: i32 = 0;
        let version_ptr = &mut version;
        JVMTIEmulator::get_version_number(ptr::null_mut(), version_ptr);

        assert_eq!(0x07FA3020, *version_ptr);
    }
}

extern crate jvmti;
extern crate libc;

#[cfg(test)]
mod tests {

    use jvmti::emulator::Emulator;
    use jvmti::emulator::JVMTIEmulator;
    use jvmti::native::JVMTIEnvPtr;
    use jvmti::native::jvmti_native::jvmtiEnv;
    use libc::c_void;
    use std::ptr;
    use std::mem::transmute;

    #[test]
    fn the_fucking_feature_is_even_working() {
        let mut jvm_emulator = Emulator::new();
        let jvmti_emulator = JVMTIEmulator::new();
        let mut jvm_emu_ptr: *mut Emulator = &mut jvm_emulator;
        let jvm_emu_ptr_ptr: *mut *mut Emulator = &mut jvm_emu_ptr;

        jvm_emulator.reserved0 = unsafe { transmute(&jvmti_emulator) };

        let jvm_ptr = Emulator::transmute(jvm_emu_ptr_ptr);
        assert!(ptr::null_mut() != jvm_ptr);

        let jvm = unsafe { **jvm_ptr };
        assert!(jvm.GetEnv.is_some());

        let mut renv: *mut c_void = ptr::null_mut();
        let renvp: *mut *mut c_void = &mut renv;
        unsafe {
            let result = jvm.GetEnv.unwrap()(jvm_ptr, renvp, 0);
            assert_eq!(0, result);
            assert_eq!(jvm_emulator.reserved0, *renvp);

            let read_env = *renvp;
            let tenvp: *mut jvmtiEnv = transmute(read_env);
            let tenv = **tenvp;
            assert!(tenv.GetVersionNumber.is_some());
        }

    }
/*
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

    #[test]
    fn jvmti_environments_can_be_emulated() {
        let mut emu = JVMTIEmulator::new();
        let mut emu_ptr: *mut JVMTIEmulator = &mut emu;
        let env: JVMTIEnvPtr = JVMTIEmulator::transmute(&mut emu_ptr);

        unsafe { assert!((**env).GetVersionNumber.is_some()); }
    }
    */
}

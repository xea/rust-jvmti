extern crate jvmti;
extern crate libc;

#[cfg(test)]
mod tests {

    use jvmti::emulator::{JVMTIEmulator};
    use jvmti::environment::jvmti::{JVMTI, JVMTIEnvironment};
    use jvmti::native::{JVMTIEnvPtr};
    use std::mem::transmute;

    #[test]
    fn get_version_number_returns_environment_version() {
        let mut emu = JVMTIEmulator::new();
        let mut emu_ptr:*const JVMTIEmulator = &mut emu;
        let emu_ptr_ptr:*mut *const JVMTIEmulator = &mut emu_ptr;
        let env_ptr:JVMTIEnvPtr = unsafe { transmute(emu_ptr_ptr) };
        let env = JVMTIEnvironment::new(env_ptr);
        let version = env.get_version_number();
        assert_eq!(0x7FA, version.major_version);
        assert_eq!(0x30, version.minor_version);
        assert_eq!(0x20, version.micro_version);
    }

}

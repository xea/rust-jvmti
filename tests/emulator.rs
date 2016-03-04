extern crate jvmti;
extern crate libc;

#[cfg(test)]
mod tests {

    use jvmti::capabilities::Capabilities;
    use jvmti::emulator::JVMEmulator;
    use jvmti::environment::jvm::JVMF;
    use jvmti::environment::jvmti::JVMTI;
    use jvmti::version::VersionNumber;

    #[test]
    fn get_environment_returns_a_valid_environment() {
        let emu = JVMEmulator::new();

        assert!(emu.get_environment().is_ok());
        let env = emu.get_environment().ok().unwrap();
        assert_eq!(VersionNumber::unknown(), env.get_version_number());
    }

    #[test]
    fn get_version_number_returns_unknown_version() {
        let emu = JVMEmulator::new();

        assert!(emu.get_environment().is_ok());
        let env = emu.get_environment().ok().unwrap();
        assert_eq!(VersionNumber::unknown(), env.get_version_number());
    }

    #[test]
    fn add_capabilities_retains_the_previously_added_capabilities() {
        let mut emu = JVMEmulator::new();

        let mut capabilities = Capabilities::new();
        capabilities.can_get_bytecodes = true;

        assert_eq!(false, emu.capabilities.can_get_bytecodes);
        assert!(emu.add_capabilities(&capabilities).is_ok());
        assert_eq!(true, emu.capabilities.can_get_bytecodes);

        capabilities.can_get_bytecodes = false;
        capabilities.can_suspend = true;

        assert_eq!(false, emu.capabilities.can_suspend);
        assert!(emu.add_capabilities(&capabilities).is_ok());
        assert_eq!(true, emu.capabilities.can_suspend);
        assert_eq!(true, emu.capabilities.can_get_bytecodes);
    }
}

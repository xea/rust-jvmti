extern crate jvmti;
extern crate libc;

#[cfg(test)]
mod tests {

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
}

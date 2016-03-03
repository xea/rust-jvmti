extern crate jvmti;

#[cfg(test)]
mod tests {

    use jvmti::agent::Agent;
    use jvmti::emulator::JVMEmulator;
    use jvmti::version::VersionNumber;

    #[test]
    fn agents_are_fucking_even_working() {
        let emulator = JVMEmulator;
        let agent = Agent::new_from(Box::new(emulator));
        let version = agent.get_version();

        assert_eq!(0x7FFF, version.major_version);
    }

    #[test]
    fn agents_are_initialized_with_empty_capabilities() {
        let emulator = JVMEmulator;
        let agent = Agent::new_from(Box::new(emulator));

        assert_eq!(false, agent.capabilities.can_suspend);
        assert_eq!(false, agent.capabilities.can_pop_frame);
        assert_eq!(false, agent.capabilities.can_generate_monitor_events);
        assert_eq!(false, agent.capabilities.can_generate_method_entry_events);
        assert_eq!(false, agent.capabilities.can_generate_method_exit_events);
        assert_eq!(false, agent.capabilities.can_generate_vm_object_alloc_events);
        assert_eq!(false, agent.capabilities.can_generate_breakpoint_events);
        // TODO this test is not complete at all. surprisingly
    }

    #[test]
    fn agents_respond_to_shutdown() {
        let emulator = JVMEmulator;
        let agent = Agent::new_from(Box::new(emulator));
        agent.shutdown();
    }

    #[test]
    fn agents_provide_with_version_numbers() {
        let emulator = JVMEmulator;
        let agent = Agent::new_from(Box::new(emulator));
        let version = agent.get_version();
        let unknown_version = VersionNumber::unknown();
        assert_eq!(unknown_version.major_version, version.major_version);
        assert_eq!(unknown_version.minor_version, version.minor_version);
        assert_eq!(unknown_version.micro_version, version.micro_version);
    }
}

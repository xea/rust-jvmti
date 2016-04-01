extern crate jvmti;

#[cfg(test)]
mod tests {

    use jvmti::agent::Agent;
    use jvmti::emulator::JVMEmulator;
    use jvmti::runtime::MethodInvocationEvent;
    use jvmti::thread::Thread;
    use jvmti::version::VersionNumber;

    #[test]
    fn agents_are_fucking_even_working() {
        let emulator = JVMEmulator::new();
        let agent = Agent::new_from(Box::new(emulator));
        let version = agent.get_version();

        assert_eq!(0x7FFF, version.major_version);
    }

    #[test]
    fn agents_are_initialized_with_empty_capabilities() {
        let emulator = JVMEmulator::new();
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
        let emulator = JVMEmulator::new();
        let agent = Agent::new_from(Box::new(emulator));
        agent.shutdown();
    }

    #[test]
    fn agents_provide_with_version_numbers() {
        let emulator = JVMEmulator::new();
        let agent = Agent::new_from(Box::new(emulator));
        let version = agent.get_version();
        let unknown_version = VersionNumber::unknown();
        assert_eq!(unknown_version.major_version, version.major_version);
        assert_eq!(unknown_version.minor_version, version.minor_version);
        assert_eq!(unknown_version.micro_version, version.micro_version);
    }

    #[test]
    fn callbacks_trigger_capabilities() {
        let emulator = JVMEmulator::new();
        let mut agent = Agent::new_from(Box::new(emulator));

        agent.on_method_entry(Some(test_on_method_entry));
        assert_eq!(true, agent.capabilities.can_generate_method_entry_events);
        agent.on_method_entry(None);
        assert_eq!(false, agent.capabilities.can_generate_method_entry_events);

        assert_eq!(false, agent.capabilities.can_generate_monitor_events);
        agent.on_monitor_wait(Some(test_on_monitor_events));
        assert_eq!(true, agent.capabilities.can_generate_monitor_events);
        agent.on_monitor_waited(Some(test_on_monitor_events));
        assert_eq!(true, agent.capabilities.can_generate_monitor_events);
        agent.on_monitor_contended_enter(Some(test_on_monitor_events));
        assert_eq!(true, agent.capabilities.can_generate_monitor_events);
        agent.on_monitor_wait(None);
        assert_eq!(true, agent.capabilities.can_generate_monitor_events);
        agent.on_monitor_waited(None);
        assert_eq!(true, agent.capabilities.can_generate_monitor_events);
        agent.on_monitor_contended_enter(None);
    }

    #[allow(unused_variables)]
    fn test_on_method_entry(event: MethodInvocationEvent) {
        // this is a callback method for testing purposes
    }

    #[allow(unused_variables)]
    fn test_on_monitor_events(thread: Thread) {
        // this is a callback method for testing purposes
    }
}

extern crate jvmti;

#[cfg(test)]
mod tests {

    use jvmti::agent::Agent;
    use jvmti::emulator::Emulator;
    use jvmti::native::JavaVMPtr;

    fn get_vm_ptr() -> JavaVMPtr {
        let mut emulator = Emulator::new();
        let mut eptr: *mut Emulator = &mut emulator;
        Emulator::transmute(&mut eptr)
    }

    #[test]
    fn agents_can_be_instantiated_using_new() {
        let agent = Agent::new(get_vm_ptr());
        assert_eq!(0xBABE, agent.get_version());
    }

    #[test]
    fn agent_responds_to_shutdown() {
        let agent = Agent::new(get_vm_ptr());
        agent.shutdown();
    }

    #[test]
    fn agent_can_register_event_callback_method() {
        let mut agent = Agent::new(get_vm_ptr());

        assert_eq!(true, agent.on_method_entry(Some(test_callback_on_method_entry)));
    }

    #[test]
    fn can_update_capabilities() {
        let agent = Agent::new(get_vm_ptr());

        agent.update();
    }

    #[test]
    fn get_version_returns_enviroment_version_number() {
        let agent = Agent::new(get_vm_ptr());

        let version = agent.get_version();

    }

    fn test_callback_on_method_entry() {

    }
}

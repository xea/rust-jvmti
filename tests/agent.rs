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
        let mut emulator = Emulator::new();
        let mut eptr: *mut Emulator = &mut emulator;
        let vm_ptr = Emulator::transmute(&mut eptr);
        let agent = Agent::new(vm_ptr);
        let version = agent.get_version();
        assert_eq!(0x20, version.micro_version);
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
    #[ignore]
    fn get_version_returns_enviroment_version_number() {
        println!("asdfasdfasdf");
        let mut emulator = Emulator::new();
        let mut eptr: *mut Emulator = &mut emulator;
        let vm_ptr: JavaVMPtr = Emulator::transmute(&mut eptr);
        let agent = Agent::new(vm_ptr);

        let version = agent.get_version();

        assert_eq!(0x3FA, version.major_version);
        assert_eq!(0x30, version.minor_version);
        assert_eq!(0x20, version.micro_version);
    }

    fn test_callback_on_method_entry() {

    }
}

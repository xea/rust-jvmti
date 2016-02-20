extern crate agent;
extern crate jvmti;
extern crate libc;


#[cfg(test)]
mod tests {

    use jvmti::emulator::Emulator;
    use agent::Agent;

    #[test]
    fn agent_should_be_created_using_jvm_pointers() {
        let mut emulator = Emulator::new();
        let mut eptr: *mut Emulator = &mut emulator;
        let vmptr = Emulator::transmute(&mut eptr);

        let agent = Agent::new(vmptr);
        assert_eq!(0xBABE, agent.get_version());
    }
    

}

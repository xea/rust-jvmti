extern crate jvmti;

#[cfg(test)]
mod tests {

    use jvmti::emulator::Emulator;
    use jvmti::environment::jvm::JVMAgent;
    use jvmti::native::JavaVMPtr;

    #[test]
    #[allow(unused_variables)]
    fn new_creates_a_new_jvm_instance() {
        let jvm = get_vm_ptr();
        let jvm_agent = JVMAgent::new(jvm);
    }

    fn get_vm_ptr() -> JavaVMPtr {
        let mut emulator = Emulator::new();
        let mut eptr: *mut Emulator = &mut emulator;
        Emulator::transmute(&mut eptr)
    }
}

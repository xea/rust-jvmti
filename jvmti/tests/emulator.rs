extern crate jvmti;

#[cfg(test)]
mod tests {

    use jvmti::emulator::Emulator;
    use std::ptr;

    #[test]
    fn emulators_can_be_transmuted_into_jvm_ptrs() {
        let mut emulator = Emulator::new();
        let mut eptr: *mut Emulator = &mut emulator;
        let vmptr = Emulator::transmute(&mut eptr);

        unsafe {
            assert_eq!(0xCAFE, (**vmptr).GetEnv.unwrap()(vmptr, ptr::null_mut(), 0));
        }

    }
}

extern crate jvmti;

mod class_stream;
mod constants;

#[cfg(test)]
mod tests {

    use jvmti::bytecode::ClassReader;
    use jvmti::bytecode::constants::*;

    fn bytecode_simple() -> &'static [u8] {
        include_bytes!("../../Simple.class")
    }

    //fn bytecode_test() -> &'static [u8] {
    //    include_bytes("../../Test.class")
    //}

    #[test]
    fn basic_test() {
        let result = ClassReader::parse_bytes(&bytecode_simple().to_vec());

        assert!(result.is_ok());

        let classfile = result.ok().unwrap();

        assert_eq!(0, classfile.minor_version);
        assert_eq!(52, classfile.major_version);
        assert_eq!(39, classfile.constant_pool.len());
        assert!(classfile.access_flags.get(ClassAccessFlag::Public));
        assert!(classfile.access_flags.get(ClassAccessFlag::Super));
        assert_eq!(10, classfile.this_class.constant_idx);
        // Has no super class
        assert_eq!(0, classfile.super_class.constant_idx);
        assert_eq!(1, classfile.fields.len());
    }
}

extern crate jvmti;

mod collections;
mod constant;
mod stream;

#[cfg(test)]
mod tests {

    mod class_reader {

        use jvmti::bytecode::classfile::*;
        use jvmti::bytecode::constant::*;
        use jvmti::bytecode::ClassReader;

        fn simple_class() -> &'static [u8] {
            include_bytes!("../../Simple.class")
        }

        fn test_class() -> &'static [u8] {
            include_bytes!("../../Test.class")
        }

        #[test]
        fn read_bytes_reads_simple_class_version_number_correctly() {
            let result = ClassReader::read_array(simple_class());

            assert!(result.is_ok(), format!("Error: {}", result.err().unwrap()));
            let class = result.ok().unwrap();

            assert_eq!(52, class.version.major_version);
            assert_eq!(0, class.version.minor_version);
        }

        #[test]
        fn read_bytes_reads_simple_constant_pool_correctly() {
            let result = ClassReader::read_array(simple_class());

            assert!(result.is_ok(), format!("Error: {}", result.err().unwrap()));
            let class = result.ok().unwrap();

            assert!(class.constant_pool.get(&class.this_class).is_some());

            let this_class: &Constant = class.constant_pool.get(&class.this_class).unwrap();

            match this_class {
                &Constant::Class(idx) => {
                    assert!(class.constant_pool.get(&ConstantPoolIndex::of(idx)).is_some(), format!("Referenced constant missing: {}", idx));
                },
                _ => assert!(false, format!("{:?}", this_class))
            }

        }

        #[test]
        fn read_bytes_reads_simple_access_flags_correctly() {
            let result = ClassReader::read_array(simple_class());

            assert!(result.is_ok(), format!("Error: {}", result.err().unwrap()));
            let class = result.ok().unwrap();

            assert!(class.access_flags.has_flag(ClassAccessFlags::PUBLIC as u16));
            assert!(class.access_flags.has_flag(ClassAccessFlags::SUPER as u16));
            assert!(!class.access_flags.has_flag(ClassAccessFlags::INTERFACE as u16));
            assert!(!class.access_flags.has_flag(ClassAccessFlags::ENUM as u16));
        }

        #[test]
        fn read_bytes_reads_interfaces_correctly() {
            let result = ClassReader::read_array(simple_class());

            assert!(result.is_ok(), format!("Error: {}", result.err().unwrap()));
            let class = result.ok().unwrap();

            assert_eq!(0, class.interfaces.len());
        }

        #[test]
        fn read_bytes_reads_test_class_correctly() {
            let reader = ClassReader::read_array(test_class());

            assert!(reader.is_ok());
        }

    }
}

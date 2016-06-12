extern crate jvmti;

mod collections;
mod stream;

#[cfg(test)]
mod tests {

    mod class_reader {

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

            assert!(result.is_ok());
            let class = result.ok().unwrap();

            assert_eq!(52, class.version.major_version);
            assert_eq!(0, class.version.minor_version);
        }

        #[test]
        fn read_bytes_reads_test_class_correctly() {
            let reader = ClassReader::read_array(test_class());

            assert!(reader.is_ok());
        }
    }
}

extern crate jvmti;

#[cfg(test)]
mod tests {

    use jvmti::bytecode::ClassReader;
    use jvmti::bytecode::ConstantType;

    #[test]
    fn read_version_number_parses_major_and_minor_version_numbers() {
        let bytes: Vec<u8> = vec![];
        let mut reader = ClassReader::new(&bytes);

        assert!(reader.read_version_number().is_none());

        let bytes = vec![0x00, 0xC0, 0x00, 0x00];
        let mut reader = ClassReader::new(&bytes);

        let version = reader.read_version_number();

        assert!(version.is_some());

        let (major, minor) = version.expect("Version numbers should be here");

        assert_eq!(0xc0, major);
        assert_eq!(0x00, minor);
    }

    #[test]
    fn read_constant_pool_reads_only_valid_constant_pools() {
        let bytes: Vec<u8> = vec![];
        let mut reader = ClassReader::new(&bytes);

        let cp = reader.read_constant_pool();

        assert!(cp.is_none());

        //               - CP size -  TAG   - LENGTH -
        let bytes = vec![ 0x00, 0x01, 0x01, 0x00, 0x00 ];
        let mut reader = ClassReader::new(&bytes);

        let cp = reader.read_constant_pool();

        assert!(cp.is_some());

        let ocp = cp.unwrap();
        assert_eq!(0, ocp.len());

        //               - CP size -  TAG   - LENGTH -
        let bytes = vec![ 0x00, 0x02, 0x01, 0x00, 0x00 ];
        let mut reader = ClassReader::new(&bytes);

        let cp = reader.read_constant_pool();

        assert!(cp.is_some());

        let ocp = cp.unwrap();
        assert_eq!(1, ocp.len());

        //               - CP size -  TAG   - LENGTH -  - A     B     C -
        let bytes = vec![ 0x00, 0x02, 0x01, 0x00, 0x03, 0x40, 0x41, 0x42 ];
        let mut reader = ClassReader::new(&bytes);

        let cp = reader.read_constant_pool();

        assert!(cp.is_some());

        let ocp = cp.unwrap();
        assert_eq!(1, ocp.len());

        //               - CP size -  TAG   - LENGTH -  - A     B    C -   TAG NAME-INDEX
        let bytes = vec![ 0x00, 0x03, 0x01, 0x00, 0x03, 0x40, 0x41, 0x42, 0x07, 0x01, 0x02 ];
        let mut reader = ClassReader::new(&bytes);

        let cp = reader.read_constant_pool();

        assert!(cp.is_some());

        let ocp = cp.unwrap();
        assert_eq!(2, ocp.len());
    }

    #[test]
    fn read_constant_pool_reads_two_entries_less_than_specified_size() {
        let bytes: Vec<u8> = vec![ 0x00, 0x00 ];
        let mut reader = ClassReader::new(&bytes);

        let cp = reader.read_constant_pool();
        assert!(cp.is_some());
        assert_eq!(0, cp.unwrap().len());

        let bytes: Vec<u8> = vec![ 0x00, 0x01 ];
        let mut reader = ClassReader::new(&bytes);

        let cp = reader.read_constant_pool();
        assert!(cp.is_some());
        assert_eq!(0, cp.unwrap().len());

        let bytes: Vec<u8> = vec![ 0x00, 0x02, 0x01, 0x00, 0x00 ];
        let mut reader = ClassReader::new(&bytes);

        let cp = reader.read_constant_pool();
        assert!(cp.is_some());
        assert_eq!(1, cp.unwrap().len());
    }

    #[test]
    fn constant_type_parses_utf8_byte_sequences_correctly() {
        let oct = ConstantType::parse(1, &vec![ 0x00, 0x00 ]);
        assert!(oct.is_some());
        let value = oct.unwrap();
        match value {
            ConstantType::Utf8 { length: clen, bytes: _ } => {
                assert_eq!(2, value.length());
                assert_eq!(0, clen);
            },
            _ => panic!()
        }
        // ---
        let oct = ConstantType::parse(1, &vec![ 0x00, 0x01, 0x40 ]);
        assert!(oct.is_some());
        let value = oct.unwrap();
        assert_eq!(3, value.length());
        match value {
            ConstantType::Utf8 { length: clen, bytes: cbytes } => {
                assert_eq!(1, clen);
                assert_eq!(vec![ 0x40 ], cbytes);
            },
            _ => panic!()
        }
        // ---
        let oct = ConstantType::parse(1, &vec![ 0x00, 0x01, 0x40, 0x13 ]);
        assert!(oct.is_some());
        let value = oct.unwrap();
        assert_eq!(3, value.length());
        match value {
            ConstantType::Utf8 { length: clen, bytes: cbytes } => {
                assert_eq!(1, clen);
                assert_eq!(vec![ 0x40 ], cbytes);
            },
            _ => panic!()
        }
        // ---
        let oct = ConstantType::parse(1, &vec![ 0x00, 0x02, 0x40, 0x13 ]);
        assert!(oct.is_some());
        let value = oct.unwrap();
        assert_eq!(4, value.length());
        match value {
            ConstantType::Utf8 { length: clen, bytes: cbytes } => {
                assert_eq!(2, clen);
                assert_eq!(vec![ 0x40, 0x13 ], cbytes);
            },
            _ => panic!()
        }
    }

}

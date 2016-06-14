extern crate jvmti;

#[cfg(test)]
mod tests {

    use jvmti::bytecode::stream::ClassInputStream;
    use jvmti::bytecode::stream::ClassOutputStream;
    use jvmti::bytecode::stream::WriteChunks;
    use jvmti::bytecode::classfile::ClassfileVersion;

    #[test]
    fn write_u8_writes_a_single_byte_at_the_end_of_the_stream() {
        let mut os: ClassOutputStream = ClassOutputStream::new();
        os.write_u8(14);

        assert_eq!(vec![ 14 ], os.to_vec());
    }

    #[test]
    fn write_u16_writes_two_bytes_at_the_end_of_the_stream() {
        let mut os: ClassOutputStream = ClassOutputStream::new();
        os.write_u16(0xABCD);

        assert_eq!(vec![ 0xAB, 0xCD ], os.to_vec());
    }

    #[test]
    fn write_u64_writes_eight_bytes_at_the_end_of_the_stream() {
        let mut os: ClassOutputStream = ClassOutputStream::new();
        os.write_u64(0x1122334455667788);

        assert_eq!(vec![ 0x11, 0x22, 0x33, 0x44, 0x55, 0x66, 0x77, 0x88 ], os.to_vec());
    }

    #[test]
    fn write_magic_bytes_writes_cafebabe() {
        let mut os: ClassOutputStream = ClassOutputStream::new();
        os.write_magic_bytes();

        assert_eq!(vec![ 0xCA, 0xFE, 0xBA, 0xBE ], os.to_vec());
    }

    #[test]
    fn write_version_number_writes_the_provided_version_number() {
        let versions: Vec<(ClassfileVersion, Vec<u8>)> = vec![
            (ClassfileVersion::new(52, 0), vec![ 0x00, 0x00, 0x00, 0x34 ]),
            (ClassfileVersion::new(0, 0), vec![ 0x00, 0x00, 0x00, 0x0 ]),
            (ClassfileVersion::new(256, 256), vec![ 0x01, 0x0, 0x01, 0x0 ]),
        ];

        for (version, expected) in versions {
            let mut os: ClassOutputStream = ClassOutputStream::new();
            os.write_version_number(&version);

            assert_eq!(expected, os.to_vec());

        }
    }

    #[test]
    fn read_constant_pool_reads_exactly_the_desired_number_of_constants() {
        let inputs: Vec<(Vec<u8>, usize, usize)> = vec![
            // Empty constant pool
            (vec![ 0, 1 ], 0, 0),
            // Empty constant pool with overflowing bytes
            (vec![ 0, 1, 0xf, 0xf, 0xf ], 0, 3),
            // Single integer constant
            (vec![ 0, 2, 3, 1, 2, 3, 4 ], 1, 0),
            // Single integer constant with overflowing bytes
            (vec![ 0, 2, 3, 1, 2, 3, 4, 0xf, 0xf, 0xf ], 1, 3),
            // Two integer constants
            (vec![ 0, 3, 3, 1, 1, 1, 1, 3, 2, 2, 2, 2 ], 2, 0),
            // One long constant
            (vec![ 0, 3, 5, 1, 1, 1, 1, 1, 1, 1, 1 ], 2, 0),
            // Two long constants
            (vec![ 0, 5, 5, 1, 1, 1, 1, 1, 1, 1, 1, 5, 2, 2, 2, 2, 2, 2, 2, 2 ], 4, 0),
            // An integer and a long constant
            (vec![ 0, 4, 3, 1, 1, 1, 1, 5, 2, 2, 2, 2, 2, 2, 2, 2 ], 3, 0),
            // A long and an integer constant
            (vec![ 0, 4, 5, 1, 1, 1, 1, 1, 1, 1, 1, 3, 2, 2, 2, 2 ], 3, 0),
            // long, int, long
            (vec![ 0, 6, 5, 1, 1, 1, 1, 1, 1, 1, 1, 3, 2, 2, 2, 2, 5, 3, 3, 3, 3, 3, 3, 3, 3 ], 5, 0),
        ];

        for (bytes, expected_count, expected_avail) in inputs {
            let is: ClassInputStream = ClassInputStream::from_vec(&bytes);

            let result = is.read_constant_pool();

            assert!(result.is_ok(), format!("Bytes {:?} Error: {:?}", bytes, result.err().unwrap()));

            let cp = result.ok().unwrap();

            //assert_eq!(expected_count, cp.len());
            assert!(expected_count == cp.len(), format!("Bytes {:?} Expected: {} Found: {}", bytes, expected_count, cp.len()));
            assert!(expected_avail == is.available(), format!("Bytes {:?} Expected: {} Available: {}", bytes, expected_avail, is.available()));
        }
    }
}

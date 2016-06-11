extern crate jvmti;

#[cfg(test)]
mod tests {

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
}

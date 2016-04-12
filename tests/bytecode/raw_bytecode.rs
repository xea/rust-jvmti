extern crate jvmti;

#[cfg(test)]
mod tests {

    use jvmti::bytecode::RawBytecode;
    use jvmti::bytecode::ConstantType;

    fn test_bytecode() -> Vec<u8> {
        vec![ 0xCA, 0xFE, 0xBA, 0xBE, 0x12, 0x34, 0x56, 0x78, 0, 1, 7, 0, 0, 0, 0 ]
    }

    #[test]
    fn read_constant_pool_rejects_empty_constant_pools() {
        let vc: Vec<u8> = vec![ 0x00, 0x00 ];
        let vb = vc.as_slice();
        let mut bc = RawBytecode::default();

        let result = RawBytecode::read_constant_pool(vb, &mut bc);

        assert!(result.is_err());
    }

    #[test]
    fn read_constant_pool_rejects_incomplete_constant_pools() {
        let vc: Vec<u8> = vec![ 0x00, 0x01, 0x07 ];
        let vb = vc.as_slice();
        let mut bc = RawBytecode::default();

        let result = RawBytecode::read_constant_pool(vb, &mut bc);

        assert!(result.is_err());
    }

    #[test]
    fn read_constant_pool_accepts_complete_constant_pools() {
        let vc: Vec<u8> = vec![ 0x00, 0x01, 0x07, 0x98, 0x76 ];
        let vb = vc.as_slice();
        let mut bc = RawBytecode::default();

        let result = RawBytecode::read_constant_pool(vb, &mut bc);

        assert_eq!(None, result.err());
        assert_eq!(1, bc.constant_pool.len());

        let cpi = bc.constant_pool.first().unwrap();

        match cpi.tag {
            ConstantType::Class { name_index } => {
                assert_eq!(0x9876, name_index);
            },
            _ => panic!("Wrong constant type")
        }
    }

    #[test]
    fn raw_bytecode_parses_version_numbers() {
        let vc: Vec<u8> = test_bytecode();
        let vs = vc.as_slice().as_ptr();

        let result = RawBytecode::from_raw_bytes(vs, vc.len() as i32);

        assert!(result.is_ok());
        let bc = result.ok().unwrap();

        assert_eq!(0x1234, bc.minor_version);
        assert_eq!(0x5678, bc.major_version);
    }

    #[test]
    fn raw_bytecode_parses_beyond_null_characters() {
        let vc: Vec<u8> = vec![ 0xCA, 0xFE, 0xBA, 0xBE, 0, 1, 6, 0, 0, 1, 7, 0, 0 ];
        let vs = vc.as_slice().as_ptr();

        let result = RawBytecode::from_raw_bytes(vs, vc.len() as i32);

        assert!(result.is_ok());

        let bc = result.ok().unwrap();

        assert_eq!(bc.minor_version, 0x01);
        assert_eq!(bc.major_version, 0x0600);
    }
}

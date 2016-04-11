extern crate jvmti;

#[cfg(test)]
mod tests {

    use jvmti::bytecode::RawBytecode;

    #[test]
    fn raw_bytecode_parses_version_numbers() {
        let vc: Vec<u8> = vec![ 0xCA, 0xFE, 0xBA, 0xBE, 0x12, 0x34, 0x56, 0x78, 7, 8, 9, 0 ];
        let vs = vc.as_slice().as_ptr();

        let result = RawBytecode::from_raw_bytes(vs, 12);

        assert!(result.is_ok());

        let bc = result.ok().unwrap();

        assert_eq!(0x1234, bc.minor_version);
        assert_eq!(0x5678, bc.major_version);

    }

    #[test]
    fn raw_bytecode_parses_beyond_null_characters() {
        let vc: Vec<u8> = vec![ 0xCA, 0xFE, 0xBA, 0xBE, 0, 1, 6, 0, 7, 8, 9, 0 ];
        let vs = vc.as_slice().as_ptr();

        let result = RawBytecode::from_raw_bytes(vs, 12);

        assert!(result.is_ok());

        let bc = result.ok().unwrap();

        assert_eq!(bc.minor_version, 0x01);
        assert_eq!(bc.major_version, 0x0600);
    }
}

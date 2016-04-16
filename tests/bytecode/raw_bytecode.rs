extern crate jvmti;

#[cfg(test)]
mod tests {

    use jvmti::bytecode::Classfile;
    use jvmti::bytecode::ClassfileReader;
    use jvmti::bytecode::ConstantType;


    #[allow(dead_code)]
    fn debug_result(r: Result<Classfile, String>) {
        let err = r.err().unwrap();

        assert_eq!("_______???".to_string(), err);
    }

    fn bytecode_simple() -> &'static [u8] {
        include_bytes!("../../Simple.class")
    }

/*
    fn bytecode_complex() -> &'static [u8] {
        include_bytes!("../../Test.class")
    }
    */

    fn bytecode_bad_magic() -> &'static [u8] {
        static BAD_MAGIC: [u8; 4] = [ 0xCA, 0xFE, 0xBB, 0xBF ];

        &BAD_MAGIC
    }

    #[test]
    fn read_magic_rejects_invalid_magic_numbers() {
        let result = ClassfileReader::read_magic(bytecode_bad_magic());

        assert!(result.is_err());
    }

    #[test]
    fn read_magic_accepts_valid_magic_numbers() {
        let result = ClassfileReader::read_magic(bytecode_simple());

        assert!(result.is_ok());
    }

    #[test]
    fn from_bytes_reads_version_numbers() {
        let result = ClassfileReader::from_bytes(bytecode_simple());

        assert!(result.is_ok());

        let cf = result.ok().unwrap();

        assert_eq!(52, cf.major_version);
        assert_eq!(0, cf.minor_version);
    }

    #[test]
    fn read_constant_pool_reads() {
        let result1 = ClassfileReader::read_constant_pool(&[ 0x00, 0x02, 0x07, 0x01, 0x02 ]);
        let result2 = ClassfileReader::read_constant_pool(&[ 0x00, 0x04, 0x07, 0x01, 0x02, 0x08, 0x03, 0x04, 0x10, 0x05, 0x06 ]);

        assert!(result1.is_ok());
        assert!(result2.is_ok());

        let (cpm1, s1) = result1.ok().unwrap();
        let (cpm2, s2) = result2.ok().unwrap();

        assert_eq!(5, s1);
        assert_eq!(11, s2);
        assert!(cpm1.constant_pool.is_some());
        assert!(cpm2.constant_pool.is_some());

        let cp1 = cpm1.constant_pool.unwrap();
        let cp2 = cpm2.constant_pool.unwrap();

        assert_eq!(2, cp1.len());
        assert_eq!(4, cp2.len());
    }


    #[test]
    fn read_constant_pool_info_recognises_class_info() {
        let result = ClassfileReader::read_constant_pool_info(&[ 0x07, 0x01, 0x02 ]);

        assert!(result.is_ok());

        let w = result.ok().unwrap();
        let (r, s) = w;

        assert!(match r {
            ConstantType::Class { name_index } => match name_index {
                0x102 => true,
                _ => false
            },
            _ => false
        });

        assert_eq!(3, s);
    }

    #[test]
    fn read_access_flags_recognises_access_flags() {
        let result = ClassfileReader::from_bytes(bytecode_simple());

        assert!(result.is_ok());

        let w = result.ok().unwrap();

        assert_eq!(0x21, w.access_flags.raw_flag());
        assert_eq!(true, w.access_flags.is_public());
        assert_eq!(true, w.access_flags.is_super());
        assert_eq!(false, w.access_flags.is_annotation());
        assert_eq!(false, w.access_flags.is_final());
        assert_eq!(false, w.access_flags.is_abstract());
        assert_eq!(false, w.access_flags.is_interface());
        assert_eq!(false, w.access_flags.is_synthetic());
        assert_eq!(false, w.access_flags.is_enum());
    }

    #[test]
    fn read_interfaces_reads_interfaces_count_number_of_interfaces() {
        let result = ClassfileReader::read_interfaces(&[ 0x00, 0x04, 0x00, 0x01, 0x00, 0x02, 0x00, 0x03, 0x00, 0x04]);

        assert!(result.is_ok());

        let f = result.ok().unwrap().0;
        let i = f.interfaces.unwrap();

        assert_eq!(4, i.len());
        assert_eq!(1, i[0].id);
        assert_eq!(2, i[1].id);
        assert_eq!(3, i[2].id);
        assert_eq!(4, i[3].id);
    }
}

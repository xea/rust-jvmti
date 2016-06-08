extern crate jvmti;

#[cfg(test)]
mod tests {

    use jvmti::bytecode::class_stream::ClassStream;
    use jvmti::bytecode::constants::ConstantType;

    #[test]
    fn read_magic_bytes_accepts_only_valid_magic_bytes() {
        let valid_magic = vec![ 0xCA, 0xFE, 0xBA, 0xBE ];
        let invalid_magics = vec![
            vec![],
            vec![ 0x00, ],
            vec![ 0xCA, 0xFE, 0xBA ],
            vec![ 0xCA, 0xFE, 0xBE, 0xBA ]
        ];

        assert_eq!(true, ClassStream::new(&valid_magic).read_magic_bytes());

        for magic in invalid_magics {
            assert_eq!(false, ClassStream::new(&magic).read_magic_bytes());
        }
    }

    #[test]
    fn read_version_number_accepts_only_valid_version_numbers() {
        let valid_versions = vec![
            vec![ 0x00, 0x10, 0x00, 0x30 ],
            vec![ 0x00, 0x00, 0x00, 0x34 ], // Java 8 class file version
            vec![ 0xFF, 0xFF, 0x00, 0x27 ]
        ];

        let invalid_versions= vec![
            vec![ ],
            vec![ 0x00 ],
            vec![ 0x00, 0x01 ],
            vec![ 0x00, 0x02, 0x03 ]
        ];

        for version in valid_versions {
            let ovn = ClassStream::new(&version).read_version_number();
            assert!(ovn.is_some());
            let vn = ovn.unwrap();
            assert!(vn.1 >= 0x27 && vn.1 <= 0x34);
        }

        for version in invalid_versions {
            let ovn = ClassStream::new(&version).read_version_number();
            assert!(ovn.is_none());
        }
    }

    #[test]
    fn read_constant_pool_doesnt_accept_empty_bytes() {
        let empty_cp = vec![];
        let r = ClassStream::new(&empty_cp).read_constant_pool();
        assert!(r.is_none());
    }

    #[test]
    fn read_constant_pool_doesnt_invalid_pools() {
        let invalid_pools = vec![
            vec![ 0x00 ], // too short
            vec![ 0x00, 0x02 ], // missing first item
        ];

        for pool in invalid_pools {
            let r = ClassStream::new(&pool).read_constant_pool();
            assert!(r.is_none());
        }
    }

    #[test]
    fn read_constant_pool_reads_empty_pools() {
        let empty_pool = vec![ 0x00, 0x01 ];
        let or = ClassStream::new(&empty_pool).read_constant_pool();
        assert!(or.is_some());
        let r = or.unwrap();

        assert_eq!(0, r.len());
    }

    #[test]
    fn read_constant_pool_handles_long_entries_correctly() {
        //                           INT
        let pool = vec![ 0x00, 0x02, 0x03, 0x01, 0x02, 0x03, 0x04 ];
        let or = ClassStream::new(&pool).read_constant_pool();
        assert!(or.is_some());
        let r = or.unwrap();

        assert_eq!(1, r.len());

        match r[0] {
            ConstantType::Integer { bytes: b } => assert_eq!(0x01020304, b),
            _ => assert!(false)
        }

        //                           LONG
        let pool = vec![ 0x00, 0x02, 0x05, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08 ];
        let or = ClassStream::new(&pool).read_constant_pool();
        assert!(or.is_some());
        let r = or.unwrap();

        assert_eq!(1, r.len());

        match r[0] {
            ConstantType::Long { high_bytes: h, low_bytes: l } => {
                assert_eq!(0x01020304, h);
                assert_eq!(0x05060708, l);
            },
            _ => assert!(false)
        }

        //                           INT                           LONG
        let pool = vec![ 0x00, 0x03, 0x03, 0x10, 0x20, 0x30, 0x40, 0x05, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08 ];
        let or = ClassStream::new(&pool).read_constant_pool();
        assert!(or.is_some());
        let r = or.unwrap();

        assert_eq!(2, r.len());

        match r[0] {
            ConstantType::Integer { bytes: b } => assert_eq!(0x10203040, b),
            _ => assert!(false)
        }

        match r[1] {
            ConstantType::Long { high_bytes: h, low_bytes: l } => {
                assert_eq!(0x01020304, h);
                assert_eq!(0x05060708, l);
            },
            _ => assert!(false)
        }
    }

    #[test]
    fn read_constant_pool_should_ignore_entries_off_limit_by_long_entries() {
        // The second INT entry should be ignored in this test because the LONG entry takes up both indexes
        //                           LONG                                                   INT
        let pool = vec![ 0x00, 0x03, 0x05, 0x10, 0x20, 0x30, 0x40, 0x50, 0x60, 0x70, 0x80, 0x03, 0x04, 0x05, 0x06, 0x07 ];
        let or = ClassStream::new(&pool).read_constant_pool();
        assert!(or.is_some());
        let r = or.unwrap();

        assert_eq!(1, r.len());

        match r[0] {
            ConstantType::Long { high_bytes: h, low_bytes: l } => {
                assert_eq!(0x10203040, h);
                assert_eq!(0x50607080, l);
            },
            _ => assert!(false)
        }
    }

    #[test]
    fn it_should_parse_multiple_entries() {
        let pool = vec![ 0, 5, 0x03, 1, 1, 1, 1, 0x07, 2, 2, 0x08, 3, 3, 0x03, 4, 4, 4, 4 ];

        let or = ClassStream::new(&pool).read_constant_pool();
        assert!(or.is_some());
        let r = or.unwrap();

        assert_eq!(4, r.len());

        match r[0] {
            ConstantType::Integer { bytes: b } => assert_eq!(0x01010101, b),
            _ => assert!(false)
        }
        match r[1] {
            ConstantType::Class { name_index: n } => assert_eq!(0x0202, n),
            _ => assert!(false)
        }
        match r[2] {
            ConstantType::String { string_index: s } => assert_eq!(0x0303, s),
            _ => assert!(false)
        }
        match r[3] {
            ConstantType::Integer { bytes: b } => assert_eq!(0x04040404, b),
            _ => assert!(false)
        }
    }

    #[test]
    fn it_should_parse_multiple_entries_with_long_items() {
        let pool = vec![ 0, 7, 0x03, 1, 1, 1, 1, 0x05, 0, 0, 0, 0, 0, 0, 0, 0, 0x07, 2, 2, 0x08, 3, 3, 0x03, 4, 4, 4, 4 ];

        let or = ClassStream::new(&pool).read_constant_pool();
        assert!(or.is_some());
        let r = or.unwrap();

        assert_eq!(5, r.len());

        match r[0] {
            ConstantType::Integer { bytes: b } => assert_eq!(0x01010101, b),
            _ => assert!(false)
        }
        match r[1] {
            ConstantType::Long { high_bytes: h, low_bytes: _ } => assert_eq!(0, h),
            _ => assert!(false)
        }
        match r[2] {
            ConstantType::Class { name_index: n } => assert_eq!(0x0202, n),
            _ => assert!(false)
        }
        match r[3] {
            ConstantType::String { string_index: s } => assert_eq!(0x0303, s),
            _ => assert!(false)
        }
        match r[4] {
            ConstantType::Integer { bytes: b } => assert_eq!(0x04040404, b),
            _ => assert!(false)
        }
    }

    #[test]
    fn read_field_should_parse_fields() {
        let bytes = vec![ 0, 0, 0, 1, 0, 2, 0, 1, 254];
        let or = ClassStream::new(&bytes).read_field();

        assert!(or.is_some());
        let r = or.unwrap();
    }

}

extern crate jvmti;

#[cfg(test)]
mod tests {

    use jvmti::bytecode::class_stream::ClassStream;

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
    }
}

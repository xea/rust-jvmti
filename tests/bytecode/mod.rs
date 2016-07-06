extern crate jvmti;

#[cfg(test)]
mod tests {

    use jvmti::bytecode::*;
    use std::io::{ Cursor, Read, Write, Error };

    #[test]
    fn test_read_n() {
        let mut target: Vec<u8> = vec![ 1, 2, 3, 4, 5, 6, 7, 8, 9 ];
        let mut cursor = Cursor::new(&mut target);
        let mut reader = BlockReader::new(&mut cursor);

        let r1 = reader.read_n(4);
        assert_eq!(vec![1, 2, 3, 4], r1.ok().unwrap());

        let r2 = reader.read_n(4);
        assert_eq!(vec![5, 6, 7, 8], r2.ok().unwrap());

        let r3 = reader.read_u8();
        assert_eq!(9, r3.ok().unwrap());

        assert!(reader.read_u8().is_err());
    }

    #[test]
    fn test_read_write_roundtrip() {
        let class: Classfile = Classfile {
            version: ClassfileVersion::new(52, 0),
            constant_pool: ConstantPool::new(vec![
                Constant::Placeholder,
                Constant::Integer(14),
                Constant::Long(5),
                Constant::Placeholder,
                Constant::Utf8("AAAAAA".to_string().into_bytes()),
                Constant::Class(ConstantPoolIndex::new(3))
                ]),
            access_flags: AccessFlags::of(0x000F),
            this_class: ConstantPoolIndex::new(1),
            super_class: ConstantPoolIndex::new(2),
            interfaces: vec![],
            fields: vec![],
            methods: vec![],
            attributes: vec![]
        };

        let r1_version = (class.version.major_version, class.version.minor_version);
        let r1_cp_len = class.constant_pool.cp_len();
        let r1_aflag = class.access_flags.flags;
        let r1_this_idx = class.this_class.idx;
        let r1_super_idx = class.super_class.idx;

        let mut target: Vec<u8> = vec![];
        {
            let mut writer: ClassWriter = ClassWriter::new(&mut target);
            let write_result = writer.write_class(&class);

            match write_result {
                Ok(_) => assert!(true),
                Err(err) => assert!(false, format!("{:?}", err))
            }
        }
//        assert!(false, format!("{:?}", target));
        {
            let read_result: Result<Classfile, Error> = ClassReader::read_class(&mut Cursor::new(&mut target));

            assert!(read_result.is_ok(), format!("{:?}", read_result.err()));

            let read_class = read_result.ok().unwrap();
            {
                assert!(false, format!("{:?}", read_class.constant_pool.constants));
            }
            assert_eq!(r1_version.0, read_class.version.major_version);
            assert_eq!(r1_version.1, read_class.version.minor_version);
            assert_eq!(r1_cp_len, read_class.constant_pool.cp_len());
            assert_eq!(r1_aflag, read_class.access_flags.flags);
            assert_eq!(r1_this_idx, read_class.this_class.idx);
            assert_eq!(r1_super_idx, read_class.super_class.idx);
        }
        assert!(true, format!("{:?}", target));
    }

    #[test]
    fn test_cursor_read_usage() {
        let mut cursor = Cursor::new(vec![ 1, 2, 3, 4 as u8 ]);

        let mut input = [ 0, 0 ];

        match cursor.read(&mut input) {
            Ok(_) => {
                assert_eq!([ 1, 2 ], input)
            },
            _ => assert!(false)
        }

        match cursor.read(&mut input) {
            Ok(_) => {
                assert_eq!([ 3, 4 ], input)
            },
            _ => assert!(false)
        }
    }

    #[test]
    fn test_cursor_write_usage() {
        let mut cursor: Cursor<Vec<u8>> = Cursor::new(vec![]);

        match cursor.write(&[ 1, 2 ]) {
            Ok(_) => assert!(true),
            _ => assert!(false)
        }

        match cursor.write(&[ 3, 4 ]) {
            Ok(_) => assert!(true),
            _ => assert!(false)
        }

        let output = cursor.into_inner();

        assert_eq!(vec![ 1, 2, 3, 4 ], output);
    }

}

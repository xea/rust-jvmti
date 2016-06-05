extern crate jvmti;

#[cfg(test)]
mod tests {

    use jvmti::bytecode::constants::ConstantType;
    use jvmti::bytecode::class_stream::ClassStream;

    #[test]
    fn it_should_parse_longs() {
        let bytes = vec![ 0x05, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08 ];
        let mut cs = ClassStream::new(&bytes);
        let oc = ConstantType::parse(&mut cs);

        match oc {
            Some(c) => match c {
                ConstantType::Long { high_bytes: h, low_bytes: l } => {
                    assert_eq!(0x01020304, h);
                    assert_eq!(0x05060708, l);
                },
                _ => assert!(false)
            },
            _ => assert!(false)
        }
    }

    #[test]
    fn it_should_parse_strings() {
        let bytes = vec![ 0x01, 0x00, 0x00 ];
        let mut cs = ClassStream::new(&bytes);
        let oc = ConstantType::parse(&mut cs);

        match oc {
            Some(c) => match c {
                ConstantType::Utf8 { length: l, bytes: b } => {
                    assert_eq!(0, l);
                    assert_eq!(0, b.len());
                },
                _ => assert!(false)
            },
            _ => assert!(false)
        }

        let bytes = vec![ 0x01, 0x00, 0x04, 0x40, 0x41, 0x42, 0x43 ];
        let mut cs = ClassStream::new(&bytes);
        let oc = ConstantType::parse(&mut cs);

        match oc {
            Some(c) => match c {
                ConstantType::Utf8 { length: l, bytes: b } => {
                    assert_eq!(4, l);
                    assert_eq!(4, b.len());
                    assert_eq!(vec![ 0x40, 0x41, 0x42, 0x43 ], b);
                },
                _ => assert!(false)
            },
            _ => assert!(false)
        }
    }
}

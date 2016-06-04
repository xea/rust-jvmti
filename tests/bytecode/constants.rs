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
}

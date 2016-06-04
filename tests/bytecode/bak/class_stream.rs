extern crate jvmti;

#[cfg(test)]
mod tests {

    use jvmti::bytecode::class_stream::ClassStream;

    #[test]
    fn read_constant_pool_should_work() {
        let data = vec![];
        let mut stream = ClassStream::new(&data);
        let r = stream.read_constant_pool();
        assert!(r.is_none());

        let data = vec![ 0x00, 0x01 ];
        let mut stream = ClassStream::new(&data);
        let r = stream.read_constant_pool();
        assert!(r.is_some());
        // 0th element should be a placeholder
        assert_eq!(1, r.unwrap().len());

        let data = vec![ 0x00, 0x02, 0x01, 0x00 ];
        let mut stream = ClassStream::new(&data);
        let r = stream.read_constant_pool();
        assert!(r.is_some());
        assert_eq!(2, r.unwrap().len());
    }
}

extern crate jvmti;

#[cfg(test)]
mod tests {
    mod class_stream {
        use jvmti::bytecode::stream::ClassInputStream;

        #[test]
        fn peek_bytes_should_return_the_requested_number_of_bytes() {
            let bytes: Vec<u8> = vec![ 0, 1, 2, 3, 4, 5, 6, 7 ];
            let cs = ClassInputStream::from_vec(&bytes);

            assert_eq!(0, cs.peek_bytes(0));
            assert_eq!(0, cs.peek_bytes(1));
            assert_eq!(1, cs.peek_bytes(2));
            assert_eq!(0x102, cs.peek_bytes(3));
            assert_eq!(0x10203, cs.peek_bytes(4));
            assert_eq!(0x1020304, cs.peek_bytes(5));
            assert_eq!(0x102030405, cs.peek_bytes(6));
            assert_eq!(0x10203040506, cs.peek_bytes(7));
            assert_eq!(0x1020304050607, cs.peek_bytes(8));
        }

        #[test]
        fn peek_bytes_should_never_move_the_stream_index() {
            let bytes: Vec<u8> = vec![ 0, 1, 2, 3, 4, 5, 6, 7 ];
            let cs = ClassInputStream::from_vec(&bytes);

            cs.peek_bytes(0);
            assert_eq!(8, cs.available());

            cs.peek_bytes(2);
            assert_eq!(8, cs.available());

            cs.peek_bytes(1);
            assert_eq!(8, cs.available());
        }

        #[test]
        fn read_bytes_should_move_the_stream_index_with_the_requested_amount() {
            let bytes: Vec<u8> = vec![ 0, 1, 2, 3, 4, 5, 6, 7 ];
            let cs = ClassInputStream::from_vec(&bytes);

            assert_eq!(8, cs.available());
            cs.read_bytes(1);
            assert_eq!(7, cs.available());
            cs.read_bytes(2);
            assert_eq!(5, cs.available());
            cs.read_bytes(3);
            assert_eq!(2, cs.available());
            cs.read_bytes(1);
            assert_eq!(1, cs.available());
        }

        #[test]
        fn mark_should_not_change_the_current_index() {
            let bytes: Vec<u8> = vec![ 0, 1, 2, 3, 4, 5, 6, 7 ];
            let cs = ClassInputStream::from_vec(&bytes);

            assert_eq!(8, cs.available());
            cs.read_bytes(4);
            assert_eq!(4, cs.available());
            cs.mark();
            assert_eq!(4, cs.available());
            cs.mark();
            assert_eq!(4, cs.available());
            cs.read_bytes(2);
            assert_eq!(2, cs.available());
            cs.mark();
            cs.mark();
            assert_eq!(2, cs.available());

        }

        #[test]
        fn mark_should_override_the_previous_call_positions() {
            let bytes: Vec<u8> = vec![ 0, 1, 2, 3, 4, 5, 6, 7 ];
            let cs = ClassInputStream::from_vec(&bytes);

            assert_eq!(8, cs.available());
            cs.read_bytes(4);
            assert_eq!(4, cs.available());
            cs.mark();
            cs.read_bytes(2);
            assert_eq!(2, cs.available());
            cs.mark();
            cs.read_bytes(2);
            assert_eq!(0, cs.available());
            cs.reset();

            let or = cs.read_bytes(2);

            assert!(or.is_some());
        }

        #[test]
        fn reset_should_rewind_the_stream_to_the_lastly_marked_position() {
            let bytes: Vec<u8> = vec![ 0, 1, 2, 3, 4, 5, 6, 7 ];
            let cs = ClassInputStream::from_vec(&bytes);

            cs.read_bytes(4);
            cs.mark();
            cs.read_bytes(2);
            cs.reset();
            assert_eq!(4, cs.available());
        }
    }
}

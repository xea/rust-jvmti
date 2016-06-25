extern crate jvmti;

#[cfg(test)]
mod tests {

    use jvmti::bytecode::*;
    use std::io::{ Cursor, Read, Write };

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

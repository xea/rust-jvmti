use self::classfile::Class;
use self::stream::ClassStream;

pub mod classfile;
pub mod collections;
pub mod stream;

///
/// Provides functionality for reading JVM class files as a whole
pub struct ClassReader {
}

impl ClassReader {

    pub fn read_bytes(bytes: &Vec<u8>) -> Result<Class, String> {
        let stream = ClassStream::from_vec(bytes);

        Err("Failed reading the class file".to_string())
    }

    pub fn consume_bytes(bytes: Vec<u8>) -> Result<Class, String> {
        let stream = ClassStream::from_vec(&bytes);

        Err("Failed reading the class file".to_string())
    }
}

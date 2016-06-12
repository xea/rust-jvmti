use self::classfile::*;
use self::constant::*;
use self::stream::{ ClassInputStream };

pub mod classfile;
pub mod collections;
pub mod constant;
pub mod stream;

///
/// Provides functionality for reading JVM class files as a whole
pub struct ClassReader {
}

impl ClassReader {

    pub fn read_array(bytes: &[u8]) -> Result<Class, String> {
        let vec: Vec<u8> = bytes.to_vec();
        ClassReader::read_bytes(&vec)
    }

    pub fn consume_bytes(bytes: Vec<u8>) -> Result<Class, String> {
        ClassReader::read_bytes(&bytes)
    }

    pub fn read_bytes(bytes: &Vec<u8>) -> Result<Class, String> {
        let mut stream = ClassInputStream::from_vec(bytes);

        let extractors: Vec<fn(&ClassInputStream) -> Result<ClassFragment, String>> = vec![
            ClassReader::read_magic_bytes,
            ClassReader::read_version_number,
            ClassReader::read_constant_pool,
            ClassReader::read_access_flags,
            //ClassReader::read_this_class,
            //ClassReader::read_super_class,
            //ClassReader::read_interfaces,
            //ClassReader::read_fields,
            //ClassReader::read_methods,
            //ClassReader::read_attributes

        ];

        // the idea is this: we start out with an Ok() value and fold items until we either reach
        // the end of the stream of hit an error. Once we find an Err() value we stop evaluating
        // subsequent items and always return the same Err()
        //
        // This way we either consume the whole class file or stop at the first error returning a
        // meaningful message describing the error
        let result: Result<ClassFragment, String> = extractors.iter().fold(Ok(ClassFragment::new()), |acc, x| {
            match acc {
                Ok(class_fragment) => match x(&mut stream) {
                    // only if both the accumulator and the current element are valid values do we
                    // continue processing, otherwise we fall back to Err
                    Ok(current_fragment) => Ok(current_fragment.merge(class_fragment)),
                    err@Err(_) => err
                },
                err@Err(_) => err
            }
        });

        result.map(|i| i.to_class())
    }

    /// Read magic bytes or return a readable error message
    fn read_magic_bytes(stream: &ClassInputStream) -> Result<ClassFragment, String> {
        match stream.read_magic_bytes() {
            Ok(_) => Ok(ClassFragment::default()),
            Err(err) => Err(err.to_string())
        }
    }


    /// Return the class file version number or return a readable error message
    fn read_version_number(stream: &ClassInputStream) -> Result<ClassFragment, String> {
        match stream.read_version_number() {
            Ok(version) => Ok(ClassFragment {
                version: Some(version),
                ..Default::default()
            }),
            Err(err) => Err(err.to_string())
        }
    }

    /// Return the constant pool or return a readable error message
    fn read_constant_pool(stream: &ClassInputStream) -> Result<ClassFragment, String> {
        match stream.read_constant_pool() {
            Ok(constant_pool) => Ok(ClassFragment {
                constant_pool: Some(constant_pool),
                ..Default::default()
            }),
            Err(err) => Err(err.to_string())
        }
    }

    /// Return access flags or return a readable error message
    fn read_access_flags(stream: &ClassInputStream) -> Result<ClassFragment, String> {
        Err("Reading access flags is not implemented".to_string())
    }

/*
    /// Return this class or return a readable error message
    fn read_this_class(stream: &ClassInputStream) -> Result<ClassFragment, String> {
        Err("Not implemented".to_string())
    }

    /// Return super class or return a readable error message
    fn read_super_class(stream: &ClassInputStream) -> Result<ClassFragment, String> {
        Err("Not implemented".to_string())
    }

    /// Return interface list or return a readable error message
    fn read_interfaces(stream: &ClassInputStream) -> Result<ClassFragment, String> {
        Err("Not implemented".to_string())
    }

    /// Return field list or return a readable error message
    fn read_fields(stream: &ClassInputStream) -> Result<ClassFragment, String> {
        Err("Not implemented".to_string())
    }

    /// Return method list or return a readable error message
    fn read_methods(stream: &ClassInputStream) -> Result<ClassFragment, String> {
        Err("Not implemented".to_string())
    }

    /// Return class attributes or return a readable error message
    fn read_attributes(stream: &ClassInputStream) -> Result<ClassFragment, String> {
        Err("Not implemented".to_string())
    }
    */
}


/// Temporary structure to hold partial class file elements that can be merged together into a
/// complete class file.
#[derive(Default)]
struct ClassFragment {
    version: Option<ClassfileVersion>,
    constant_pool: Option<ConstantPool>
}

impl ClassFragment {
    /// Create a new class fragment initialised to empty by default
    pub fn new() -> ClassFragment {
        ClassFragment {
            ..Default::default()
        }
    }

    pub fn merge(mut self, other: Self) -> Self {
        self.version = other.version.or(self.version);
        self.constant_pool = other.constant_pool.or(self.constant_pool);
        self
    }

    /// Transform this class fragment into a final class file. Members set on the fragment will
    /// be defined on the class too, other members will be initialized with their default values
    pub fn to_class(self) -> Class {
        Class {
            version: self.version.unwrap_or(ClassfileVersion::default()),
            constant_pool: self.constant_pool.unwrap_or(ConstantPool::default())
        }
    }
}

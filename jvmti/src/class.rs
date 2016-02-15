use super::native::JavaClass;
use std::ptr;


#[derive(Debug, Eq, PartialEq)]
pub enum JavaType<'a> {
    Boolean,
    Byte,
    Char,
    Double,
    Float,
    Int,
    Long,
    Short,
    Void,
    Class(&'a str),
    Array(Box<JavaType<'a>>)
}

impl<'a> JavaType<'a> {

    pub fn parse(signature: &'a str) -> Option<JavaType<'a>> {
        match signature.len() {
            0 => None,
            1 => match &*signature {
                "B" => Some(JavaType::Byte),
                "C" => Some(JavaType::Char),
                "D" => Some(JavaType::Double),
                "F" => Some(JavaType::Float),
                "I" => Some(JavaType::Int),
                "L" => Some(JavaType::Long),
                "S" => Some(JavaType::Short),
                "V" => Some(JavaType::Void),
                "Z" => Some(JavaType::Boolean),
                _ => None
            },
            _ => {
                match signature.chars().nth(0).unwrap() {
                    '[' => {
                        let (_, local_type) = signature.split_at(1);

                        match JavaType::parse(local_type) {
                            Some(result) => Some(JavaType::Array(Box::new(result))),
                            None => None
                        }
                    },
                    'L' => Some(JavaType::Class(signature)),
                    _ => None
                }
            }
        }
    }
}
///
/// Represents a JNI local reference to a Java class
///
pub struct ClassId {
    pub native_id: JavaClass
}

pub struct Class<'a> {
    pub id: ClassId,
    pub signature: JavaType<'a>
}

impl<'a> Class<'a> {

    ///
    /// Constructs a new Class instance.
    ///
    pub fn new(id: ClassId, signature: JavaType<'a>) -> Class {
        Class { id: id, signature: signature }
    }

/*
    ///
    /// Return a new Class instance holding an unknown java class
    ///
    pub fn unknown() -> Class<'a> {
        Class { id: ClassId { native_id: ptr::null_mut() } }
    }
    */
}

/*
pub struct TypeSignature<'a> {
    pub signature: String
}

impl<'a> TypeSignature<'a> {

    pub fn parse(signature: String) -> Option<TypeSignature<'a>> {
        None
    }

    /// Returns a class signature where the signature shows that the signature is not known.
    pub fn unknown() -> TypeSignature<'a> {
        TypeSignature { signature: "<Unknown type signature>".to_string(), arguments: vec![], return_type: Type::Void }
    }

    /// Returns true if this signature is a primitive type signature (ie. not a class type), otherwise
    /// returns true.
    pub fn is_primitive(&self) -> bool {
        !self.is_class()
    }

    /// Returns true if the signature is a class signature (ie. not a primitive type), otherwise
    /// returns false.
    pub fn is_class(&self) -> bool {
        self.signature.starts_with("L")
    }

    /// Returns the fully-qualified name of the current type
    pub fn fqn(&self) -> String {
        match self.is_class() {
            true => self.into_fqn(),//(&*self.signature).replace("/", ".").replace(";", "").to_string(),
            false => self.signature.clone()
        }
    }

    /// Transforms the current signature into a fully-qualified name.
    /// Note: this function doesn't check if the signature is in the correct format.
    fn into_fqn(&self) -> String {
        let (_, second) = self.signature.split_at(1);
        second.replace("/", ".").replace(";", "")
    }
}
*/

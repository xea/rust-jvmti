use super::native::JavaClass;
use std::ptr;

///
/// Represents a JNI local reference to a Java class
///
pub struct ClassId {
    pub native_id: JavaClass
}

pub struct Class {
    pub id: ClassId,
    pub signature: TypeSignature
}

impl Class {

    ///
    /// Constructs a new Class instance.
    ///
    pub fn new(id: ClassId, signature: TypeSignature) -> Class {
        Class { id: id, signature: signature }
    }

    ///
    /// Return a new Class instance holding an unknown java class
    ///
    pub fn unknown() -> Class {
        Class { id: ClassId { native_id: ptr::null_mut() }, signature: TypeSignature::unknown() }
    }
}

pub struct TypeSignature {
    pub signature: String
}

impl TypeSignature {

    /// Returns a class signature where the signature shows that the signature is not known.
    pub fn unknown() -> TypeSignature {
        TypeSignature { signature: "<Unknown type signature>".to_string() }
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

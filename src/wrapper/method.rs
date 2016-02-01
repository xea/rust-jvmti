use super::native::jvmti_native::*;
use super::class::TypeSignature;
use std::ptr;

pub struct MethodId {
    pub native_id: jmethodID
}

pub struct Method {
    pub id: MethodId,
    pub signature: MethodSignature
}

impl Method {
    pub fn new(method_id: MethodId, signature: MethodSignature) -> Method {
        Method {
            id: method_id,
            signature: signature
        }
    }

    ///
    /// Return a Method instance encapsulating an unknown method (ie. a method that cannot be resolved)
    /// In theory, such methods should never occur.
    ///
    pub fn unknown() -> Method {
        Method {
            id: MethodId { native_id: ptr::null_mut() },
            signature: MethodSignature::new("<Unknown method>".to_string(), "<Unknown signature>".to_string())
        }
    }
}
pub struct MethodSignature {
    pub name: String,
    pub signature: String
}

impl MethodSignature {

    pub fn new(name: String, signature: String) -> MethodSignature {
        MethodSignature {
            name: name,
            signature: signature
        }
    }

    pub fn argument_types(&self) -> Vec<TypeSignature> {
//        self.signature.chars().map(|c| TypeSignature::unknown()).collect()
        let mut chars = self.signature.chars();
        // dropping the leading '('
        chars.next();

        vec![]
    }
}

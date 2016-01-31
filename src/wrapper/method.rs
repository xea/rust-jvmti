use super::native::jvmti_native::*;
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

/*
    pub fn name(&self) -> String {
        match self.env.get_method_name(&self) {
            Ok(sign) => format!("{} {}", sign.name, sign.signature),
            Err(_) => "<Signature unavailable>".to_string()
        }
    }

    pub fn get_class(&self) -> Result<Class, NativeError> {
        self.env.get_method_declaring_class(self)
    }*/

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
}

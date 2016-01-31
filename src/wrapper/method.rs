use super::native::jvmti_native::*;
use super::class::Class;
use super::error::NativeError;
use super::environment::JVMTIEnvironment;

pub struct MethodId {
    pub native_id: jmethodID
}

pub struct Method {
    id: MethodId,
    pub signature: MethodSignature
}

impl Method {
    pub fn new(method_id: MethodId, signature: MethodSignature) -> Method {
        Method {
            id: method_id,
            signature: signature
        }
    }

    pub fn id(&self) -> &MethodId {
        &self.id
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

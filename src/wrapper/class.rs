use super::native::JavaClass;
use std::ptr;

pub struct ClassId {
    pub native_id: JavaClass
}

pub struct Class {
    pub id: ClassId,
    pub signature: ClassSignature
}

impl Class {

    pub fn new(id: ClassId, signature: ClassSignature) -> Class {
        Class { id: id, signature: signature }
    }

    pub fn unknown() -> Class {
        Class { id: ClassId { native_id: ptr::null_mut() }, signature: ClassSignature::unknown() }
    }
}

pub struct ClassSignature {
    pub signature: String
}

impl ClassSignature {

    pub fn unknown() -> ClassSignature {
        ClassSignature { signature: "<Unknown class signature>".to_string() }
    }
}

use super::native::JavaMethod;

pub struct MethodId {
    pub native_id: JavaMethod
}

pub struct Method {
    pub id: MethodId
}

pub struct MethodSignature {
    pub name: String
}

impl MethodSignature {

    pub fn new(raw_signature: String) -> MethodSignature {
        MethodSignature { name: raw_signature }
    }

    pub fn unknown() -> MethodSignature {
        MethodSignature { name: "<UNKNOWN METHOD>".to_string() }
    }
}

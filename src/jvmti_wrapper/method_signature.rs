pub struct MethodSignature {
    pub name: String,
    pub signature: String,
    pub generic_signature: String
}

impl MethodSignature {

    pub fn new(name: String, signature: String, generic_signature: String) -> MethodSignature {
        MethodSignature {
            name: name,
            signature: signature,
            generic_signature: generic_signature
        }
    }
}

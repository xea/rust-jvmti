

pub struct Class {
    version: ClassfileVersion,
}

pub struct ClassfileVersion {
    pub minor_version: u16,
    pub major_version: u16
}

pub struct ConstantPoolIndex {
    index: u16
}

impl Default for ClassfileVersion {
    fn default() -> Self {
        ClassfileVersion { minor_version: 0, major_version: 52 }
    }
}


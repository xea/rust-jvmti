
pub struct Classfile {
    pub version: ClassfileVersion
}

impl Classfile {
    pub fn new() -> Classfile {
        Classfile::default()
    }
}

impl Default for Classfile {
    fn default() -> Self {
        Classfile {
            version: ClassfileVersion::default()
        }
    }
}

pub struct ClassfileVersion {
    pub minor_version: u16,
    pub major_version: u16
}

impl ClassfileVersion {
    pub fn new(major_version: u16, minor_version: u16) -> ClassfileVersion {
        ClassfileVersion { major_version: major_version, minor_version: minor_version }
    }
}

impl Default for ClassfileVersion {
    fn default() -> Self {
        const DEFAULT_MAJOR_VERSION: u16 = 52;
        const DEFAULT_MINOR_VERSION: u16 = 0;

        ClassfileVersion { major_version: DEFAULT_MAJOR_VERSION, minor_version: DEFAULT_MINOR_VERSION }
    }
}

pub struct ConstantPool {
}

use super::constants::ConstantType;
use super::constants::AccessFlag;

const DEFAULT_CLASSFILE_VERSION: (u16, u16) = (0x00, 0x34);

pub struct Classfile {
    pub minor_version: u16,
    pub major_version: u16,
    pub constant_pool: Vec<ConstantType>,
    pub access_flags: AccessFlag,
}

impl Classfile {

    /// Create a new, valid class file with empty but initialised members
    pub fn new() -> Classfile {
        Classfile {
            minor_version: Classfile::default_minor_version(),
            major_version: Classfile::default_major_version(),
            constant_pool: Classfile::default_constant_pool(),
            access_flags: AccessFlag::new()
        }
    }

    /// Return the default minor version number for class files
    pub fn default_minor_version() -> u16 { DEFAULT_CLASSFILE_VERSION.0 }
    /// Return the default major version number for class files
    pub fn default_major_version() -> u16 { DEFAULT_CLASSFILE_VERSION.1 }
    /// Return the default constant pool
    pub fn default_constant_pool() -> Vec<ConstantType> { vec![] }
}

use super::constants::*;

const DEFAULT_CLASSFILE_VERSION: (u16, u16) = (0x00, 0x34);

/// Class files contain definitions of a single class or interface for the Java Virtual Machine.
pub struct Classfile {
    pub minor_version: u16,
    pub major_version: u16,
    pub constant_pool: Vec<ConstantType>,
    pub access_flags: AccessFlag,
    pub this_class: ConstantReference,
    pub super_class: ConstantReference,
    pub interfaces: Vec<ConstantReference>,
    pub fields: Vec<Field>,
    pub methods: Vec<Method>,
    pub attributes: Vec<Attribute>
}

impl Classfile {
    /// Create a new, valid class file with empty but initialised members
    pub fn new() -> Classfile {
        Classfile {
            minor_version: Classfile::default_minor_version(),
            major_version: Classfile::default_major_version(),
            constant_pool: Classfile::default_constant_pool(),
            access_flags: AccessFlag::new(),
            this_class: ConstantReference::new(0),
            super_class: ConstantReference::new(0),
            interfaces: vec![],
            fields: vec![],
            methods: vec![],
            attributes: vec![]
        }
    }

    /// Return the default minor version number for class files
    pub fn default_minor_version() -> u16 { DEFAULT_CLASSFILE_VERSION.0 }
    /// Return the default major version number for class files
    pub fn default_major_version() -> u16 { DEFAULT_CLASSFILE_VERSION.1 }
    /// Return the default constant pool
    pub fn default_constant_pool() -> Vec<ConstantType> { vec![] }
}

pub struct Field {
    pub access_flags: AccessFlag,
    pub name_index: ConstantReference,
    pub descriptor_index: ConstantReference,
    pub attributes: Vec<Attribute>
}

pub struct Method {
    pub access_flags: AccessFlag,
    pub name_index: ConstantReference,
    pub descriptor_index: ConstantReference,
    pub attributes: Vec<Attribute>
}


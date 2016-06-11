/// Contains the definition of a single JVM class or interface.
#[derive(Default)]
pub struct Class {
    pub version: ClassfileVersion,
    pub constant_pool: Vec<Box<ConstantPoolEntry>>
}

/// Version number of the class file.
pub struct ClassfileVersion {
    pub minor_version: u16,
    pub major_version: u16,
}

/// Points to an entry in the constant pool
pub struct ConstantPoolIndex {
    pub index: u16
}


impl ClassfileVersion {
    pub fn new(major_version: u16, minor_version: u16) -> ClassfileVersion {
        ClassfileVersion {
            minor_version: minor_version,
            major_version: major_version
        }
    }
}

impl Default for ClassfileVersion {
    fn default() -> Self {
        ClassfileVersion { minor_version: 0, major_version: 52 }
    }
}

pub trait ConstantPoolEntry {
    fn entry_type(&self) -> ConstantType;
    fn is_long_entry(&self) -> bool;
}

pub enum Constant {
    Utf8 { length: usize }
}

impl Constant {
}

impl ConstantPoolEntry for Constant {
    fn entry_type(&self) -> ConstantType {
        match *self {
            Constant::Utf8 { length: _ } => ConstantType::Utf8
        }
    }

    fn is_long_entry(&self) -> bool { false }
}

/*
pub struct Utf8Constant {
}

impl ConstantPoolEntry for Utf8Constant {}

pub struct IntegerConstant {
    pub bytes: u32
}

pub struct FloatConstant {
    pub bytes: u32
}

impl FloatConstant {
}
*/

pub enum ConstantType {
    Utf8 = 1,
    Integer = 3,
    Float = 4,
    Long = 5,
    Double = 6,
    Class = 7,
    String = 8,
    FieldRef = 9,
    MethodRef = 10,
    InterfaceMethodRef = 11,
    NameAndType = 12,
    MethodHandle = 15,
    MethodType = 16,
    InvokeDynamic = 18,
    Unknown
}

impl ConstantType {
    pub fn from_byte(byte: u8) -> ConstantType {
        match byte {
            1 => ConstantType::Utf8,
            3 => ConstantType::Integer,
            4 => ConstantType::Float,
            5 => ConstantType::Long,
            6 => ConstantType::Double,
            7 => ConstantType::Class,
            8 => ConstantType::String,
            9 => ConstantType::FieldRef,
            10 => ConstantType::MethodRef,
            11 => ConstantType::InterfaceMethodRef,
            12 => ConstantType::NameAndType,
            15 => ConstantType::MethodHandle,
            16 => ConstantType::MethodType,
            18 => ConstantType::InvokeDynamic,
            _ => ConstantType::Unknown
        }
    }

}

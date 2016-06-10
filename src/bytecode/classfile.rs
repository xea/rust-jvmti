

pub struct Class {
    version: ClassfileVersion,
    constant_pool: Vec<ConstantPoolEntry>
}

pub struct ClassfileVersion {
    pub minor_version: u16,
    pub major_version: u16,
}

pub struct ConstantPoolIndex {
    index: u16
}

impl Default for ClassfileVersion {
    fn default() -> Self {
        ClassfileVersion { minor_version: 0, major_version: 52 }
    }
}

pub enum ConstantPoolEntry {

}

pub struct Utf8Constant {
}

pub struct IntegerConstant {
    bytes: u32
}

pub struct FloatConstant {
    bytes: u32
}

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
    InvokeDynamic = 18
}

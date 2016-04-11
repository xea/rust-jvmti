
pub struct ClassFileReader;


/*
enum ConstantType {
    Class = 7,
    FieldRef = 9,
    MethodRef = 10,
    InterfaceMethodRef = 11,
    String = 8,
    Integer = 3,
    Float = 4,
    Long = 5,
    Double = 6,
    NameAndType = 12,
    Utf8 = 1,
    MethodHandle = 15,
    MethodType = 16,
    InvokeDynamic = 18
}

enum AccessFlag {
    Public = 0x0001,
    Final = 0x0010,
    Super = 0x0020,
    Interface = 0x0200,
    Abstract = 0x0400,
    Synthetic = 0x1000,
    Annotation = 0x02000,
    Enum = 0x4000
}

struct ConstantPoolEntry {
    pub tag: u8,
    pub info: Vec<ConstantType>
}

struct RawClassFile {
    /// The magic item supplies the magic number identifying the class file format; it has the value 0xCAFEBABE.
    magic: u32,
    ///  Together, a major and a minor version number determine the version of the class file format.
    minor_version: u16,
    major_version: u16,
    /// The value of the constant_pool_count item is equal to the number of entries in the constant_pool table plus one.
    constant_pool_count: u16,
    /// The constant_pool is a table of structures (§4.4) representing various string constants, class
    /// and interface names, field names, and other constants that are referred to within the ClassFile structure and its substructures.
    constant_pool: Vec<ConstantPoolEntry>,
    access_flags: u16,
    this_class: u16,
    super_class: u16,
    interfaces_count: u16,

}
*/

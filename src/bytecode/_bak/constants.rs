
pub enum ConstantType {
    Class { name_index: u16 }, // 7
    FieldRef { class_index: u16, name_and_type_index: u16 }, // 9
    MethodRef { class_index: u16, name_and_type_index: u16 }, // 10,
    InterfaceMethodRef { class_index: u16, name_and_type_index: u16 }, // 11
    String { string_index: u16 }, // 8,
    Integer { bytes: u32 }, // 3,
    Float { bytes: u32 }, // 4,
    Long { high_bytes: u32, low_bytes: u32 }, // 5,
    Double { high_bytes: u32, low_bytes: u32 }, // 6,
    NameAndType { name_index: u16, descriptor_index: u16 }, // 12,
    Utf8 { length: u16, bytes: Vec<u8> }, // 1,
    MethodHandle { reference_kind: u8, reference_index: u16 }, // 15,
    MethodType { descriptor_index: u16 }, // 16,
    InvokeDynamic { bootstrap_method_attr_index: u16, name_and_type_index: u16 }, // 18,
    Placeholder,
    Unknown
}

pub enum ClassAccessFlagType {
    Public = 0x0001,
    Final = 0x0010,
    Super = 0x0020,
    Interface = 0x0200,
    Abstract = 0x0400,
    Synthetic = 0x1000,
    Annotation = 0x2000,
    Enum = 0x4000
}

pub enum FieldAccessFlagType {
    Public = 0x0001,
    Private = 0x0002,
    Protected = 0x0004,
    Static = 0x0008,
    Final = 0x0010,
    Volatile = 0x0040,
    Transient = 0x0080,
    Synthetic = 0x1000,
    Enum = 0x4000
}

pub enum MethodAccessFlagType {
    Public = 0x0001,
    Private = 0x0002,
    Protected = 0x0004,
    Static = 0x0008,
    Final = 0x0010,
    Synchronized = 0x0020,
    Bridge = 0x0040,
    Varargs = 0x0080,
    Native = 0x0100,
    Abstract = 0x0400,
    Strict = 0x0800,
    Synthetic = 0x1000
}

pub enum Attribute {
    // JVM attributes
    ConstantValue,
    Code,
    StackMapTable,
    Exceptions,
    BootstrapMethods,
    // Java SE Attributes
    InnerClasses,
    EnclosingMethod,
    Synthetic,
    Signature,
    RuntimeVisibleAnnotations,
    RuntimeInvisibleAnnotations,
    RuntimeVisibleParameterAnnotations,
    RuntimeInvisibleParameterAnnotations,
    RuntimeVisibleTypeAnnotations,
    RuntimeInvisibleTypeAnnotations,
    AnnotationDefault,
    MethodParameters,
    // Extra attributes
    SourceFile,
    SourceDebugExtension,
    LineNumberTable,
    LocalVariableTable,
    LocalVariableTypeTable,
    Deprecated
}

impl Attribute {
    pub fn to_string(&self) -> String {
        match self {
            &Attribute::ConstantValue => "ConstantValue",
            &Attribute::Code => "Code",
            &Attribute::StackMapTable => "StackMapTable",
            &Attribute::Exceptions => "Exceptions",
            &Attribute::BootstrapMethods => "BootstrapMethods",
            &Attribute::InnerClasses => "InnerClasses",
            &Attribute::EnclosingMethod => "EnclosingMethod",
            &Attribute::Synthetic => "Synthetic",
            &Attribute::Signature => "Signature",
            &Attribute::RuntimeVisibleAnnotations => "RuntimeVisibleAnnotations",
            &Attribute::RuntimeInvisibleAnnotations => "RuntimeInvisibleAnnotations",
            &Attribute::RuntimeVisibleParameterAnnotations => "RuntimeVisibleParameterAnnotations",
            &Attribute::RuntimeInvisibleParameterAnnotations => "RuntimeInvisibleParameterAnnotations",
            &Attribute::RuntimeVisibleTypeAnnotations => "RuntimeVisibleTypeAnnotations",
            &Attribute::RuntimeInvisibleTypeAnnotations => "RuntimeInvisibleTypeAnnotations",
            &Attribute::AnnotationDefault => "AnnotationDefault",
            &Attribute::MethodParameters => "MethodParameters",
            &Attribute::SourceFile => "SourceFile",
            &Attribute::SourceDebugExtension => "SourceDebugExtension",
            &Attribute::LineNumberTable => "LineNumberTable",
            &Attribute::LocalVariableTable => "LocalVariableTable",
            &Attribute::LocalVariableTypeTable => "LocalVariableTypeTable",
            &Attribute::Deprecated => "Deprecated"
        }.to_string()
    }
}

use super::constants::ConstantType;
use super::constants::AccessFlag;

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

/// Represents a reference to an entry in the class file's constant pool
pub struct ConstantReference {
    pub constant_idx: u16
}

impl ConstantReference {
    ///
    /// Create a new constant reference pointing to the idx-th constant in the pool.
    /// Note that the value of 0 is deemed invalid by the specification and is used to indicate
    /// an unknown reference here.
    pub fn new(idx: u16) -> ConstantReference {
        ConstantReference { constant_idx: idx }
    }

    /// Return a new constant reference pointing to an unknown constant in the pool
    pub fn unknown() -> ConstantReference {
        ConstantReference { constant_idx: 0 }
    }
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

pub struct Attribute {
    /// attribute_name_index must be a CONSTANT_Utf8_info structure (§4.4.7) representing the name of the attribute.
    pub attribute_name_index: ConstantReference,
    pub info: Vec<u8>
}

pub enum AttributeType {
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

impl AttributeType {
    pub fn to_string(&self) -> String {
        match self {
            &AttributeType::ConstantValue => "ConstantValue",
            &AttributeType::Code => "Code",
            &AttributeType::StackMapTable => "StackMapTable",
            &AttributeType::Exceptions => "Exceptions",
            &AttributeType::BootstrapMethods => "BootstrapMethods",
            &AttributeType::InnerClasses => "InnerClasses",
            &AttributeType::EnclosingMethod => "EnclosingMethod",
            &AttributeType::Synthetic => "Synthetic",
            &AttributeType::Signature => "Signature",
            &AttributeType::RuntimeVisibleAnnotations => "RuntimeVisibleAnnotations",
            &AttributeType::RuntimeInvisibleAnnotations => "RuntimeInvisibleAnnotations",
            &AttributeType::RuntimeVisibleParameterAnnotations => "RuntimeVisibleParameterAnnotations",
            &AttributeType::RuntimeInvisibleParameterAnnotations => "RuntimeInvisibleParameterAnnotations",
            &AttributeType::RuntimeVisibleTypeAnnotations => "RuntimeVisibleTypeAnnotations",
            &AttributeType::RuntimeInvisibleTypeAnnotations => "RuntimeInvisibleTypeAnnotations",
            &AttributeType::AnnotationDefault => "AnnotationDefault",
            &AttributeType::MethodParameters => "MethodParameters",
            &AttributeType::SourceFile => "SourceFile",
            &AttributeType::SourceDebugExtension => "SourceDebugExtension",
            &AttributeType::LineNumberTable => "LineNumberTable",
            &AttributeType::LocalVariableTable => "LocalVariableTable",
            &AttributeType::LocalVariableTypeTable => "LocalVariableTypeTable",
            &AttributeType::Deprecated => "Deprecated"
        }.to_string()
    }

    pub fn from_string(string: &str) -> Option<AttributeType> {
        match string {
            "ConstantValue" => Some(AttributeType::ConstantValue),
            "Code" => Some(AttributeType::Code),
            "StackMapTable" => Some(AttributeType::StackMapTable),
            "Exceptions" => Some(AttributeType::Exceptions),
            "BootstrapMethods" => Some(AttributeType::BootstrapMethods),
            "InnerClasses" => Some(AttributeType::InnerClasses),
            "EnclosingMethod" => Some(AttributeType::EnclosingMethod),
            "Synthetic" => Some(AttributeType::Synthetic),
            "Signature" => Some(AttributeType::Signature),
            "RuntimeVisibleAnnotations" => Some(AttributeType::RuntimeVisibleAnnotations),
            "RuntimeInvisibleAnnotations" => Some(AttributeType::RuntimeInvisibleAnnotations),
            "RuntimeVisibleParameterAnnotations" => Some(AttributeType::RuntimeVisibleParameterAnnotations),
            "RuntimeInvisibleParameterAnnotations" => Some(AttributeType::RuntimeInvisibleParameterAnnotations),
            "RuntimeVisibleTypeAnnotations" => Some(AttributeType::RuntimeVisibleTypeAnnotations),
            "RuntimeInvisibleTypeAnnotations" => Some(AttributeType::RuntimeInvisibleTypeAnnotations),
            "AnnotationDefault" => Some(AttributeType::AnnotationDefault),
            "MethodParameters" => Some(AttributeType::MethodParameters),
            "SourceFile" => Some(AttributeType::SourceFile),
            "SourceDebugExtension" => Some(AttributeType::SourceDebugExtension),
            "LineNumberTable" => Some(AttributeType::LineNumberTable),
            "LocalVariableTable" => Some(AttributeType::LocalVariableTable),
            "LocalVariableTypeTable" => Some(AttributeType::LocalVariableTypeTable),
            "Deprecated" => Some(AttributeType::Deprecated),
            _ => None
        }
    }
}

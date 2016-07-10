
///
/// A `Classfile` represents a definition of a single JVM class or interface. Unlike the bytecode
/// itself, it doesn't represent every byte in the class definition, though, many information are
/// encoded in the type system instead. This approach may seem restrictive but it helps achieving
/// bytecode safety.
#[derive(Debug)]
pub struct Classfile {
    pub version: ClassfileVersion,
    pub constant_pool: ConstantPool,
    pub access_flags: AccessFlags,
    pub this_class: ConstantPoolIndex,
    pub super_class: ConstantPoolIndex,
    pub interfaces: Vec<ConstantPoolIndex>,
    pub fields: Vec<Field>,
    pub methods: Vec<Method>,
    pub attributes: Vec<Attribute>
}

impl Classfile {
    /// Create a new classfile, initialised with sensible default values
    pub fn new() -> Classfile {
        Classfile::default()
    }
}

impl Default for Classfile {
    fn default() -> Self {
        Classfile {
            version: ClassfileVersion::default(),
            constant_pool: ConstantPool::default(),
            access_flags: AccessFlags::default(),
            this_class: ConstantPoolIndex::default(),
            super_class: ConstantPoolIndex::default(),
            interfaces: vec![],
            fields: vec![],
            methods: vec![],
            attributes: vec![]
        }
    }
}

///
/// Describe a classfile version number.
#[derive(Ord, PartialOrd, Eq, PartialEq, Debug)]
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

///
/// A `ConstantPool` is a table of various string and number literal constants that are referred
/// within the substructures of the `Classfile`.
#[derive(Debug)]
pub struct ConstantPool {
    pub constants: Vec<Constant>
}

impl ConstantPool {
    pub fn new(constants: Vec<Constant>) -> ConstantPool {
        ConstantPool {
            constants: constants
        }
    }

    pub fn get_utf8(&self, idx: u16) -> Option<&Vec<u8>> {
        match self.constants.get(idx as usize) {
            Some(constant) => match constant {
                _ => None
            },
            _ => None
        }
    }

    pub fn get_utf8_string(&self, idx: u16) -> Option<String> {
        match self.get_utf8(idx) {
            Some(bytes) => match String::from_utf8(bytes.clone()) {
                Ok(string) => Some(string),
                _ => None
            },
            _ => None
        }
    }

    pub fn get_utf8_index(&self) -> Option<usize> {
        None
    }

    pub fn resolve_index(&self, idx: &ConstantPoolIndex) -> Option<&Constant> {
        self.constants.get(idx.idx)
    }

    pub fn cp_len(&self) -> usize {
        //self.constants.iter().fold(0, |acc, x| acc + x.cp_size())
        self.constants.len()
    }
}

impl Default for ConstantPool {
    fn default() -> Self {
        ConstantPool {
            constants: vec![]
        }
    }
}

#[derive(Default, Debug)]
pub struct ConstantPoolIndex {
    pub idx: usize
}

impl ConstantPoolIndex {
    pub fn new(idx: usize) -> Self {
        ConstantPoolIndex { idx: idx }
    }
}

#[derive(Debug)]
pub enum Constant {
    Utf8(Vec<u8>),
    Integer(u32),
    Float(u32),
    Long(u64),
    Double(u64),
    Class(ConstantPoolIndex),
    FieldRef { class_index: ConstantPoolIndex, name_and_type_index: ConstantPoolIndex },
    MethodRef { class_index: ConstantPoolIndex, name_and_type_index: ConstantPoolIndex },
    InterfaceMethodRef { class_index: ConstantPoolIndex, name_and_type_index: ConstantPoolIndex },
    String(ConstantPoolIndex),
    NameAndType { name_index: ConstantPoolIndex, descriptor_index: ConstantPoolIndex },
    MethodHandle { reference_kind: ReferenceKind, reference_index: ConstantPoolIndex },
    MethodType(ConstantPoolIndex),
    InvokeDynamic { bootstrap_method_attr_index: ConstantPoolIndex, name_and_type_index: ConstantPoolIndex },
    Unknown(u8),
    Placeholder
}

impl Constant {
    pub fn cp_size(&self) -> usize {
        match self {
            &Constant::Long(_) => 2,
            &Constant::Double(_) => 2,
            &Constant::Placeholder => 0,
            _ => 1
        }
    }
}

#[derive(Debug)]
pub enum ReferenceKind {
    GetField = 1,
    GetStatic = 2,
    PutField = 3,
    PutStatic = 4,
    InvokeVirtual = 5,
    InvokeStatic = 6,
    InvokeSpecial = 7,
    NewInvokeSpecial = 8,
    InvokeInterface = 9,
    Unknown = 255
}

impl ReferenceKind {
    pub fn from_u8(value: u8) -> ReferenceKind {
        match value {
            1 => ReferenceKind::GetField,
            2 => ReferenceKind::GetStatic,
            3 => ReferenceKind::PutField,
            4 => ReferenceKind::PutStatic,
            5 => ReferenceKind::InvokeVirtual,
            6 => ReferenceKind::InvokeStatic,
            7 => ReferenceKind::InvokeSpecial,
            8 => ReferenceKind::NewInvokeSpecial,
            9 => ReferenceKind::InvokeInterface,
            _ => ReferenceKind::Unknown
        }
    }

    pub fn to_u8(&self) -> u8 {
        match *self {
            ReferenceKind::GetField => 1,
            ReferenceKind::GetStatic => 2,
            ReferenceKind::PutField => 3,
            ReferenceKind::PutStatic => 4,
            ReferenceKind::InvokeVirtual => 5,
            ReferenceKind::InvokeStatic => 6,
            ReferenceKind::InvokeSpecial => 7,
            ReferenceKind::NewInvokeSpecial => 8,
            ReferenceKind::InvokeInterface => 9,
            ReferenceKind::Unknown => 255
        }
    }
}

#[derive(Default, Debug)]
pub struct AccessFlags {
    pub flags: u16
}

impl AccessFlags {
    pub fn new() -> AccessFlags {
        AccessFlags::of(0)
    }

    pub fn of(val: u16) -> AccessFlags {
        AccessFlags { flags: val }
    }

    pub fn has_flag(&self, flag: u16) -> bool {
        self.flags & flag > 0
    }

    pub fn set_flag(&mut self, flag: u16) {
        self.flags |= flag;
    }

    pub fn clear_flag(&mut self, flag: u16) {
        self.flags &= flag ^ 0xFFFF;
    }
}

pub enum ClassAccessFlags {
    Public = 0x0001, // Declared public; may be accessed from outside its package.
    Final = 0x0010, // Declared final; no subclasses allowed.
    Super = 0x0020, // Treat superclass methods specially when invoked by the invokespecial instruction.
    Interface = 0x0200, // Is an interface, not a class.
    Abstract = 0x0400, // Declared abstract; must not be instantiated.
    Synthetic = 0x1000, // Declared synthetic; not present in the source code.
    Annotation = 0x2000, // Declared as an annotation type.
    Enum = 0x4000 // Declared as an enum type.
}

pub enum FieldAccessFlags {
    Public = 0x0001, //	Declared public; may be accessed from outside its package.
    Private = 0x0002, //	Declared private; usable only within the defining class.
    Protected = 0x0004, //	Declared protected; may be accessed within subclasses.
    Static = 0x0008, //	Declared static.
    Final = 0x0010, //	Declared final; never directly assigned to after object construction (JLS §17.5).
    Volatile = 0x0040, //	Declared volatile; cannot be cached.
    Transient = 0x0080, //	Declared transient; not written or read by a persistent object manager.
    Synthetic = 0x1000, //	Declared synthetic; not present in the source code.
    Enum = 0x4000 //	Declared as an element of an enum.
}

pub enum MethodAccessFlags {
    Public = 0x0001, //	Declared public; may be accessed from outside its package.
    Private = 0x0002, //	Declared private; accessible only within the defining class.
    Protected = 0x0004, //	Declared protected; may be accessed within subclasses.
    Static = 0x0008, //	Declared static.
    Final = 0x0010, //	Declared final; must not be overridden (§5.4.5).
    Synchronized = 0x0020, //	Declared synchronized; invocation is wrapped by a monitor use.
    Bridge = 0x0040, //	A bridge method, generated by the compiler.
    Varargs = 0x0080, //	Declared with variable number of arguments.
    Native = 0x0100, //	Declared native; implemented in a language other than Java.
    Abstract = 0x0400, //	Declared abstract; no implementation is provided.
    Strict = 0x0800, //	Declared strictfp; floating-point mode is FP-strict.
    Synthetic = 0x1000 //	Declared synthetic; not present in the source code.
}

pub enum InnerClassAccessFlags {
    Public = 0x0001, //	Marked or implicitly public in source.
    Private = 0x0002, //	Marked private in source.
    Protected = 0x0004, //	Marked protected in source.
    Static = 0x0008, //	Marked or implicitly static in source.
    Final = 0x0010, //	Marked final in source.
    Interface = 0x0200, //	Was an interface in source.
    Abstract = 0x0400, //	Marked or implicitly abstract in source.
    Synthetic = 0x1000, //	Declared synthetic; not present in the source code.
    Annotation = 0x2000, //	Declared as an annotation type.
    Enum = 0x4000, //	Declared as an enum type.
}

pub enum ParameterAccessFlags {
    Final = 0x0010,
    Synthetic = 0x1000,
    Mandated = 0x8000
}

#[derive(Default, Debug)]
pub struct Field {
    pub access_flags: AccessFlags,
    pub name_index: ConstantPoolIndex,
    pub descriptor_index: ConstantPoolIndex,
    pub attributes: Vec<Attribute>
}

#[derive(Default, Debug)]
pub struct Method {
    pub access_flags: AccessFlags,
    pub name_index: ConstantPoolIndex,
    pub descriptor_index: ConstantPoolIndex,
    pub attributes: Vec<Attribute>
}

#[derive(Debug)]
pub enum Attribute {
    ConstantValue(ConstantPoolIndex),
    Code { max_stack: u16, max_locals: u16, code: Vec<u8>, exception_table: Vec<ExceptionHandler>, attributes: Vec<Attribute> },
    StackMapTable(Vec<StackMapFrame>),
    Exceptions(Vec<ConstantPoolIndex>),
    InnerClass(Vec<InnerClass>),
    EnclosingMethod { class_index: ConstantPoolIndex, method_index: ConstantPoolIndex },
    Synthetic,
    Signature(ConstantPoolIndex),
    SourceFile(ConstantPoolIndex),
    SourceDebugExtension(Vec<u8>),
    LineNumbeTable(Vec<LineNumberTable>),
    LocalVariableTable(Vec<LocalVariableTable>),
    LocalVariableTableType(Vec<LocalVariableTableType>),
    Deprecated,
    RuntimeVisibleAnnotations(Vec<Annotation>),
    RuntimeInvisibleAnnotations(Vec<Annotation>),
    RuntimeVisibleParameterAnnotations(Vec<Vec<Annotation>>),
    RuntimeInvisibleParameterAnnotations(Vec<Vec<Annotation>>),
    RuntimeVisibleTypeAnnotations(Vec<TypeAnnotation>),
    RuntimeInvisibleTypeAnnotations(Vec<TypeAnnotation>),
    AnnotationDefault(ElementValue),
    BootstrapMethods(Vec<BootstrapMethod>),
    MethodParameters(Vec<MethodParameter>),
    RawAttribute { name_index: ConstantPoolIndex, info: Vec<u8> }
}

#[derive(Debug)]
pub enum StackMapFrame {
    SameFrame,
    SameLocals1StackItemFrame { stack: VerificationType },
    SameLocals1StackItemFrameExtended { offset_delta: u16, stack: VerificationType },
    ChopFrame { offset_delta: u16 },
    SameFrameExtended { offset_delta: u16 },
    AppendFrame { offset_delta: u16, locals: Vec<VerificationType> },
    FullFrame { offset_delta: u16, locals: Vec<VerificationType>, stack: Vec<VerificationType> }
}

#[derive(Debug)]
pub enum VerificationType {
    Top,
    Integer,
    Float,
    Long,
    Double,
    Null,
    Uninitializedthis,
    Object { cpool_index: ConstantPoolIndex },
    Uninitialized { offset: u16 }
}

#[derive(Debug)]
pub struct ExceptionHandler {
    pub start_pc: u16,
    pub end_pc: u16,
    pub handler_pc: u16,
    pub catch_type: ConstantPoolIndex
}

#[derive(Debug)]
pub struct InnerClass {
    pub inner_class_info_index: ConstantPoolIndex,
    pub outer_class_info_index: ConstantPoolIndex,
    pub inner_name_index: ConstantPoolIndex,
    pub access_flags: AccessFlags
}

#[derive(Debug)]
pub struct LineNumberTable {
    pub start_pc: u16,
    pub line_number: u16
}

#[derive(Debug)]
pub struct LocalVariableTable {
    pub start_pc: u16,
    pub length: u16,
    pub name_index: ConstantPoolIndex,
    pub descriptor_index: ConstantPoolIndex,
    pub index: u16
}

#[derive(Debug)]
pub struct LocalVariableTableType {
    pub start_pc: u16,
    pub length: u16,
    pub name_index: ConstantPoolIndex,
    pub signature_index: ConstantPoolIndex,
    pub index: u16
}

#[derive(Debug)]
pub struct Annotation {
    pub type_index: ConstantPoolIndex,
    pub element_value_pairs: Vec<ElementValuePair>
}

#[derive(Debug)]
pub struct ElementValuePair {
    pub element_name_index: ConstantPoolIndex,
    pub value: ElementValue
}


#[derive(Debug)]
pub enum ElementValue {
    ConstantValue(ConstantPoolIndex),
    Enum { type_name_index: ConstantPoolIndex, const_name_index: ConstantPoolIndex },
    ClassInfo(ConstantPoolIndex),
    Annotation(Annotation),
    Array(Vec<ElementValue>)
}

#[derive(Debug)]
pub struct TypeAnnotation {
    // TODO
}

#[derive(Debug)]
pub struct BootstrapMethod {
    pub bootstrap_method_ref: ConstantPoolIndex,
    pub bootstrap_arguments: Vec<ConstantPoolIndex>
}

#[derive(Debug)]
pub struct MethodParameter {
    pub name_index: ConstantPoolIndex,
    pub access_flags: AccessFlags
}

use super::class_stream::ClassStream;
use super::class_stream::ReadChunks;
use super::class_stream::ReadMapper;

pub enum ConstantType {
    Utf8 { length: u16, bytes: Vec<u8> }, // 1,
    Integer { bytes: u32 }, // 3,
    Float { bytes: u32 }, // 4,
    Long { high_bytes: u32, low_bytes: u32 }, // 5,
    Double { high_bytes: u32, low_bytes: u32 }, // 6,
    Class { name_index: u16 }, // 7
    String { string_index: u16 }, // 8,
    FieldRef { class_index: u16, name_and_type_index: u16 }, // 9
    MethodRef { class_index: u16, name_and_type_index: u16 }, // 10,
    InterfaceMethodRef { class_index: u16, name_and_type_index: u16 }, // 11
    NameAndType { name_index: u16, descriptor_index: u16 }, // 12,
    MethodHandle { reference_kind: u8, reference_index: u16 }, // 15,
    MethodType { descriptor_index: u16 }, // 16,
    InvokeDynamic { bootstrap_method_attr_index: u16, name_and_type_index: u16 }, // 18,
    Placeholder,
    Unknown
}

impl ConstantType {

    pub fn is_long_entry(&self) -> bool {
        match *self {
            ConstantType::Long { high_bytes: _, low_bytes: _ } => true,
            ConstantType::Double { high_bytes: _, low_bytes: _ } => true,
            _ => false
        }
    }

    pub fn parse(stream: &mut ClassStream) -> Option<ConstantType> {
        let opt_tag = stream.read_u8();

        match opt_tag {
            Some(tag) => {
                match tag {
                    3 => stream.read_map_len(4, |bs| ConstantType::Integer { bytes: bs.get_u32() }),
                    4 => stream.read_map_len(4, |bs| ConstantType::Float { bytes: bs.get_u32() }),
                    5 => stream.read_map_len(8, |bs| ConstantType::Long { high_bytes: bs.get_u32(), low_bytes: bs.get_u32() }),
                    6 => stream.read_map_len(8, |bs| ConstantType::Double { high_bytes: bs.get_u32(), low_bytes: bs.get_u32() }),
                    7 => stream.read_map_len(2, |bs| ConstantType::Class { name_index: bs.get_u16() }),
                    8 => stream.read_map_len(2, |bs| ConstantType::String { string_index: bs.get_u16() }),
                    9 => stream.read_map_len(4, |bs| ConstantType::FieldRef { class_index: bs.get_u16(), name_and_type_index: bs.get_u16() }),
                    10 => stream.read_map_len(4, |bs| ConstantType::MethodRef { class_index: bs.get_u16(), name_and_type_index: bs.get_u16() }),
                    11 => stream.read_map_len(4, |bs| ConstantType::InterfaceMethodRef { class_index: bs.get_u16(), name_and_type_index: bs.get_u16() }),
                    12 => stream.read_map_len(4, |bs| ConstantType::NameAndType { name_index: bs.get_u16(), descriptor_index: bs.get_u16() }),
                    15 => stream.read_map_len(3, |bs| ConstantType::MethodHandle { reference_kind: bs.get_u8(), reference_index: bs.get_u16() }),
                    16 => stream.read_map_len(2, |bs| ConstantType::MethodType { descriptor_index: bs.get_u16() }),
                    18 => stream.read_map_len(4, |bs| ConstantType::InvokeDynamic { bootstrap_method_attr_index: bs.get_u16(), name_and_type_index: bs.get_u16() }),
                    1 => stream.read_map_len(2, |bs| {
                        let bytes_count = bs.get_u16();

                        match bs.read_n(bytes_count as usize) {
                            Some(bytes) => ConstantType::Utf8 { length: bytes_count, bytes: bytes },
                            None => ConstantType::Unknown
                        }
                    }),
                    _ => Some(ConstantType::Unknown)
                }
            },
            None => None
        }
    }
}

pub enum ClassAccessFlag {
    Public,
    Final,
    Super,
    Interface,
    Abstract,
    Synthetic,
    Annotation,
    Enum
}

pub enum FieldAccessFlag {
    Public,
    Private,
    Protected,
    Static,
    Final,
    Volatile,
    Transient,
    Synthetic,
    Enum
}

pub enum MethodAccessFlag {
    Public,
    Private,
    Protected,
    Static,
    Final,
    Synchronized,
    Bridge,
    Varargs,
    Native,
    Abstract,
    Strict,
    Synthetic,
}

pub struct AccessFlag {
    raw_flag: u16
}

impl AccessFlag {
    pub fn new() -> AccessFlag {
        AccessFlag::of(0)
    }

    pub fn of(value: u16) -> AccessFlag {
        AccessFlag { raw_flag: value }
    }

    pub fn get<T: FlagValue>(&self, other: T) -> bool {
        self.raw_flag & other.val() > 0
    }

    pub fn set<T: FlagValue>(&mut self, flag: T) {
        self.raw_flag |= flag.val();
    }

    pub fn clear<T: FlagValue>(&mut self, flag: T) {
        self.raw_flag &= 0xFFFF ^ flag.val();
    }
}

pub trait FlagValue {
    fn val(&self) -> u16;
}

impl FlagValue for ClassAccessFlag {
    fn val(&self) -> u16 {
        match self {
            &ClassAccessFlag::Public => 0x0001,
            &ClassAccessFlag::Final => 0x0010,
            &ClassAccessFlag::Super => 0x0020,
            &ClassAccessFlag::Interface => 0x0200,
            &ClassAccessFlag::Abstract => 0x0400,
            &ClassAccessFlag::Synthetic => 0x1000,
            &ClassAccessFlag::Annotation => 0x2000,
            &ClassAccessFlag::Enum => 0x4000
        }
    }
}

impl FlagValue for FieldAccessFlag {
    fn val(&self) -> u16 {
        match self {
            &FieldAccessFlag::Public => 0x0001,
            &FieldAccessFlag::Private => 0x0002,
            &FieldAccessFlag::Protected => 0x0004,
            &FieldAccessFlag::Static => 0x0008,
            &FieldAccessFlag::Final => 0x0010,
            &FieldAccessFlag::Volatile => 0x0040,
            &FieldAccessFlag::Transient => 0x0080,
            &FieldAccessFlag::Synthetic => 0x1000,
            &FieldAccessFlag::Enum => 0x4000
        }
    }
}

impl FlagValue for MethodAccessFlag {
    fn val(&self) -> u16 {
        match self {
            &MethodAccessFlag::Public => 0x0001,
            &MethodAccessFlag::Private => 0x0002,
            &MethodAccessFlag::Protected => 0x0004,
            &MethodAccessFlag::Static => 0x0008,
            &MethodAccessFlag::Final => 0x0010,
            &MethodAccessFlag::Synchronized => 0x0020,
            &MethodAccessFlag::Bridge => 0x0040,
            &MethodAccessFlag::Varargs => 0x0080,
            &MethodAccessFlag::Native => 0x0100,
            &MethodAccessFlag::Abstract => 0x0400,
            &MethodAccessFlag::Strict => 0x0800,
            &MethodAccessFlag::Synthetic => 0x1000
        }
    }
}

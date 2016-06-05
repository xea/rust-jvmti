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

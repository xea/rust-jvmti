use super::class_stream::ClassStream;
use super::class_stream::ReadChunks;
use super::class_stream::ReadMapper;

pub enum ConstantType {
    Integer { bytes: u32 }, // 3,
    Long { high_bytes: u32, low_bytes: u32 }, // 5,
    Double { high_bytes: u32, low_bytes: u32 }, // 6,
    Class { name_index: u16 }, // 7
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
                    _ => Some(ConstantType::Unknown)
                }
            },
            None => None
        }
    }
}

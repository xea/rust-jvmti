use self::immutable::ImmutableCollection;
use self::collections::{ ReadMapper, ReadChunks };

pub mod attribute;
pub mod collections;
pub mod immutable;

pub trait BytecodeItem {
    fn from_bytes(bytes: &Vec<u8>) -> Self;
    fn to_bytes(&self) -> Vec<u8>;
}

pub struct Class {
    version: ClassfileVersion,
}

pub struct ClassfileVersion {
    pub minor_version: u16,
    pub major_version: u16
}

pub struct ConstantPoolIndex {
    index: u16
}

impl BytecodeItem for Class {

    fn from_bytes(bytes: &Vec<u8>) -> Self {
        Class {
            version: ClassfileVersion::default()
        }
    }

    fn to_bytes(&self) -> Vec<u8> {
        let mut b: Vec<u8> = vec![];
        b.append(&mut self.version.to_bytes());

        b
    }
}

impl Default for ClassfileVersion {
    fn default() -> Self {
        ClassfileVersion { minor_version: 0, major_version: 52 }
    }
}

impl BytecodeItem for ClassfileVersion {

    fn from_bytes(bytes: &Vec<u8>) -> Self {
        const MIN_VERSION_LENGTH: usize = 4;

        if bytes.len() >= MIN_VERSION_LENGTH {
            ClassfileVersion { minor_version: 14, major_version: 52 }
        } else {
            ClassfileVersion::default()
        }
    }

    fn to_bytes(&self) -> Vec<u8> {
        vec![ self.minor_version, self.major_version ].iter().map(|x| vec![ (x & 0xff00 >> 8) as u8, (x & 0xff) as u8 ]).flat_map(|x| x).collect()
    }
}

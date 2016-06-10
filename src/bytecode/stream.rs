use super::*;

pub trait BytecodeItem {
    fn from_bytes(bytes: &Vec<u8>) -> Self;
    fn to_bytes(&self) -> Vec<u8>;
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
        vec![ self.minor_version, self.major_version ]
            .iter()
            .map(|x| vec![ (x & 0xff00 >> 8) as u8, (x & 0xff) as u8 ])
            .flat_map(|x| x).collect()
    }
}

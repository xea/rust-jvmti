use super::stream::ClassOutputStream;
use super::stream::ClassInputStream;
use super::stream::ClassInputStreamError;
use super::stream::ClassStreamEntry;
use super::stream::ReadChunks;
use super::stream::WriteChunks;
use super::constant::ConstantPool;

#[derive(Default)]
pub struct Class {
    pub version: ClassfileVersion,
    pub constant_pool: ConstantPool
}

pub struct ClassfileVersion {
    pub minor_version: u16,
    pub major_version: u16
}

impl ClassfileVersion {
    pub fn new(major_version: u16, minor_version: u16) -> ClassfileVersion {
        ClassfileVersion { major_version: major_version, minor_version: minor_version }
    }

    pub fn has_lambdas(&self) -> bool {
        self.major_version >= 52
    }
}

impl Default for ClassfileVersion {
    fn default() -> ClassfileVersion {
        ClassfileVersion { major_version: 52, minor_version: 0 }
    }
}


impl ClassStreamEntry for ClassfileVersion {
    fn read_element(stream: &ClassInputStream) -> Result<Self, ClassInputStreamError> {
        match (stream.read_u16(), stream.read_u16()) {
            (Some(minor_version), Some(major_version)) => Ok(ClassfileVersion::new(major_version, minor_version)),
            _ => Err(ClassInputStreamError::PrematureEnd)
        }
    }

    fn write_element(&self, stream: &mut ClassOutputStream) {
        stream.write_u16(self.minor_version);
        stream.write_u16(self.major_version);
    }
}

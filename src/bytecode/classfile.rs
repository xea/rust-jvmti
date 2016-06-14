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
    pub constant_pool: ConstantPool,
    pub access_flags: AccessFlags,
    pub this_class: ConstantPoolIndex,
    pub super_class: ConstantPoolIndex,
    pub interfaces: Vec<ConstantPoolIndex>
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

#[derive(Default)]
pub struct AccessFlags {
    flags: u16
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
    PUBLIC = 0x0001,
    FINAL = 0x0010,
    SUPER = 0x0020,
    INTERFACE = 0x0200,
    ABSTRACT = 0x0400,
    SYNTHETIC = 0x1000,
    ANNOTATION = 0x2000,
    ENUM = 0x4000
}

#[derive(Default)]
pub struct ConstantPoolIndex {
    pub idx: u16
}

impl ConstantPoolIndex {
    pub fn new() -> ConstantPoolIndex { ConstantPoolIndex::of(0) }
    pub fn of(idx: u16) -> ConstantPoolIndex { ConstantPoolIndex { idx: idx }}
}

impl ClassStreamEntry for ConstantPoolIndex {

    fn read_element(stream: &ClassInputStream) -> Result<Self, ClassInputStreamError> {
        match stream.read_u16() {
            Some(val) => Ok(ConstantPoolIndex::of(val)),
            _ => Err(ClassInputStreamError::PrematureEnd)
        }
    }

    fn write_element(&self, stream: &mut ClassOutputStream) {
        stream.write_u16(self.idx);
    }
}

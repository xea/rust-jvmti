use super::stream::*;
use super::constant::ConstantPool;

pub struct Attribute {
    pub name_index: u16,
    pub info: Vec<u8>
}

impl Attribute {
    pub fn resolve_element(stream: &ClassInputStream) -> Result<Attribute, ClassInputStreamError> {
        match (stream.read_u16(), stream.read_u32()) {
            (Some(name_index), Some(att_len)) => {
                match stream.read_n(att_len as usize) {
                    Some(bytes) => Ok(Attribute { name_index: name_index, info: bytes }),
                    None => Err(ClassInputStreamError::PrematureEnd)
                }
            },
            (_, _) => Err(ClassInputStreamError::PrematureEnd)
        }
    }

/*
    pub fn resolve_constant_value(stream: &ClassInputStream, len: u32) -> Result<Attribute, ClassInputStreamError> {
        match (len, stream.read_u16()) {
            (2, Some(constant_idx)) => Ok(Attribute::ConstantValue(constant_idx)),
            (_, Some(_)) => Err(ClassInputStreamError::InvalidAttribute),
            _ => Err(ClassInputStreamError::PrematureEnd)
        }
    }
    */
}

impl ClassStreamEntry for Attribute {

    fn read_element(_: &ClassInputStream) -> Result<Self, ClassInputStreamError> {
        Err(ClassInputStreamError::MissingConstantPool)
    }

    fn write_element(&self, _: &mut ClassOutputStream) {
    }
}

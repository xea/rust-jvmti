use super::stream::*;

#[derive(Default)]
pub struct ConstantPool {
    pub constants: Vec<Constant>
}

impl ConstantPool {
    pub fn from_vec(vec: Vec<Constant>) -> ConstantPool {
        ConstantPool {
            constants: vec
        }
    }

    pub fn len(&self) -> usize {
        self.constants.len()
    }

    pub fn get(&self, idx: usize) -> Option<&Constant> {
        self.constants.get(idx)
    }
}

impl ClassStreamEntry for ConstantPool {
    fn read_element(stream: &ClassInputStream) -> Result<Self, ClassInputStreamError> {
        match stream.read_u16() {
            Some(cp_len) => {
                Ok(ConstantPool { constants: vec![] })
            },
            None => Err(ClassInputStreamError::PrematureEnd)
        }
    }

    fn write_element(&self, stream: &mut ClassOutputStream) {
        stream.write_u16(self.len() as u16);

        for constant in self.constants.iter() {
            constant.write_element(stream);
        }
    }
}

pub enum Constant {
    Integer(u32),
    Unknown
}

impl Constant {
    pub fn is_long_entry(&self) -> bool {
        match *self {
            _ => false
        }
    }
}

impl ClassStreamEntry for Constant {

    fn read_element(stream: &ClassInputStream) -> Result<Self, ClassInputStreamError> {
        match stream.read_u8() {
            Some(3) => match stream.read_u32() {
                Some(val) => Ok(Constant::Integer(val)),
                None => Err(ClassInputStreamError::PrematureEnd)
            },
            Some(tag) => Err(ClassInputStreamError::InvalidConstantTag(tag)),
            _ => Err(ClassInputStreamError::PrematureEnd)
        }
    }

    fn write_element(&self, stream: &mut ClassOutputStream) {
        match *self {
            Constant::Integer(value) => {
                stream.write_u8(3);
                stream.write_u32(value);
            },
            _ => ()
        }
    }
}

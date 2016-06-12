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
    Utf8(Vec<u8>),
    Integer(u32),
    Float(f32),
    Long(u64),
    Double(f64),
    Class(u16),
    String(u16),
    FieldRef { class_index: u16, name_and_type_index: u16 },
    MethodRef { class_index: u16, name_and_type_index: u16 },
    InterfaceMethodRef { class_index: u16, name_and_type_index: u16 },
    NameAndType { name_index: u16, descriptor_index: u16 },
    MethodHandle { reference_kind: u8, reference_index: u16 },
    MethodType(u16),
    InvokeDynamic { bootstrap_method_attr_index: u16, name_and_type_index: u16 },
    Placeholder,
    Unknown
}

impl Constant {
    pub fn is_long_entry(&self) -> bool {
        match *self {
            Constant::Long(_) => true,
            Constant::Double(_) => true,
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
            Some(4) => match stream.read_u32() {
                Some(val) => Ok(Constant::Float(val as f32)),
                None => Err(ClassInputStreamError::PrematureEnd)
            },
            Some(5) => match stream.read_u64() {
                Some(val) => Ok(Constant::Long(val)),
                None => Err(ClassInputStreamError::PrematureEnd)
            },
            Some(6) => match stream.read_u64() {
                Some(val) => Ok(Constant::Double(val as f64)),
                None => Err(ClassInputStreamError::PrematureEnd)
            },
            Some(7) => match stream.read_u16() {
                Some(val) => Ok(Constant::Class(val)),
                None => Err(ClassInputStreamError::PrematureEnd)
            },
            Some(8) => match stream.read_u16() {
                Some(val) => Ok(Constant::String(val)),
                None => Err(ClassInputStreamError::PrematureEnd)
            },
            Some(9) => match (stream.read_u16(), stream.read_u16()) {
                (Some(class_idx), Some(nametype_idx)) => Ok(Constant::FieldRef { class_index: class_idx, name_and_type_index: nametype_idx }),
                _ => Err(ClassInputStreamError::PrematureEnd)
            },
            Some(10) => match (stream.read_u16(), stream.read_u16()) {
                (Some(class_idx), Some(nametype_idx)) => Ok(Constant::MethodRef { class_index: class_idx, name_and_type_index: nametype_idx }),
                _ => Err(ClassInputStreamError::PrematureEnd)
            },
            Some(11) => match (stream.read_u16(), stream.read_u16()) {
                (Some(class_idx), Some(nametype_idx)) => Ok(Constant::InterfaceMethodRef { class_index: class_idx, name_and_type_index: nametype_idx }),
                _ => Err(ClassInputStreamError::PrematureEnd)
            },
            Some(12) => match (stream.read_u16(), stream.read_u16()) {
                (Some(name_idx), Some(desc_idx)) => Ok(Constant::NameAndType { name_index: name_idx, descriptor_index: desc_idx }),
                _ => Err(ClassInputStreamError::PrematureEnd)
            },
            Some(15) => match (stream.read_u8(), stream.read_u16()) {
                (Some(ref_kind), Some(ref_idx)) => Ok(Constant::MethodHandle { reference_kind: ref_kind, reference_index: ref_idx }),
                _ => Err(ClassInputStreamError::PrematureEnd)
            },
            Some(16) => match stream.read_u16() {
                Some(desc_idx) => Ok(Constant::MethodType(desc_idx)),
                _ => Err(ClassInputStreamError::PrematureEnd)
            },
            Some(18) => match (stream.read_u16(), stream.read_u16()) {
                (Some(bootstrap_idx), Some(nametype_idx)) => Ok(Constant::InvokeDynamic { bootstrap_method_attr_index: bootstrap_idx, name_and_type_index: nametype_idx }),
                _ => Err(ClassInputStreamError::PrematureEnd)
            },
            Some(1) => match stream.read_u16() {
                Some(len) => match stream.read_n(len as usize) {
                    Some(bytes) => Ok(Constant::Utf8(bytes)),
                    _ => Err(ClassInputStreamError::PrematureEnd)
                },
                _ => Err(ClassInputStreamError::PrematureEnd)

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
            Constant::Float(value) => {
                stream.write_u8(4);
                stream.write_u32(value as u32);
            },
            Constant::Long(value) => {
                stream.write_u8(5);
                stream.write_u64(value);
            },
            Constant::Double(value) => {
                stream.write_u8(6);
                stream.write_u64(value as u64);
            },
            Constant::Class(value) => {
                stream.write_u8(7);
                stream.write_u16(value);
            },
            Constant::String(value) => {
                stream.write_u8(8);
                stream.write_u16(value);
            },
            _ => ()
        }
    }
}

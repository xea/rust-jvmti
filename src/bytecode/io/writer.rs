use std::io::{ Write, Error, ErrorKind };
use super::super::classfile::*;

pub struct ClassWriter<'a> {
    target: &'a mut Write
}

impl<'a> ClassWriter<'a> {
    pub fn new<T>(target: &'a mut T) -> ClassWriter where T: Write {
        ClassWriter { target: target }
    }

    pub fn write_class(&mut self, classfile: &Classfile) -> Result<usize, Error> {
        self.write_magic_bytes()
        .and(self.write_classfile_version(&classfile.version))
        .and(self.write_constant_pool(&classfile.constant_pool))
        .and(self.write_access_flags(&classfile.access_flags))
        .and(self.write_constant_pool_index(&classfile.this_class))
        .and(self.write_constant_pool_index(&classfile.super_class))
        .and(self.write_interfaces(&classfile.interfaces))
        .and(self.write_fields(&classfile.fields, &classfile.constant_pool))
        .and(self.write_methods(&classfile.methods, &classfile.constant_pool))
        .and(self.write_attributes(&classfile.attributes, &classfile.constant_pool))
    }

    pub fn write_magic_bytes(&mut self) -> Result<usize, Error> {
        self.write_u32(0xCAFEBABE)
    }

    pub fn write_classfile_version(&mut self, version: &ClassfileVersion) -> Result<usize, Error> {
        self.write_u16(version.minor_version)
        .and(self.write_u16(version.major_version))
    }

    pub fn write_constant_pool(&mut self, cp: &ConstantPool) -> Result<usize, Error> {
        cp.constants.iter().fold(self.write_u16(cp.cp_len() as u16), |acc, x| {
            match acc {
                Ok(ctr) => self.write_constant(x).map(|c| c + ctr),
                err@_ => err
            }
        })
    }

    fn write_constant(&mut self, constant: &Constant) -> Result<usize, Error> {
        match constant {
            &Constant::Utf8(ref bytes) => self.write_u8(1).and(self.write_u16(bytes.len() as u16)).and(self.write_n(bytes)),
            &Constant::Integer(ref value) => self.write_u8(3).and(self.write_u32(*value)),
            &Constant::Float(ref value) => self.write_u8(4).and(self.write_u32(*value)),
            &Constant::Long(ref value) => self.write_u8(5).and(self.write_u64(*value)),
            &Constant::Double(ref value) => self.write_u8(6).and(self.write_u64(*value)),
            &Constant::Class(ref idx) => self.write_u8(7).and(self.write_u16(idx.idx as u16)),
            &Constant::String(ref idx) => self.write_u8(8).and(self.write_u16(idx.idx as u16)),
            &Constant::MethodType(ref idx) => self.write_u8(16).and(self.write_u16(idx.idx as u16)),
            &Constant::FieldRef { class_index: ref c_idx, name_and_type_index: ref n_idx } => self.write_u8(9).and(self.write_u16(c_idx.idx as u16)).and(self.write_u16(n_idx.idx as u16)),
            &Constant::MethodRef { class_index: ref c_idx, name_and_type_index: ref n_idx } => self.write_u8(10).and(self.write_u16(c_idx.idx as u16)).and(self.write_u16(n_idx.idx as u16)),
            &Constant::InterfaceMethodRef { class_index: ref c_idx, name_and_type_index: ref n_idx } => self.write_u8(11).and(self.write_u16(c_idx.idx as u16)).and(self.write_u16(n_idx.idx as u16)),
            &Constant::NameAndType { name_index: ref n_idx, descriptor_index: ref d_idx } => self.write_u8(12).and(self.write_u16(n_idx.idx as u16)).and(self.write_u16(d_idx.idx as u16)),
            &Constant::MethodHandle { reference_kind: ref kind, reference_index: ref r_idx } => self.write_u8(15).and(self.write_u8(kind.to_u8())).and(self.write_u16(r_idx.idx as u16)),
            &Constant::Placeholder => Ok(0),
            _ => Err(Error::new(ErrorKind::InvalidData, "Unknown constant detected"))
        }
    }

    fn write_access_flags(&mut self, flags: &AccessFlags) -> Result<usize, Error> {
        self.write_u16(flags.flags)
    }

    fn write_constant_pool_index(&mut self, class_index: &ConstantPoolIndex) -> Result<usize, Error> {
        self.write_u16(class_index.idx as u16)
    }

    fn write_interfaces(&mut self, ifs: &Vec<ConstantPoolIndex>) -> Result<usize, Error> {
        ifs.iter().fold(self.write_u16(ifs.len() as u16), |acc, x| {
            match acc {
                Ok(ctr) => self.write_u16(x.idx as u16).map(|c| c + ctr),
                err@_ => err
            }
        })
    }

    fn write_fields(&mut self, fields: &Vec<Field>, cp: &ConstantPool) -> Result<usize, Error> {
        fields.iter().fold(self.write_u16(fields.len() as u16), |acc, x| {
            match acc {
                Ok(ctr) => self.write_field(x, cp).map(|c| c + ctr),
                err@_ => err
            }
        })
    }

    fn write_field(&mut self, field: &Field, cp: &ConstantPool) -> Result<usize, Error> {
        self.write_access_flags(&field.access_flags)
            .and(self.write_constant_pool_index(&field.name_index))
            .and(self.write_constant_pool_index(&field.descriptor_index))
            .and(self.write_attributes(&field.attributes, cp))
    }

    fn write_methods(&mut self, methods: &Vec<Method>, cp: &ConstantPool) -> Result<usize, Error> {
        methods.iter().fold(self.write_u16(methods.len() as u16), |acc, x| {
            match acc {
                Ok(ctr) => self.write_method(x, cp).map(|c| c + ctr),
                err@_ => err
            }
        })
    }

    fn write_method(&mut self, method: &Method, cp: &ConstantPool) -> Result<usize, Error> {
        self.write_access_flags(&method.access_flags)
            .and(self.write_constant_pool_index(&method.name_index))
            .and(self.write_constant_pool_index(&method.descriptor_index))
            .and(self.write_attributes(&method.attributes, cp))
    }

    fn write_attributes(&mut self, attributes: &Vec<Attribute>, cp: &ConstantPool) -> Result<usize, Error> {
        attributes.iter().fold(self.write_u16(attributes.len() as u16), |acc, x| {
            match acc {
                Ok(ctr) => self.write_attribute(x, cp).map(|c| c + ctr),
                err@_ => err
            }
        })
    }

    fn write_attribute(&mut self, attribute: &Attribute, cp: &ConstantPool) -> Result<usize, Error> {
        match attribute {
            &Attribute::RawAttribute { name_index: ref n_idx, info: ref bytes } => self.write_u16(n_idx.idx as u16).and(self.write_u32(bytes.len() as u32)).and(self.write_n(bytes)),
            _ => Ok(0)
        }
    }

    pub fn write_n(&mut self, bytes: &Vec<u8>) -> Result<usize, Error> {
        bytes.iter().fold(Ok(0), |acc, x| match acc {
            Ok(ctr) => self.write_u8(*x).map(|c| c + ctr),
            err@_ => err
        })
    }

    pub fn write_u64(&mut self, value: u64) -> Result<usize, Error> {
        let buf: [u8; 8] = [
            ((value & 0xFF << 56) >> 56) as u8,
            ((value & 0xFF << 48) >> 48) as u8,
            ((value & 0xFF << 40) >> 40) as u8,
            ((value & 0xFF << 32) >> 32) as u8,
            ((value & 0xFF << 24) >> 24) as u8,
            ((value & 0xFF << 16) >> 16) as u8,
            ((value & 0xFF << 8) >> 8) as u8,
            (value & 0xFF) as u8
        ];

        self.target.write(&buf)
    }

    pub fn write_u32(&mut self, value: u32) -> Result<usize, Error> {
        let buf: [u8; 4] = [
            ((value & 0xFF << 24) >> 24) as u8,
            ((value & 0xFF << 16) >> 16) as u8,
            ((value & 0xFF << 8) >> 8) as u8,
            (value & 0xFF) as u8
        ];

        self.target.write(&buf)
    }

    pub fn write_u16(&mut self, value: u16) -> Result<usize, Error> {
        let buf: [u8; 2] = [((value & 0xFF00) >> 8) as u8, (value & 0xFF) as u8];

        self.target.write(&buf)
    }

    pub fn write_u8(&mut self, value: u8) -> Result<usize, Error> {
        self.target.write(&[value])
    }
}

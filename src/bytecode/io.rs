use std::io::{ Cursor, Read, Write, Error, ErrorKind };
use super::classfile::*;

pub struct ClassReader {
}

impl ClassReader {

    pub fn read_class<T>(source: &mut T) -> Result<Classfile, Error> where T: Read {
        let mut reader = BlockReader::new(source);

        let fns: Vec<fn(&mut BlockReader, &ClassFragment) -> Result<ClassFragment, Error>> = vec![
            ClassReader::read_magic_bytes,
            ClassReader::read_classfile_version,
            ClassReader::read_constant_pool,
            ClassReader::read_access_flags,
            ClassReader::read_this_class,
            ClassReader::read_super_class,
            ClassReader::read_interfaces,
            ClassReader::read_fields,
            ClassReader::read_methods,
            ClassReader::read_class_attributes
        ];

        let result = fns.iter().fold(Ok(ClassFragment::default()), |acc, x| {
            match acc {
                Ok(acc_fragment) => match x(&mut reader, &acc_fragment) {
                    Ok(cur_fragment) => Ok(acc_fragment.merge(cur_fragment)),
                    err@_ => err
                },
                err@_ => err
            }
        });

        match result {
            Ok(fragment) => Ok(fragment.to_class()),
            Err(err) => Err(err)
        }
    }

    fn read_magic_bytes(reader: &mut BlockReader, _: &ClassFragment) -> Result<ClassFragment, Error> {
        match reader.read_u32() {
            Ok(0xCAFEBABE) => Ok(ClassFragment::default()),
            _ => Err(Error::new(ErrorKind::InvalidData, "Invalid magic bytes"))
        }
    }

    fn read_classfile_version(reader: &mut BlockReader, _: &ClassFragment) -> Result<ClassFragment, Error> {
        match (reader.read_u16(), reader.read_u16()) {
            (Ok(minor_version), Ok(major_version)) => {
                Ok(ClassFragment {
                    version: Some(ClassfileVersion::new(major_version, minor_version)),
                    ..Default::default()
                })
            },
            _ => Err(Error::new(ErrorKind::UnexpectedEof, "Could not read classfile version number"))
        }
    }

    fn read_constant_pool(reader: &mut BlockReader, _: &ClassFragment) -> Result<ClassFragment, Error> {
        match reader.read_u16() {
            Ok(cp_len) => {
                let mut constants: Vec<Constant> = vec![ Constant::Placeholder ];

                for _ in 1..cp_len {
                    if constants.len() < cp_len as usize {
                        match ClassReader::read_constant(reader) {
                            Ok(constant) => {
                                let constant_size = constant.cp_size();

                                constants.push(constant);

                                for _ in 1..constant_size {
                                    constants.push(Constant::Placeholder);
                                }
                            },
                            Err(err) => return Err(err)
                        }
                    }
                }

                Ok(ClassFragment {
                    constant_pool: Some(ConstantPool::new(constants)),
                    ..Default::default()
                })
            },
            Err(err) => Err(err)
        }
    }

    fn read_constant(reader: &mut BlockReader) -> Result<Constant, Error> {
        let tag = reader.read_u8();

        match tag {
            Ok(1) => match reader.read_u16() {
                Ok(str_len) => match reader.read_n(str_len as usize) {
                    Ok(bytes) => {
                        Ok(Constant::Utf8(bytes))
                    },
                    Err(err) => Err(err)
                },
                Err(err) => Err(err)
            },
            Ok(3) => reader.read_u32().map(|value| Constant::Integer(value)),
            Ok(4) => reader.read_u32().map(|value| Constant::Float(value)),
            Ok(5) => reader.read_u64().map(|value| Constant::Long(value)),
            Ok(6) => reader.read_u64().map(|value| Constant::Double(value)),
            Ok(7) => reader.read_u16().map(|idx| Constant::Class(ConstantPoolIndex::new(idx as usize))),
            Ok(8) => reader.read_u16().map(|idx| Constant::String(ConstantPoolIndex::new(idx as usize))),
            Ok(9) => ClassReader::require_n(reader, 4, |mut r| Constant::FieldRef {
                class_index: ConstantPoolIndex::new(r.get_u16() as usize),
                name_and_type_index: ConstantPoolIndex::new(r.get_u16() as usize)
            }),
            Ok(10) => ClassReader::require_n(reader, 4, |mut r| Constant::MethodRef {
                class_index: ConstantPoolIndex::new(r.get_u16() as usize),
                name_and_type_index: ConstantPoolIndex::new(r.get_u16() as usize)
            }),
            Ok(11) => ClassReader::require_n(reader, 4, |mut r| Constant::InterfaceMethodRef {
                class_index: ConstantPoolIndex::new(r.get_u16() as usize),
                name_and_type_index: ConstantPoolIndex::new(r.get_u16() as usize)
            }),
            Ok(12) => ClassReader::require_n(reader, 4, |mut r| Constant::NameAndType {
                    name_index: ConstantPoolIndex::new(r.get_u16() as usize),
                    descriptor_index: ConstantPoolIndex::new(r.get_u16() as usize)
            }),
            Ok(15) => ClassReader::require_n(reader, 3, |mut r| Constant::MethodHandle {
                reference_kind: ReferenceKind::from_u8(r.get_u8()),
                reference_index: ConstantPoolIndex::new(r.get_u16() as usize)
            }),
            Ok(16) => reader.read_u16().map(|idx| Constant::MethodType(ConstantPoolIndex::new(idx as usize))),
            Ok(18) => ClassReader::require_n(reader, 4, |mut r| Constant::InvokeDynamic {
                bootstrap_method_attr_index: ConstantPoolIndex::new(r.get_u16() as usize),
                name_and_type_index: ConstantPoolIndex::new(r.get_u16() as usize)
            }),
            Ok(tag) => Ok(Constant::Unknown(tag)),
            Err(err) => Err(err)
        }
    }

    fn read_access_flags(reader: &mut BlockReader, _: &ClassFragment) -> Result<ClassFragment, Error> {
        match reader.read_u16() {
            Ok(val) => Ok(ClassFragment {
                access_flags: Some(AccessFlags::of(val)),
                ..Default::default()
            }),
            Err(err) => Err(err)
        }
    }

    fn read_this_class(reader: &mut BlockReader, _: &ClassFragment) -> Result<ClassFragment, Error> {
        match ClassReader::read_constant_pool_index(reader) {
            Ok(idx) => Ok(ClassFragment {
                this_class: Some(idx),
                ..Default::default()
            }),
            Err(err) => Err(err)
        }
    }

    fn read_super_class(reader: &mut BlockReader, _: &ClassFragment) -> Result<ClassFragment, Error> {
        match ClassReader::read_constant_pool_index(reader) {
            Ok(idx) => Ok(ClassFragment {
                super_class: Some(idx),
                ..Default::default()
            }),
            Err(err) => Err(err)
        }
    }

    fn read_interfaces(reader: &mut BlockReader, _: &ClassFragment) -> Result<ClassFragment, Error> {
        match reader.read_u16() {
            Ok(ifs_len) => {
                (0..ifs_len).fold(Ok(vec![]), |acc, _| {
                    match acc {
                        Ok(mut ifs) => match ClassReader::read_constant_pool_index(reader) {
                            Ok(interface) => {
                                ifs.push(interface);
                                Ok(ifs)
                            },
                            Err(err) => Err(err)
                        },
                        err@_ => err
                    }
                })
            },
            Err(err) => Err(err)
        }.map(|ifs| ClassFragment {
            interfaces: Some(ifs),
            ..Default::default()
        })
    }

    fn read_fields(reader: &mut BlockReader, cf: &ClassFragment) -> Result<ClassFragment, Error> {
        match reader.read_u16() {
            Ok(fields_len) => {
                (0..fields_len).fold(Ok(vec![]), |acc, _| {
                    match acc {
                        Ok(mut fields) => match ClassReader::read_field(reader, cf) {
                            Ok(field) => {
                                fields.push(field);
                                Ok(fields)
                            },
                            Err(err) => Err(err)
                        },
                        err@_ => err
                    }
                })
            },
            Err(err) => Err(err)
        }.map(|fields| ClassFragment {
            fields: Some(fields),
            ..Default::default()
        })
    }

    fn read_field(reader: &mut BlockReader, cf: &ClassFragment) -> Result<Field, Error> {
        match ClassReader::require_n(reader, 6, |mut r| { (r.get_u16(), r.get_u16(), r.get_u16()) }) {
            Ok((flags, n_idx, d_idx)) => match ClassReader::read_attributes(reader, cf) {
                Ok(attributes) => Ok(Field {
                    access_flags: AccessFlags::of(flags),
                    name_index: ConstantPoolIndex::new(n_idx as usize),
                    descriptor_index: ConstantPoolIndex::new(d_idx as usize),
                    attributes: attributes
                }),
                Err(err) => Err(err)
            },
            Err(err) => Err(err)
        }
    }

    fn read_methods(reader: &mut BlockReader, cf: &ClassFragment) -> Result<ClassFragment, Error> {
        match reader.read_u16() {
            Ok(methods_len) => {
                (0..methods_len).fold(Ok(vec![]), |acc, _| {
                    match acc {
                        Ok(mut methods) => match ClassReader::read_method(reader, cf) {
                            Ok(method) => {
                                methods.push(method);
                                Ok(methods)
                            },
                            Err(err) => Err(err)
                        },
                        err@_ => err
                    }
                })
            },
            Err(err) => Err(err)
        }.map(|methods| ClassFragment {
            methods: Some(methods),
            ..Default::default()
        })
    }

    fn read_method(reader: &mut BlockReader, cf: &ClassFragment) -> Result<Method, Error> {
        match ClassReader::require_n(reader, 6, |mut r| { (r.get_u16(), r.get_u16(), r.get_u16()) }) {
            Ok((flags, n_idx, d_idx)) => match ClassReader::read_attributes(reader, cf) {
                Ok(attributes) => Ok(Method {
                    access_flags: AccessFlags::of(flags),
                    name_index: ConstantPoolIndex::new(n_idx as usize),
                    descriptor_index: ConstantPoolIndex::new(d_idx as usize),
                    attributes: attributes
                }),
                Err(err) => Err(err)
            },
            Err(err) => Err(err)
        }
    }

    fn read_class_attributes(reader: &mut BlockReader, cf: &ClassFragment) -> Result<ClassFragment, Error> {
        match ClassReader::read_attributes(reader, cf) {
            Ok(attributes) => Ok(ClassFragment {
                attributes: Some(attributes),
                ..Default::default()
            }),
            Err(err) => Err(err)
        }
    }

    fn read_attributes(reader: &mut BlockReader, cf: &ClassFragment) -> Result<Vec<Attribute>, Error> {
        match reader.read_u16() {
            Ok(attr_len) => (0..attr_len).fold(Ok(vec![]), |acc, _| {
                match acc {
                    Ok(mut attributes) => match ClassReader::read_attribute(reader, cf) {
                        Ok(attribute) => {
                            attributes.push(attribute);
                            Ok(attributes)
                        },
                        Err(err) => Err(err)
                    },
                    err@_ => err
                }
            }),
            Err(err) => Err(err)
        }
    }

    fn read_attribute(reader: &mut BlockReader, cf: &ClassFragment) -> Result<Attribute, Error> {
        match reader.read_u16() {
            Ok(n_idx) => match reader.read_u32() {
                Ok(a_len) => match reader.read_n(a_len as usize) {
                    Ok(mut bytes) => Ok(ClassReader::parse_attribute(n_idx, BlockReader::new(&mut Cursor::new(&mut bytes)), cf)),
                    Err(err) => Err(err)
                },
                Err(err) => Err(err)
            },
            Err(err) => Err(err)
        }
    }

    fn parse_attribute(idx: u16, mut reader: BlockReader, cf: &ClassFragment) -> Attribute {
        match cf.constant_pool {
            Some(ref cp) => match cp.get_utf8_string(idx) {
                Some(ref s) => match s.as_str() {
                    "ConstantValue" => Some(Attribute::ConstantValue(ConstantPoolIndex::new(reader.get_u16() as usize))),
                    "Code" => Some(Attribute::Code {
                        max_stack: reader.get_u16(),
                        max_locals: reader.get_u16(),
                        code: {
                            let n = reader.get_u32();
                            reader.get_n(n as usize)
                        },
                        exception_table: {
                            let n = reader.get_u16();
                            (0..n).map(|_| ExceptionHandler { start_pc: reader.get_u16(), end_pc: reader.get_u16(), handler_pc: reader.get_u16(), catch_type: ConstantPoolIndex::new(reader.get_u16() as usize) }).collect()
                        },
                        attributes: ClassReader::read_attributes(&mut reader, cf).unwrap_or(vec![])
                        }),
                    "StackMapTable" => None, // TODO
                    "Exceptions" => Some(Attribute::Exceptions({
                        let n = reader.get_u16();
                        (0..n).map(|_| ConstantPoolIndex::new(reader.get_u16() as usize)).collect()
                        })),
                    "InnerClass" => Some(Attribute::InnerClass({
                        let n = reader.get_u16();
                        (0..n).map(|_| InnerClass {
                            inner_class_info_index: ConstantPoolIndex::new(reader.get_u16() as usize),
                            outer_class_info_index: ConstantPoolIndex::new(reader.get_u16() as usize),
                            inner_name_index: ConstantPoolIndex::new(reader.get_u16() as usize),
                            access_flags: AccessFlags::of(reader.get_u16())
                            }).collect()
                        })),
                    "EnclosingMethod" => Some(Attribute::EnclosingMethod { class_index: ConstantPoolIndex::new(reader.get_u16() as usize), method_index: ConstantPoolIndex::new(reader.get_u16() as usize)}),
                    "Synthetic" => Some(Attribute::Synthetic),
                    "Signature" => Some(Attribute::Signature(ConstantPoolIndex::new(reader.get_u16() as usize))),
                    _ => None
                },
                _ => None
            },
            _ => None
        }.unwrap_or(Attribute::RawAttribute { name_index: ConstantPoolIndex::new(idx as usize), info: reader.get_bytes() })
    }

    fn read_constant_pool_index(reader: &mut BlockReader) -> Result<ConstantPoolIndex, Error> {
        match reader.read_u16() {
            Ok(idx) => Ok(ConstantPoolIndex::new(idx as usize)),
            Err(err) => Err(err)
        }
    }

    fn require_n<T, U>(reader: &mut BlockReader, count: usize, extractor: U) -> Result<T, Error> where U: Fn(BlockReader) -> T {
        match reader.read_n(count) {
            Ok(bytes) => {
                let mut cursor = Cursor::new(bytes);
                let r = BlockReader::new(&mut cursor);

                Ok(extractor(r))
            },
            Err(err) => Err(err)
        }
    }
}

// TODO remove pub after testing
pub struct BlockReader<'a> {
    source: &'a mut Read
}

impl<'a> BlockReader<'a> {

    pub fn new<T>(source: &'a mut T) -> BlockReader where T: Read {
        BlockReader { source: source }
    }

    pub fn read_u64(&mut self) -> Result<u64, Error> {
        let mut buf: [u8; 8] = [0; 8];

        match self.source.read_exact(&mut buf) {
            Ok(_) => Ok(
                ((buf[0] as u64) << 56) +
                ((buf[1] as u64) << 48) +
                ((buf[2] as u64) << 40) +
                ((buf[3] as u64) << 32) +
                ((buf[4] as u64) << 24) +
                ((buf[5] as u64) << 16) +
                ((buf[6] as u64) << 8) +
                buf[7] as u64),
            Err(err) => Err(err)
        }
    }

    #[allow(dead_code)]
    pub fn get_u64(&mut self) -> u64 {
        self.read_u64().unwrap_or(0)
    }

    pub fn read_u32(&mut self) -> Result<u32, Error> {
        let mut buf: [u8; 4] = [0; 4];

        match self.source.read_exact(&mut buf) {
            Ok(_) => Ok(
                ((buf[0] as u32) << 24) +
                ((buf[1] as u32) << 16) +
                ((buf[2] as u32) << 8) +
                buf[3] as u32),
            Err(err) => Err(err)
        }
    }

    #[allow(dead_code)]
    pub fn get_u32(&mut self) -> u32 {
        self.read_u32().unwrap_or(0)
    }

    pub fn read_u16(&mut self) -> Result<u16, Error> {
        let mut buf: [u8; 2] = [0; 2];

        match self.source.read_exact(&mut buf) {
            Ok(_) => Ok(((buf[0] as u16) << 8) + buf[1] as u16),
            Err(err) => Err(err)
        }
    }

    pub fn get_u16(&mut self) -> u16 {
        self.read_u16().unwrap_or(0)
    }

    pub fn read_u8(&mut self) -> Result<u8, Error> {
        let mut buf: [u8; 1] = [0; 1];

        match self.source.read_exact(&mut buf) {
            Ok(_) => Ok(buf[0]),
            Err(err) => Err(err)
        }
    }

    pub fn get_u8(&mut self) -> u8 {
        self.read_u8().unwrap_or(0)
    }

    pub fn read_n(&mut self, count: usize) -> Result<Vec<u8>, Error> {
        let mut tmp: Vec<u8> = Vec::with_capacity(count);

        match self.source.take(count as u64).read_to_end(&mut tmp) {
            Ok(_) => Ok(tmp),
            Err(err) => Err(err)
        }
    }

    pub fn get_n(&mut self, count: usize) -> Vec<u8> {
        match self.read_n(count) {
            Ok(bytes) => bytes,
            Err(_) => vec![]
        }
    }

    pub fn read_bytes(&mut self) -> Result<Vec<u8>, Error> {
        let mut tmp: Vec<u8> = vec![];

        match self.source.read_to_end(&mut tmp) {
            Ok(_) => Ok(tmp),
            Err(err) => Err(err)
        }
    }

    pub fn get_bytes(&mut self) -> Vec<u8> {
        let mut tmp: Vec<u8> = vec![];

        let _ = self.source.read_to_end(&mut tmp);

        tmp
    }
}

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
        .and(self.write_fields(&classfile.fields))
        .and(self.write_methods(&classfile.methods))
        .and(self.write_attributes(&classfile.attributes))
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

    fn write_fields(&mut self, fields: &Vec<Field>) -> Result<usize, Error> {
        fields.iter().fold(self.write_u16(fields.len() as u16), |acc, x| {
            match acc {
                Ok(ctr) => self.write_field(x).map(|c| c + ctr),
                err@_ => err
            }
        })
    }

    fn write_field(&mut self, field: &Field) -> Result<usize, Error> {
        self.write_access_flags(&field.access_flags)
            .and(self.write_constant_pool_index(&field.name_index))
            .and(self.write_constant_pool_index(&field.descriptor_index))
            .and(self.write_attributes(&field.attributes))
    }

    fn write_methods(&mut self, methods: &Vec<Method>) -> Result<usize, Error> {
        methods.iter().fold(self.write_u16(methods.len() as u16), |acc, x| {
            match acc {
                Ok(ctr) => self.write_method(x).map(|c| c + ctr),
                err@_ => err
            }
        })
    }

    fn write_method(&mut self, method: &Method) -> Result<usize, Error> {
        self.write_access_flags(&method.access_flags)
            .and(self.write_constant_pool_index(&method.name_index))
            .and(self.write_constant_pool_index(&method.descriptor_index))
            .and(self.write_attributes(&method.attributes))
    }

    fn write_attributes(&mut self, attributes: &Vec<Attribute>) -> Result<usize, Error> {
        attributes.iter().fold(self.write_u16(attributes.len() as u16), |acc, x| {
            match acc {
                Ok(ctr) => self.write_attribute(x).map(|c| c + ctr),
                err@_ => err
            }
        })
    }

    fn write_attribute(&mut self, attribute: &Attribute) -> Result<usize, Error> {
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

struct ClassFragment {
    pub version: Option<ClassfileVersion>,
    pub constant_pool: Option<ConstantPool>,
    pub access_flags: Option<AccessFlags>,
    pub this_class: Option<ConstantPoolIndex>,
    pub super_class: Option<ConstantPoolIndex>,
    pub interfaces: Option<Vec<ConstantPoolIndex>>,
    pub fields: Option<Vec<Field>>,
    pub methods: Option<Vec<Method>>,
    pub attributes: Option<Vec<Attribute>>
}

impl ClassFragment {
    pub fn merge(mut self, other: Self) -> Self {
        self.version = other.version.or(self.version);
        self.constant_pool = other.constant_pool.or(self.constant_pool);
        self.access_flags = other.access_flags.or(self.access_flags);
        self.this_class = other.this_class.or(self.this_class);
        self.super_class = other.super_class.or(self.super_class);
        self.interfaces = other.interfaces.or(self.interfaces);
        self.fields = other.fields.or(self.fields);
        self.methods = other.methods.or(self.methods);
        self.attributes = other.attributes.or(self.attributes);
        self
    }

    /// Transform this class fragment into a final class file. Members set on the fragment will
    /// be defined on the class too, other members will be initialized with their default values
    pub fn to_class(self) -> Classfile {
        Classfile {
            version: self.version.unwrap_or(ClassfileVersion::default()),
            constant_pool: self.constant_pool.unwrap_or(ConstantPool::default()),
            access_flags: self.access_flags.unwrap_or(AccessFlags::new()),
            this_class: self.this_class.unwrap_or(ConstantPoolIndex::default()),
            super_class: self.super_class.unwrap_or(ConstantPoolIndex::default()),
            interfaces: self.interfaces.unwrap_or(vec![]),
            fields: self.fields.unwrap_or(vec![]),
            methods: self.methods.unwrap_or(vec![]),
            attributes: self.attributes.unwrap_or(vec![])
        }
    }
}

impl Default for ClassFragment {
    fn default() -> Self {
        ClassFragment {
            version: None,
            constant_pool: None,
            access_flags: None,
            this_class: None,
            super_class: None,
            interfaces: None,
            fields: None,
            methods: None,
            attributes: None
        }
    }
}

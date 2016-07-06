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
            //ClassReader::read_super_class,
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
                (1..cp_len).fold(Ok(vec![ Constant::Placeholder ]), |acc, _| {
                    match (acc, acc.map(|v| v.len() < cp_len as usize - 1).unwrap_or(false)) {
                        (Ok(mut constants), true) => match ClassReader::read_constant(reader) {
                            Ok(constant) => Err(Error::new(ErrorKind::AddrInUse, "blablba")),
                            Err(err) => Err(err)
                            /*
                            Ok(constant) => {
                                let constant_oversize = constant.cp_size();

                                constants.push(constant);

                                for _ in 1..constant_oversize {
                                    constants.push(Constant::Placeholder)
                                }

                                Ok(constants)
                            },
                            Err(err) => Err(err)
                            */
                        },
                        (_, false) => acc,
                        (Err(err), _) => Err(err)
                    }
                }).map(|constants| ClassFragment {
                    constant_pool: Some(ConstantPool::new(constants)),
                    ..Default::default()
                })
                /*
                (1..cp_len).fold(Ok(vec![ Constant::Placeholder ]), |acc, _| {
                    match acc {
                        Ok(mut constants) => match ClassReader::read_constant(reader) {
                            Ok(constant) => {
                                let constant_oversize = constant.cp_size();

                                constants.push(constant);

                                for _ in 1..constant_oversize {
                                    constants.push(Constant::Placeholder)
                                }

                                Ok(constants)
                            },
                            Err(err) => Err(err)
                        },
                        err@_ => err
                    }
                }).map(|constants| ClassFragment {
                    constant_pool: Some(ConstantPool::new(constants)),
                    ..Default::default()
                })
                */
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

/*
        let r = match self.source.read_exact(tmp.as_mut_slice()) {
            Ok(_) => Ok(tmp),
            Err(err) => Err(err)
        };
        */
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

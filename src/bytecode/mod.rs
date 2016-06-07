use self::class_stream::ClassStream;
use self::classfile::*;
use self::constants::ConstantType;
use self::constants::AccessFlag;

pub mod constants;
pub mod classfile;
pub mod class_stream;

#[derive(Default)]
pub struct ClassFragment {
    major_version: Option<u16>,
    minor_version: Option<u16>,
    constant_pool: Option<Vec<ConstantType>>,
    access_flags: Option<AccessFlag>,
    this_class: Option<ConstantReference>,
    super_class: Option<ConstantReference>,
    interfaces: Option<Vec<ConstantReference>>,
    fields: Option<Vec<Field>>,
    methods: Option<Vec<Method>>,
    attributes: Option<Vec<Attribute>>
}

impl ClassFragment {
    pub fn new() -> ClassFragment {
        ClassFragment {
            ..Default::default()
        }
    }

    pub fn merge(mut self, other: Self) -> Self {
        self.major_version = other.major_version.or(self.major_version);
        self.minor_version = other.minor_version.or(self.minor_version);
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

    pub fn to_classfile(self) -> Classfile {
        Classfile {
            major_version: self.major_version.unwrap_or(Classfile::default_major_version()),
            minor_version: self.minor_version.unwrap_or(Classfile::default_minor_version()),
            constant_pool: self.constant_pool.unwrap_or(Classfile::default_constant_pool()),
            access_flags: self.access_flags.unwrap_or(AccessFlag::new()),
            this_class: self.this_class.unwrap_or(ConstantReference::unknown()),
            super_class: self.super_class.unwrap_or(ConstantReference::unknown()),
            interfaces: self.interfaces.unwrap_or(vec![]),
            fields: self.fields.unwrap_or(vec![]),
            methods: self.methods.unwrap_or(vec![]),
            attributes: self.attributes.unwrap_or(vec![])
        }
    }
}

pub struct ClassReader {
}

impl ClassReader {

    pub fn parse_bytes<'a>(bytes: &'a Vec<u8>) -> Result<Classfile, String> {
        let mut cs = ClassStream::new(bytes);

        let fns: Vec<fn(&mut ClassStream) -> Result<ClassFragment, String>> = vec![
            ClassReader::read_magic_bytes,
            ClassReader::read_version_number,
            ClassReader::read_constant_pool,
            ClassReader::read_class_access_flags,
            ClassReader::read_this_class,
            ClassReader::read_super_class
        ];

        let result: Result<ClassFragment, String> = fns.iter().fold(Ok(ClassFragment::new()), |acc, x| {
            match acc {
                Ok(fragment) => match x(&mut cs) {
                    Ok(xr) => Ok(xr.merge(fragment)),
                    err@Err(_) => err
                },
                err@Err(_) => err
            }
        });

        result.map(|i| i.to_classfile())
    }

    fn read_magic_bytes(stream: &mut ClassStream) -> Result<ClassFragment, String> {
        match stream.read_magic_bytes() {
            true => Ok(ClassFragment::new()),
            false => Err("Couldn't find class file magic constant (CAFEBABE)".to_string())
        }
    }

    fn read_version_number(stream: &mut ClassStream) -> Result<ClassFragment, String> {
        match stream.read_version_number() {
            Some((minor_version, major_version)) => Ok(ClassFragment {
                major_version: Some(major_version),
                minor_version: Some(minor_version),
                ..Default::default()
            }),
            _ => Err("Couldn't read class version number from stream".to_string())
        }
    }

    fn read_constant_pool(stream: &mut ClassStream) -> Result<ClassFragment, String> {
        match stream.read_constant_pool() {
            r@Some(_) => Ok(ClassFragment {
                constant_pool: r,
                ..Default::default()
            }),
            _ => Err("Failed to read constant pool from stream".to_string())
        }
    }

    fn read_class_access_flags(stream: &mut ClassStream) -> Result<ClassFragment, String> {
        match stream.read_class_access_flags() {
            r@Some(_) => Ok(ClassFragment {
                access_flags: r,
                ..Default::default()
            }),
            _ => Err("Failed to read or parse class access flag".to_string())
        }
    }

    fn read_this_class(stream: &mut ClassStream) -> Result<ClassFragment, String> {
        match stream.read_constant_reference() {
            r@Some(_) => Ok(ClassFragment {
                this_class: r,
                ..Default::default()
            }),
            _ => Err("Failed to read constant reference to this class".to_string())
        }
    }

    fn read_super_class(stream: &mut ClassStream) -> Result<ClassFragment, String> {
        match stream.read_constant_reference() {
            r@Some(_) => Ok(ClassFragment {
                this_class: r,
                ..Default::default()
            }),
            _ => Err("Failed to read constant reference to super class".to_string())
        }
    }
}

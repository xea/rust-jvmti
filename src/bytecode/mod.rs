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
        self
    }

    pub fn to_classfile(self) -> Classfile {
        Classfile {
            major_version: self.major_version.unwrap_or(Classfile::default_major_version()),
            minor_version: self.minor_version.unwrap_or(Classfile::default_minor_version()),
            constant_pool: self.constant_pool.unwrap_or(Classfile::default_constant_pool()),
            access_flags: self.access_flags.unwrap_or(AccessFlag::new())
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
}

//use super::native::*;
use libc::c_uchar;
use std::ops::Shl;

pub mod classfile;

#[allow(dead_code)]
pub enum ConstantType {
    Class { name_index: u16 }, // 7
    FieldRef { class_index: u16, name_and_type_index: u16 }, // 9
    MethodRef { class_index: u16, name_and_type_index: u16 }, // 10,
    InterfaceMethodRef { class_index: u16, name_and_type_index: u16 }, // 11
    String { string_index: u16 }, // 8,
    Integer { bytes: u32 }, // 3,
    Float { bytes: u32 }, // 4,
    Long { high_bytes: u32, low_bytes: u32 }, // 5,
    Double { high_bytes: u32, low_bytes: u32 }, // 6,
    NameAndType { name_index: u16, descriptor_index: u16 }, // 12,
    Utf8 { length: u16, bytes: Vec<u8> }, // 1,
    MethodHandle { reference_kind: u8, reference_index: u16 }, // 15,
    MethodType { descriptor_index: u16 }, // 16,
    InvokeDynamic { bootstrap_method_attr_index: u16, name_and_type_index: u16 }, // 18,
    Placeholder,
    Unknown
}

#[derive(Default)]
pub struct ClassfileFragment {
    pub major_version: Option<u16>,
    pub minor_version: Option<u16>,
    pub constant_pool: Option<Vec<ConstantType>>
}

type PartialRead = (ClassfileFragment, usize);

pub struct Classfile {
    pub major_version: u16,
    pub minor_version: u16,
    pub constant_pool: Vec<ConstantType>
}

impl ClassfileFragment {

    pub fn merge(self, other_fragment: ClassfileFragment) -> ClassfileFragment {
        ClassfileFragment {
            major_version: other_fragment.major_version.or(self.major_version),
            minor_version: other_fragment.minor_version.or(self.minor_version),
            constant_pool: other_fragment.constant_pool.or(self.constant_pool)
        }
    }

    pub fn update(&mut self, other_fragment: ClassfileFragment) {
        self.major_version = other_fragment.major_version.or(self.major_version);
        self.minor_version = other_fragment.minor_version.or(self.minor_version);
        if other_fragment.constant_pool.is_some() { self.constant_pool = other_fragment.constant_pool; }
    }

    pub fn to_classfile(self) -> Classfile {
        Classfile {
            major_version: self.major_version.or(Some(0x00)).unwrap(),
            minor_version: self.minor_version.or(Some(0x00)).unwrap(),
            constant_pool: self.constant_pool.or(Some(vec![])).unwrap()
        }
    }
}

pub struct ClassfileReader;

impl ClassfileReader {

    pub fn from_raw_bytes(raw_bytes: *const c_uchar, data_length: i32) -> Result<Classfile, String> {
        let mut buf: Vec<u8> = vec![];

        unsafe {
            for i in 0..data_length {
                buf.push(*(raw_bytes.offset(i as isize)));
            }
        }

        let bytes = buf.as_slice();

        ClassfileReader::from_bytes(bytes)
    }


    pub fn from_bytes(bytes: &[u8]) -> Result<Classfile, String> {
        let steps: Vec<fn(&[u8]) -> Result<(ClassfileFragment, usize), String>> = vec![
            ClassfileReader::read_magic,
            ClassfileReader::read_version_number,
            ClassfileReader::read_constant_pool
        ];

        ClassfileReader::read_bytes(bytes, steps)
    }

    pub fn read_bytes(bytes: &[u8], steps: Vec<fn(&[u8]) -> Result<(ClassfileFragment, usize), String>>) -> Result<Classfile, String> {
        let mut current_ptr: usize = 0;
        let mut classfile = ClassfileFragment::default();

        let result = steps.iter().fold(None, |acc, xfn| {
            match acc {
                None => {
                    let current_slice = &bytes[current_ptr..];

                    match xfn(current_slice) {
                        Ok((fragment, offset)) => {
                            classfile.update(fragment);
                            current_ptr += offset;
                            None
                        },
                        Err(error) => Some(error)
                    }
                },
                e@_ => e
            }
        });

        match result {
            None => Ok(classfile.to_classfile()),
            Some(error) => Err(error)
        }
    }

    pub fn read_magic(bytes: &[u8]) -> Result<(ClassfileFragment, usize), String> {
        let expected_magic = [ 0xCA, 0xFE, 0xBA, 0xBE ];

        if bytes.len() < expected_magic.len() {
            Err("Invalid class file magic".to_string())
        } else if &bytes[0..4] == expected_magic {
            Ok((ClassfileFragment::default(), 4))
        } else {
            Err("Invalid class file magic".to_string())
        }
    }

    pub fn read_version_number(bytes: &[u8]) -> Result<(ClassfileFragment, usize), String> {
        // size of two u16s
        if bytes.len() < 4 {
            Err(format!("Not enough version bytes: {}", bytes.len()))
        } else {
            let mut fragment = ClassfileFragment::default();

            fragment.minor_version = Some((bytes[0] as u16).shl(8) + bytes[1] as u16);
            fragment.major_version = Some((bytes[2] as u16).shl(8) + bytes[3] as u16);

            Ok((fragment, 4))
        }
    }

    pub fn read_constant_pool(bytes: &[u8]) -> Result<(ClassfileFragment, usize), String> {
        // size of an u16
        if bytes.len() < 2 {
            Err(format!("Not enough bytes available: {}", bytes.len()))
        } else {
            let cp_size = bytes.read_u16() - 1;

            let mut cf = ClassfileFragment::default();
            let mut cp: Vec<ConstantType> = vec![ ConstantType::Placeholder ];
            let mut byte_counter: usize = 2;

            match (0..cp_size).fold(None, |acc, _| {
                match acc {
                    None => {
                        match ClassfileReader::read_constant_pool_info(&bytes[byte_counter as usize..]) {
                            Ok((constant, size)) => {
                                byte_counter += size;
                                cp.push(constant);
                                None
                            },
                            Err(err) => Some(err)
                        }
                    },
                    err@_ => err
                }
            }) {
                None => {
                    cf.constant_pool = Some(cp);
                    Ok((cf, byte_counter))
                },
                Some(err) => Err(err)
            }
        }
    }

    pub fn read_constant_pool_info(bytes: &[u8]) -> Result<(ConstantType, usize), String> {
        // There's no constant type that takes less than 3 bytes
        let minimum_required_bytes = 3;

        if bytes.len() < minimum_required_bytes {
            Err(format!("Less then required number of bytes available: {}", bytes.len()).to_string())
        } else {
            let tag = bytes[0];

            match tag {
                1 => {
                    let utf8_len: u16 = bytes[1..].read_u16();
                    let utf8_bytes: Vec<u8> = bytes[3..utf8_len as usize].iter().map(|b| *b).collect();
                    Ok((ConstantType::Utf8 { length: utf8_len, bytes: utf8_bytes }, 3 + utf8_len as usize))
                },
                3 => Ok((ConstantType::Integer              { bytes: bytes[1..].read_u32() }, 5)),
                4 => Ok((ConstantType::Float                { bytes: bytes[1..].read_u32() }, 5)),
                5 => Ok((ConstantType::Long                 { high_bytes: bytes[1..].read_u32(), low_bytes: bytes[5..].read_u32() }, 9)),
                6 => Ok((ConstantType::Double               { high_bytes: bytes[1..].read_u32(), low_bytes: bytes[5..].read_u32() }, 9)),
                7 => Ok((ConstantType::Class                { name_index: bytes[1..].read_u16() }, 3)),
                8 => Ok((ConstantType::String               { string_index: bytes[1..].read_u16() }, 3)),
                9 => Ok((ConstantType::FieldRef             { class_index: bytes[1..].read_u16(), name_and_type_index: bytes[5..].read_u16() }, 5)),
                10 => Ok((ConstantType::MethodRef           { class_index: bytes[1..].read_u16(), name_and_type_index: bytes[5..].read_u16() }, 5)),
                11 => Ok((ConstantType::InterfaceMethodRef  { class_index: bytes[1..].read_u16(), name_and_type_index: bytes[5..].read_u16() }, 5)),
                12 => Ok((ConstantType::NameAndType         { name_index: bytes[1..].read_u16(), descriptor_index: bytes[5..].read_u16() }, 5)),
                15 => Ok((ConstantType::MethodHandle        { reference_kind: bytes[1], reference_index: bytes[2..].read_u16() }, 4)),
                16 => Ok((ConstantType::MethodType          { descriptor_index: bytes[1..].read_u16() }, 3)),
                18 => Ok((ConstantType::InvokeDynamic       { bootstrap_method_attr_index: bytes[1..].read_u16(), name_and_type_index: bytes[3..].read_u16() }, 5)),
                t@_ => Err(format!("Unrecognised constant pool tag: {} (sequence: {:02x} {:02x} {:02x})", t, bytes[1], bytes[2], bytes[3]).to_string())
            }
        }
    }
}

trait ReadChunks {
    fn read_u8(&self) -> u8;
    fn read_u16(&self) -> u16;
    fn read_u32(&self) -> u32;
}

impl ReadChunks for [u8] {
    fn read_u8(&self) -> u8 { match self.len() { 0 => 0, _ => self[0] } }
    fn read_u16(&self) -> u16 {
        match self.len() {
            0 => 0,
            1 => self[0] as u16,
            _ => (self[0] as u16).shl(8) + self[1] as u16
        }
    }

    fn read_u32(&self) -> u32 {
        match self.len() {
            0 => 0,
            1 => self[0] as u32,
            2 => (self[0] as u32).shl(8) + self[1] as u32,
            3 => (self[0] as u32).shl(16) + (self[1] as u32).shl(8) + self[2] as u32,
            _ => (self[0] as u32).shl(24) + (self[1] as u32).shl(16) + (self[2] as u32).shl(8) + self[3] as u32
        }
    }

}

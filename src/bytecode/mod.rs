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

#[allow(dead_code)]
pub struct ConstantPoolInfo {
    pub tag: ConstantType
}

pub enum MaybeMaybe<T> {
    Undefined,
    Nothing,
    Just(T)
}

pub struct ClassfileFragment {
    pub major_version: MaybeMaybe<u16>,
    pub minor_version: MaybeMaybe<u16>,
    pub constant_pool: MaybeMaybe<Vec<ConstantPoolInfo>>
}

#[derive(Default)]
pub struct RawBytecode {
    pub major_version: u16,
    pub minor_version: u16,
    pub constant_pool: Vec<ConstantPoolInfo>
}

impl RawBytecode {

    pub fn from_raw_bytes(raw_bytes: *const c_uchar, data_length: i32) -> Result<RawBytecode, String> {
        let mut buf: Vec<u8> = vec![];
        let mut bc = RawBytecode::default();

        unsafe {
            for i in 0..data_length {
                buf.push(*(raw_bytes.offset(i as isize)));
            }
        }

        let bytes = buf.as_slice();

        let methods: Vec<fn(&[u8], &mut RawBytecode) -> Result<usize, String>> = vec![
            RawBytecode::read_magic_numbers,
            RawBytecode::read_version_number,
            RawBytecode::read_constant_pool
        ];

        let mut current_ptr: usize = 0;

        let result = methods.iter().fold(None, |acc, xfn| {
            match acc {
                None => {
                    let current_slice = &bytes[current_ptr..];

                    match xfn(current_slice, &mut bc) {
                        Ok(offset) => {
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
            None => Ok(bc),
            Some(error) => Err(error)
        }
    }

    #[allow(unused_variables)]
    fn read_magic_numbers(bytes: &[u8], bytecode: &mut RawBytecode) -> Result<usize, String> {
        // size of four u8s
        if bytes.len() < 4 {
            Err("Oi, there aren't enough magic bytes".to_string())
        } else if &bytes[0..4] == [ 0xCA, 0xFE, 0xBA, 0xBE ] {
            Ok(4 as usize)
        } else {
            Err("Lofasz".to_string())
        }
    }

    fn read_version_number(bytes: &[u8], bytecode: &mut RawBytecode) -> Result<usize, String> {
        // size of two u16s
        if bytes.len() < 4 {
            Err("We still haven't got enough bytes".to_string())
        } else {
            bytecode.minor_version = (bytes[0] as u16).shl(8) + bytes[1] as u16;
            bytecode.major_version = (bytes[2] as u16).shl(8) + bytes[3] as u16;
            Ok(4 as usize)
        }
    }

    pub fn read_constant_pool(bytes: &[u8], bytecode: &mut RawBytecode) -> Result<usize, String> {
        // size of an u16
        if bytes.len() < 2 {
            Err("Shit".to_string())
        } else {
            let cp_size = bytes.read_u16();

            if cp_size > 0 {
                (0..cp_size).map(|i| {
                    RawBytecode::read_constant_pool_info(&bytes[(2 + (i * 2)) as usize..])
                }).fold(Ok(0 as usize), |acc, current| {
                    match acc {
                        Err(err) => Err(err),
                        Ok(size) => match current {
                            Ok(r) => {
                                bytecode.constant_pool.push(r.0);
                                Ok(size + r.1)
                            },
                            Err(err) => Err(err)
                        }
                    }
                })
            } else {
                Err(format!("Invalid constant pool length: {}", cp_size).to_string())
            }
        }
    }

    pub fn read_constant_pool_info(bytes: &[u8]) -> Result<(ConstantPoolInfo, usize), String> {
        // There's no constant type that takes less than 3 bytes
        let minimum_required_bytes = 3;

        if bytes.len() < minimum_required_bytes {
            Err(format!("Less then required number of bytes available: {}", bytes.len()).to_string())
        } else {
            let tag = bytes[0];

            match tag {
                3 => Ok((ConstantPoolInfo { tag: ConstantType::Integer { bytes: bytes[1..].read_u32() }}, 5)),
                4 => Ok((ConstantPoolInfo { tag: ConstantType::Float { bytes: bytes[1..].read_u32() }}, 5)),
                5 => Ok((ConstantPoolInfo { tag: ConstantType::Long { high_bytes: bytes[1..].read_u32(), low_bytes: bytes[5..].read_u32() }}, 9)),
                6 => Ok((ConstantPoolInfo { tag: ConstantType::Double { high_bytes: bytes[1..].read_u32(), low_bytes: bytes[5..].read_u32() }}, 9)),
                7 => Ok((ConstantPoolInfo { tag: ConstantType::Class { name_index: bytes[1..].read_u16() }}, 3)),
                8 => Ok((ConstantPoolInfo { tag: ConstantType::String { string_index: bytes[1..].read_u16() }}, 3)),
                9 => Ok((ConstantPoolInfo { tag: ConstantType::FieldRef { class_index: bytes[1..].read_u16(), name_and_type_index: bytes[5..].read_u16() }}, 5)),
                10 => Ok((ConstantPoolInfo { tag: ConstantType::MethodRef { class_index: bytes[1..].read_u16(), name_and_type_index: bytes[5..].read_u16() }}, 5)),
                11 => Ok((ConstantPoolInfo { tag: ConstantType::InterfaceMethodRef { class_index: bytes[1..].read_u16(), name_and_type_index: bytes[5..].read_u16() }}, 5)),
                12 => Ok((ConstantPoolInfo { tag: ConstantType::NameAndType { name_index: bytes[1..].read_u16(), descriptor_index: bytes[5..].read_u16() }}, 5)),
                15 => Ok((ConstantPoolInfo { tag: ConstantType::MethodHandle { reference_kind: bytes[1], reference_index: bytes[2..].read_u16() }}, 4)),
                16 => Ok((ConstantPoolInfo { tag: ConstantType::MethodType { descriptor_index: bytes[1..].read_u16() }}, 3)),
                18 => Ok((ConstantPoolInfo { tag: ConstantType::InvokeDynamic { bootstrap_method_attr_index: bytes[1..].read_u16(), name_and_type_index: bytes[3..].read_u16() }}, 5)),
                t@_ => Err(format!("Unrecognised constant pool tag: {}", t).to_string())
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

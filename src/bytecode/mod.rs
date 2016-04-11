//use super::native::*;
use libc::c_uchar;
use std::ops::Shl;

pub mod classfile;

enum ConstantType {
    Class = 7,
    FieldRef = 9,
    MethodRef = 10,
    InterfaceMethodRef = 11,
    String = 8,
    Integer = 3,
    Float = 4,
    Long = 5,
    Double = 6,
    NameAndType = 12,
    Utf8 = 1,
    MethodHandle = 15,
    MethodType = 16,
    InvokeDynamic = 18,
    Unknown = -1
}

pub struct ConstantPoolInfo {
    tag: ConstantType
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

        let applied: Vec<bool> = methods.iter().map(|xfn|{
            let current_slice = &bytes[current_ptr..];

            match xfn(current_slice, &mut bc) {
                Ok(offset) => {
                    current_ptr += offset;
                    true
                },
                Err(_) => false
            }
        }).take_while(|r| *r).collect();

        if applied.len() == methods.len() {
            Ok(bc)
        } else {
            Err("ERROR".to_string())
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

    fn read_constant_pool(bytes: &[u8], bytecode: &mut RawBytecode) -> Result<usize, String> {
        // size of an u16
        if bytes.len() < 2 {
            Err("Shit".to_string())
        } else {
            let cp_size = bytes.read_u16();

            (0..cp_size).map(|i| {
                match RawBytecode::read_constant_pool_info(&bytes[(i * 2) as usize..]) {
                    Some(cpi) => bytecode.constant_pool.push(cpi),
                    None => ()
                }
            });

            Ok(0 as usize)
        }
    }

    fn read_constant_pool_info(bytes: &[u8]) -> Option<ConstantPoolInfo> {
        if bytes.len() < 1 {
            None
        } else {
            None
        }
    }
}

pub trait ReadChunks {
    fn read_u8(&self) -> u8;
    fn read_u16(&self) -> u16;
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
}

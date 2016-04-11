//use super::native::*;
use libc::c_uchar;
use std::ops::Shl;

pub mod classfile;

#[derive(Default)]
pub struct RawBytecode {
    pub major_version: u16,
    pub minor_version: u16
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
            RawBytecode::read_version_number
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
        if bytes.len() < 4 {
            Err("Oi, there aren't enough magic bytes".to_string())
        } else if &bytes[0..4] == [ 0xCA, 0xFE, 0xBA, 0xBE ] {
            Ok(4 as usize)
        } else {
            Err("Lofasz".to_string())
        }
    }

    fn read_version_number(bytes: &[u8], bytecode: &mut RawBytecode) -> Result<usize, String> {
        if bytes.len() < 4 {
            Err("We still haven't got enough bytes".to_string())
        } else {
            bytecode.minor_version = (bytes[0] as u16).shl(8) + bytes[1] as u16;
            bytecode.major_version = (bytes[2] as u16).shl(8) + bytes[3] as u16;
            Ok(4 as usize)
        }
    }
}

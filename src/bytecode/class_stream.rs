use std::mem::size_of;
use super::constants::*;
use super::classfile::*;

///
/// A class stream takes a vector of bytes (`u8`) and reads Java class file fragments from it.
pub struct ClassStream<'a> {
    idx: usize,
    bytes: &'a Vec<u8>
}

impl<'a> ClassStream<'a> {

    /// Create a new class stream operating on the given vector of bytes
    pub fn new(bytes: &'a Vec<u8>) -> ClassStream {
        ClassStream { idx: 0, bytes: bytes }
    }

    /// Reset the stream index back to the beginning of the stream
    #[allow(dead_code)]
    pub fn rewind(&mut self) -> () {
        self.idx = 0;
    }

    /// Verify if the stream pointer is standing at a valid class file magic constant CAFEBABE indicating
    /// the beginning of a class file
    pub fn read_magic_bytes(&mut self) -> bool {
        let opt_magic = self.read_u32();

        match opt_magic {
            Some(0xCAFEBABE) => true,
            _ => false
        }
    }

    /// Return a Java class file version number
    pub fn read_version_number(&mut self) -> Option<(u16, u16)> {
        let opt_version = (self.read_u16(), self.read_u16());

        match opt_version {
            (Some(minor_version), Some(major_version)) => Some((minor_version, major_version)),
            _ => None
        }
    }

    pub fn read_constant_pool(&mut self) -> Option<Vec<ConstantType>> {
        let pool_count = self.read_u16();

        // TODO the following block needs some refactoring as it's quite ugly in it's current form
        match pool_count {
            Some(count) => {
                let upper_bound: usize = if count > 1 { count as usize } else { 1 };

                let read_size: usize = 0;

                let r: (usize, Option<Vec<ConstantType>>) = (1..upper_bound).fold((read_size, Some(vec![])), |acc, _| {
                    match acc {
                        (_, None) => (0, None),
                        (c, Some(mut v)) => {
                            if c < upper_bound - 1 {
                                match ConstantType::parse(self) {
                                    Some(constant) => {
                                        let offset: usize = if constant.is_long_entry() { 2 } else { 1 };

                                        v.push(constant);
                                        (c + offset, Some(v))
                                    },
                                    None => (0, None)
                                }
                            } else {
                                (c, Some(v))
                            }
                        }
                    }
                });

                match r {
                    (_, None) => None,
                    (_, r@_) => r
                }
            },
            None => None
        }
    }

    pub fn read_class_access_flags(&mut self) -> Option<AccessFlag> {
        match self.read_u16() {
            Some(flag) => Some(AccessFlag::of(flag)),
            _ => None
        }
    }

    pub fn read_constant_reference(&mut self) -> Option<ConstantReference> {
        match self.read_u16() {
            Some(reference) => Some(ConstantReference::new(reference)),
            _ => None
        }
    }

    pub fn read_fields(&mut self) -> Option<Vec<Field>> {
        let opt_count = self.read_u16();

        match opt_count {
            Some(count) => {
                (0..count).map(|_| { self.read_field() }).fold(Some(vec![]), |acc, x| acc.map_or(None, |mut v| {
                    x.map_or(None, |d| { v.push(d); Some(v) })
                }))
            },
            _ => None
        }
    }

    pub fn read_field(&mut self) -> Option<Field> {
        // at least 8 bytes are required to parse a field
        if self.bytes.len() >= 8 {
            let raw_flag = self.get_u16();
            let raw_name = self.get_u16();
            let raw_desc = self.get_u16();
            let att_count = self.get_u16() as usize;

            self.read_map_len(att_count, |bs| {
                Field {
                    access_flags: AccessFlag::of(raw_flag),
                    name_index: ConstantReference::new(raw_name),
                    descriptor_index: ConstantReference::new(raw_desc),
                    attributes: vec![]
                }
            })
        } else {
            None
        }
    }

    pub fn read_methods(&mut self) -> Option<Vec<Method>> {
        //let count = self.read_u16();

        None
    }

    pub fn read_method(&mut self) -> Option<Method> {
        None
    }

    pub fn read_attributes(&mut self) -> Option<Vec<Attribute>> {
        //let count = self.read_u16();

        None
    }

    pub fn read_attribute(&mut self) -> Option<Attribute> {
        None
    }
}

///
/// Allows reading sized unsigned integers from arbitrary inputs (most typically from arrays or
/// vectors of u8). The trait also has a method `read_n()` that lets reading a fixed length byte
/// sequence from the underlying source
///
pub trait ReadChunks {
    fn read_u32(&mut self) -> Option<u32>;
    fn read_u16(&mut self) -> Option<u16>;
    fn read_u8(&mut self) -> Option<u8>;
    fn read_n(&mut self, len: usize) -> Option<Vec<u8>>;

    fn get_u32(&mut self) -> u32;
    fn get_u16(&mut self) -> u16;
    fn get_u8(&mut self) -> u8;
}

impl<'a> ReadChunks for ClassStream<'a> {


    fn read_u32(&mut self) -> Option<u32> {
        if self.idx + size_of::<u32>() <= self.bytes.len() {
            let r = Some(self.bytes[self.idx] as u32 * 0x1000000
                + self.bytes[self.idx + 1] as u32 * 0x10000
                + self.bytes[self.idx + 2] as u32 * 0x100
                + self.bytes[self.idx + 3] as u32);
            self.idx += 4;
            r
        } else {
            None
        }
    }

    fn read_u16(&mut self) -> Option<u16> {
        if self.idx + size_of::<u16>() <= self.bytes.len() {
            let r = Some((self.bytes[self.idx] as u16 * 0x100
                + self.bytes[self.idx + 1] as u16));
            self.idx += 2;
            r
        } else {
            None
        }
    }

    fn read_u8(&mut self) -> Option<u8> {
        if self.idx + size_of::<u8>() <= self.bytes.len() {
            let r = Some(self.bytes[self.idx]);
            self.idx += 1;
            r
        } else {
            None
        }
    }

    fn read_n(&mut self, len: usize) -> Option<Vec<u8>> {
        let upper_bound = (self.idx + len) as usize;

        if upper_bound <= self.bytes.len() {
            let r = Some(self.bytes[self.idx..upper_bound].to_vec());
            self.idx += len;
            r
        } else {
            None
        }
    }

    fn get_u32(&mut self) -> u32 {
        self.read_u32().unwrap_or(0)
    }

    fn get_u16(&mut self) -> u16 {
        self.read_u16().unwrap_or(0)//.unwrap_or(self.get_u8() as u16)
    }

    fn get_u8(&mut self) -> u8 {
        self.read_u8().unwrap_or(0)
    }
}

pub trait ReadMapper {
    fn read_map<T, U>(&self, t: T) -> U where T: Fn(&Self) -> U {
        t(self)
    }

    fn read_map_if<T, U, V>(&self, fc: V, t: T) -> Option<U> where V: Fn(&Self) -> bool, T: Fn(&Self) -> U {
        match fc(self) {
            true => Some(t(self)),
            false => None
        }
    }

    fn read_map_len<T, U>(&mut self, size: usize, t: T) -> Option<U> where T: Fn(&mut Self) -> U;
}

impl<'a> ReadMapper for ClassStream<'a> {
    fn read_map_len<T, U>(&mut self, size: usize, t: T) -> Option<U> where T: Fn(&mut Self) -> U {
        let upper_bound = (self.idx + size) as usize;

        if upper_bound <= self.bytes.len() {
            Some(t(self))
        } else {
            None
        }
    }
}

use std::mem::size_of;
use super::constants::ConstantType;

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
            (Some(major_version), Some(minor_version)) => Some((major_version, minor_version)),
            _ => None
        }
    }

    pub fn read_constant_pool(&mut self) -> Option<Vec<ConstantType>> {
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
}

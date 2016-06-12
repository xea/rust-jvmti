use std::cell::Cell;
use super::classfile::*;
use super::constant::*;

pub enum ClassInputStreamError {
    InvalidMagic(u32),
    InvalidConstantTag(u8),
    PrematureEnd,
    NotImplemented
}

impl ClassInputStreamError {
    /// Convert the current error object into a human-readable string explaining the error
    pub fn to_string(&self) -> String {
        match *self {
            ClassInputStreamError::InvalidMagic(bad_magic) => format!("Invalid magic bytes: {:x}", bad_magic),
            _ => format!("")
        }
    }
}

pub struct ClassInputStream<'a> {
    idx: Cell<usize>,
    marker: Cell<Option<usize>>,
    bytes: &'a Vec<u8>
}

pub struct ClassOutputStream {
    bytes: Vec<u8>
}

impl<'a> ClassInputStream<'a> {

    pub fn from_vec(vec: &'a Vec<u8>) -> ClassInputStream {
        ClassInputStream { idx: Cell::new(0), marker: Cell::new(None), bytes: vec }
    }

    pub fn read_magic_bytes(&self) -> Result<(), ClassInputStreamError> {
        match self.read_u32() {
            Some(0xCAFEBABE) => Ok(()),
            Some(m) => Err(ClassInputStreamError::InvalidMagic(m)),
            None => Err(ClassInputStreamError::PrematureEnd),
        }
    }

    pub fn read_version_number(&self) -> Result<ClassfileVersion, ClassInputStreamError> {
        ClassfileVersion::read_element(self)
    }

    pub fn read_constant_pool(&self) -> Result<ConstantPool, ClassInputStreamError> {
        match self.read_u16() {
            Some(cp_len) => {
                let fold_start: (usize, Result<Vec<Constant>, ClassInputStreamError>) = (0, Ok(vec![]));

                // TODO this needs some refactoring because it's rather overcomplicated but at least it seems working
                let result = (1..cp_len).map(|_| self.read_constant()).map(|res_const| {
                    match res_const {
                        Ok(cnst) => Ok((if cnst.is_long_entry() { 2 } else { 1 }, cnst)),
                        Err(err) => Err(err)
                    }
                }).fold(fold_start, |(n, acc), c| {
                    match (acc, c) {
                        (Ok(mut tmp_cp), Ok((len, cnst))) => {
                            if n + len <= cp_len as usize {
                                tmp_cp.push(cnst);
                                (n + len, Ok(tmp_cp))
                            } else {
                                (n, Ok(tmp_cp))
                            }
                        },
                        (e@Err(_), _) => (n, e),
                        (_, Err(e)) => (n, Err(e))
                    }
                });

                match result {
                    (_, Ok(r)) => Ok(ConstantPool::from_vec(r)),
                    (_, Err(r)) => Err(r)
                }
            },
            _ => Err(ClassInputStreamError::PrematureEnd)
        }
    }

    fn read_constant(&self) -> Result<Constant, ClassInputStreamError> {
        Err(ClassInputStreamError::NotImplemented)
    }

    ///
    /// Mark the current position in the stream so subsequent `reset()` calls can return to this
    /// position. This can be used to define "safe points" in the stream.
    pub fn mark(&self) {
        self.marker.set(Some(self.idx.get()));
    }

    ///
    /// Move the stream index to the last marked posiiton if there was one. If the stream hadn't
    /// been marked then
    pub fn reset(&self) {
        match self.marker.get() {
            Some(idx) => self.idx.set(idx),
            None => self.idx.set(0)
        }
    }

    /// Return the number of available bytes in the stream
    pub fn available(&self) -> usize {
        (self.bytes.len() - self.idx.get()) as usize
    }

    ///
    /// Attempt to read `count` number of bytes from the underlying bytes, interpreting them as
    /// a single integral number and return this value or failing that, return None
    pub fn read_bytes(&self, count: usize) -> Option<u64> {
        let cur_idx: usize = self.idx.get();

        if count <= self.available() {
            let val = self.peek_bytes(count);
            self.idx.set(cur_idx + count);
            Some(val)
        } else {
            None
        }
    }

    ///
    /// Try to read `count`number of bytes from the backing array without moving the stream index.
    /// and return the read bytes interpreted as a single integral value
    pub fn peek_bytes(&self, count: usize) -> u64 {
        let mut val: u64 = 0;
        let cur_idx: usize = self.idx.get();

        if count <= self.available() {
            for i in 0..count {
                let current_byte: u8 = self.bytes[cur_idx + i];
                val <<= 8;
                val += current_byte as u64;
            }
        }

        val
    }

    ///
    /// Attempts to read `count` number of bytes from the underlying bytes, interpreting them as
    /// a single integral number and return this number or failing that, return 0
    fn get_bytes(&self, count: usize) -> u64 {
        self.read_bytes(count).unwrap_or(0)
    }
}

pub trait ClassStreamEntry: Sized {
    fn read_element(stream: &ClassInputStream) -> Result<Self, ClassInputStreamError>;
    fn write_element(&self, stream: &mut ClassOutputStream);
}

impl ClassOutputStream {
    pub fn new() -> ClassOutputStream {
        ClassOutputStream {
            bytes: vec![]
        }
    }

    pub fn write_magic_bytes(&mut self) -> () {
        self.write_u32(0xCAFEBABE);
    }

    pub fn write_version_number(&mut self, version: &ClassfileVersion) -> () {
        self.write_u16(version.minor_version);
        self.write_u16(version.major_version);
    }

    pub fn write_bytes(&mut self, bytes: &Vec<u8>) -> () {
        for byte in bytes.iter() {
            self.bytes.push(*byte);
        }
    }

    pub fn to_vec(self) -> Vec<u8> {
        self.bytes
    }
}


pub trait ReadChunks {
    fn read_u64(&self) -> Option<u64>;
    fn read_u32(&self) -> Option<u32>;
    fn read_u16(&self) -> Option<u16>;
    fn read_u8(&self) -> Option<u8>;

    fn get_u64(&self) -> u64;
    fn get_u32(&self) -> u32;
    fn get_u16(&self) -> u16;
    fn get_u8(&self) -> u8;
}

pub trait WriteChunks {
    fn write_u64(&mut self, value: u64) -> ();
    fn write_u32(&mut self, value: u32) -> ();
    fn write_u16(&mut self, value: u16) -> ();
    fn write_u8(&mut self, value: u8) -> ();
}

impl<'a> ReadChunks for ClassInputStream<'a> {
    fn read_u64(&self) -> Option<u64> {
        self.read_bytes(8)
    }

    fn read_u32(&self) -> Option<u32> {
        self.read_bytes(4).map(|v| v as u32)
    }

    fn read_u16(&self) -> Option<u16> {
        self.read_bytes(2).map(|v| v as u16)
    }

    fn read_u8(&self) -> Option<u8> {
        self.read_bytes(1).map(|v| v as u8)
    }

    fn get_u64(&self) -> u64 {
        self.get_bytes(8)
    }

    fn get_u32(&self) -> u32 {
        self.get_bytes(4) as u32
    }

    fn get_u16(&self) -> u16 {
        self.get_bytes(2) as u16
    }

    fn get_u8(&self) -> u8 {
        self.get_bytes(1) as u8
    }
}

impl<'a> WriteChunks for ClassOutputStream {
    fn write_u64(&mut self, value: u64) -> () {
        self.write_bytes(&vec![
            ((value & 0xFF << 56) >> 56) as u8,
            ((value & 0xFF << 48) >> 48) as u8,
            ((value & 0xFF << 40) >> 40) as u8,
            ((value & 0xFF << 32) >> 32) as u8,
            ((value & 0xFF << 24) >> 24) as u8,
            ((value & 0xFF << 16) >> 16) as u8,
            ((value & 0xFF << 8) >> 8) as u8,
            (value & 0xFF) as u8
        ]);

    }

    fn write_u32(&mut self, value: u32) -> () {
        self.write_bytes(&vec![
            ((value & 0xFF << 24) >> 24) as u8,
            ((value & 0xFF << 16) >> 16) as u8,
            ((value & 0xFF << 8) >> 8) as u8,
            (value & 0xFF) as u8
        ]);
    }

    fn write_u16(&mut self, value: u16) -> () {
        self.write_bytes(&vec![ ((value & 0xFF00) >> 8) as u8, (value & 0xFF) as u8 ]);
    }

    fn write_u8(&mut self, value: u8) -> () {
        self.write_bytes(&vec![value]);
    }
}

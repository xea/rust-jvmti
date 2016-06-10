use std::cell::Cell;


pub struct ClassStream<'a> {
    idx: Cell<usize>,
    marker: Cell<Option<usize>>,
    bytes: &'a Vec<u8>
}

pub enum ClassStreamError {
    NotImplemented
}

pub struct ClassFragment {

}

pub trait ClassInputStream {
    fn read_magic_bytes(&self) -> Result<ClassFragment, ClassStreamError>;
}

impl<'a> ClassStream<'a> {

    pub fn from_vec(vec: &'a Vec<u8>) -> ClassStream {
        ClassStream { idx: Cell::new(0), marker: Cell::new(None), bytes: vec }
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

impl<'a> ReadChunks for ClassStream<'a> {
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

/*
impl ReadChunks for Vec<u8> {

    fn read_u64(&self) -> Option<u64> {
        None
    }

    fn read_u32(&self) -> Option<u32> {
        None
    }

    fn read_u16(&self) -> Option<u16> {
        None
    }

    fn read_u8(&self) -> Option<u8> {
        None
    }

    fn get_u64(&self) -> u64 {
        0
    }

    fn get_u32(&self) -> u32 {
        0
    }

    fn get_u16(&self) -> u16 {
        0
    }

    fn get_u8(&self) -> u8 {
        0
    }
}
*/


/*
pub trait BytecodeItem {
    fn from_bytes(bytes: &Vec<u8>) -> Self;
    fn to_bytes(&self) -> Vec<u8>;
}

impl BytecodeItem for Class {

    fn from_bytes(bytes: &Vec<u8>) -> Self {
        Class {
            version: ClassfileVersion::default()
        }
    }

    fn to_bytes(&self) -> Vec<u8> {
        let mut b: Vec<u8> = vec![];
        b.append(&mut self.version.to_bytes());

        b
    }
}


impl BytecodeItem for ClassfileVersion {

    fn from_bytes(bytes: &Vec<u8>) -> Self {
        const MIN_VERSION_LENGTH: usize = 4;

        if bytes.len() >= MIN_VERSION_LENGTH {
            ClassfileVersion { minor_version: 14, major_version: 52 }
        } else {
            ClassfileVersion::default()
        }
    }

    fn to_bytes(&self) -> Vec<u8> {
        vec![ self.minor_version, self.major_version ]
            .iter()
            .map(|x| vec![ (x & 0xff00 >> 8) as u8, (x & 0xff) as u8 ])
            .flat_map(|x| x).collect()
    }
}
*/

use std::mem::size_of;
use std::ops::Shl;

pub struct ClassfileFragment {
    pub major_version: Option<u16>,
    pub minor_version: Option<u16>,
    pub constant_pool: Option<Vec<ConstantType>>
}

pub struct Classfile {
    pub major_version: u16,
    pub minor_version: u16,
    pub constant_pool: Vec<ConstantType>
}

impl Classfile {
    pub fn new() -> Classfile {
        Classfile {
            major_version: 0x00,
            minor_version: 0x00,
            constant_pool: vec![]
        }
    }

    pub fn merge(mut self, fragment: ClassfileFragment) -> Self {
        self.major_version = fragment.major_version.unwrap_or(self.major_version);
        self.minor_version = fragment.minor_version.unwrap_or(self.minor_version);
        self.constant_pool = fragment.constant_pool.unwrap_or(self.constant_pool);
        self
    }
}

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

impl ConstantType {
    pub fn parse(constant_type: u8, bytes: &Vec<u8>) -> Option<ConstantType> {
        //let r: Option<Classfile> = bytes.read_map(|f| Some(Classfile::new()));
        //let r1: Option<Classfile> = bytes.read_map_if(|c| c.len() >= 3, |c| Classfile::new());
        //let r2: Option<Classfile> = bytes.read_map_len(3, |c| Classfile::new());
        match constant_type {
            // Utf8
            //1 => bytes.read_map_len(2, |bs| ConstantType::Utf8 { length: bs.read_u16(), bytes: bs[2..(2 + bs.read_u16() as u8)].to_vec() }),
            3 => bytes.read_map_len(4, |bs| ConstantType::Integer { bytes: bs.read_u32() }),
            4 => bytes.read_map_len(4, |bs| ConstantType::Float { bytes: bs.read_u32() }),
            5 => bytes.read_map_len(8, |bs| ConstantType::Long { high_bytes: bs.read_u32(), low_bytes: bs[4..].read_u32() }),
            6 => bytes.read_map_len(8, |bs| ConstantType::Double { high_bytes: bs.read_u32(), low_bytes: bs[4..].read_u32() }),
            7 => bytes.read_map_len(2, |bs| ConstantType::Class { name_index: bs.read_u16() }),
            8 => bytes.read_map_len(2, |bs| ConstantType::String { string_index: bs.read_u16() }),
            9 => bytes.read_map_len(4, |bs| ConstantType::FieldRef { class_index: bs.read_u16(), name_and_type_index: bs[2..].read_u16() }),
            10 => bytes.read_map_len(4, |bs| ConstantType::MethodRef { class_index: bs.read_u16(), name_and_type_index: bs[2..].read_u16() }),
            11 => bytes.read_map_len(4, |bs| ConstantType::InterfaceMethodRef { class_index: bs.read_u16(), name_and_type_index: bs[2..].read_u16() }),
            12 => bytes.read_map_len(4, |bs| ConstantType::NameAndType { name_index: bs.read_u16(), descriptor_index: bs[2..].read_u16() }),
            15 => bytes.read_map_len(3, |bs| ConstantType::MethodHandle { reference_kind: bs.read_u8(), reference_index: bs[1..].read_u16() }),
            16 => bytes.read_map_len(2, |bs| ConstantType::MethodType { descriptor_index: bs.read_u16() }),
            18 => bytes.read_map_len(4, |bs| ConstantType::InvokeDynamic { bootstrap_method_attr_index: bs.read_u16(), name_and_type_index: bs[2..].read_u16() }),
            1 => bytes.read_map_len(2, |bs| {
                let bytes_count = bs.read_u16();
                let upper_bound = (bytes_count + 2) as usize;

                match bs.len() >= upper_bound {
                    true => ConstantType::Utf8 { length: bytes_count, bytes: bs[2..upper_bound].to_vec() },
                    // TODO consider raising an error here
                    false => ConstantType::Unknown
                }
            }),
            _ => None
        }
    }


    pub fn length(&self) -> usize {
        (match *self {
            ConstantType::Integer { bytes: _ }  => 4,
            ConstantType::Float { bytes: _ }  => 4,
            ConstantType::Long { high_bytes: _, low_bytes: _ }  => 8,
            ConstantType::Double { high_bytes: _, low_bytes: _ }  => 8,
            ConstantType::Class { name_index: _ } => 2,
            ConstantType::String { string_index: _ } => 2,
            ConstantType::FieldRef { class_index: _, name_and_type_index: _ } => 4,
            ConstantType::MethodRef { class_index: _, name_and_type_index: _ } => 4,
            ConstantType::InterfaceMethodRef { class_index: _, name_and_type_index: _ } => 4,
            ConstantType::NameAndType { name_index: _, descriptor_index: _ } => 4,
            ConstantType::MethodHandle { reference_kind: _, reference_index: _ } => 3,
            ConstantType::MethodType { descriptor_index: _ } => 2,
            ConstantType::InvokeDynamic { bootstrap_method_attr_index: _, name_and_type_index: _ } => 4,
            ConstantType::Utf8 { length: count, bytes: _ } => count + 2,
            _ => 0
        }) as usize
    }

    pub fn is_unknown(&self) -> bool {
        match *self {
            ConstantType::Unknown => true,
            _ => false
        }
    }

    pub fn is_8byte(&self) -> bool {
        match *self {
            ConstantType::Long { high_bytes: _, low_bytes: _ }  => true,
            ConstantType::Double { high_bytes: _, low_bytes: _ }  => true,
            _ => false
        }
    }
}

/*

ClassFile {
    u4             magic;
    u2             minor_version;
    u2             major_version;
    u2             constant_pool_count;
    cp_info        constant_pool[constant_pool_count-1];
    u2             access_flags;
    u2             this_class;
    u2             super_class;
    u2             interfaces_count;
    u2             interfaces[interfaces_count];
    u2             fields_count;
    field_info     fields[fields_count];
    u2             methods_count;
    method_info    methods[methods_count];
    u2             attributes_count;
    attribute_info attributes[attributes_count];
}

*/

pub struct ClassReader<'a> {

    idx: usize,
    bytes: &'a Vec<u8>

}

impl<'a> ClassReader<'a> {

    pub fn new(bytes: &Vec<u8>) -> ClassReader {
        ClassReader { idx: 0, bytes: bytes }
    }

    pub fn from_bytes(bytes: &Vec<u8>) -> Classfile {
        let mut reader = ClassReader { idx: 0, bytes: bytes };

        let version = reader.read_version_number();
        let constant_pool = reader.read_constant_pool();

        Classfile::new()
    }

    pub fn read_magic_bytes(&mut self) -> bool {
        let magic_bytes = self.read_u32();

        match magic_bytes {
            Some(bytes) => bytes == 0xCAFEBABE,
            None => false
        }
    }

    pub fn read_version_number(&mut self) -> Option<(u16, u16)> {
        let major_version = self.read_u16();
        let minor_version = self.read_u16();

        match (major_version, minor_version) {
            (Some(major), Some(minor)) => Some((major, minor)),
            _ => None
        }
    }

    pub fn read_constant_pool(&mut self) -> Option<Vec<ConstantType>> {
        match self.read_u16() {
            Some(constant_pool_size) => {
                // This hack is needed because the JVM class file specification makes 8 bytes constants take up
                // two entries in the constant pool, instead of one.
                // Quote from the spec: "In retrospect, making 8-byte constants take two constant pool entries was a poor choice."
                let mut previous_8byte_constant = false;
                let range_max = if constant_pool_size > 1 { constant_pool_size } else { 1 };

                let constants: Vec<ConstantType> = (1..range_max as usize).map(|_| {
                    if previous_8byte_constant {
                        previous_8byte_constant = false;
                        return ConstantType::Placeholder;
                    }

                    match self.read_u8() {
                        Some(constant_type) => match ConstantType::parse(constant_type, self.bytes) {
                            Some(constant) => {
                                if constant.is_8byte() {
                                    previous_8byte_constant = true;
                                }

                                self.idx += constant.length();
                                constant
                            },
                            _ => ConstantType::Unknown
                        },
                        None => ConstantType::Unknown
                    }
                }).collect();

                match constants.iter().any(|c| c.is_unknown() ) {
                    true => None,
                    false => Some(constants)
                }
            },
            None => None
        }
    }

    pub fn read_u32(&mut self) -> Option<u32> {
        if self.idx + size_of::<u32>() <= self.bytes.len() {
            let r = Some(self.bytes[self.idx] as u32 * 0x1000000 + self.bytes[self.idx + 1] as u32 * 0x10000 + self.bytes[self.idx + 2] as u32 * 0x100 + self.bytes[self.idx + 3] as u32);
            self.idx += 4;
            r
        } else {
            None
        }
    }

    pub fn read_u16(&mut self) -> Option<u16> {
        if self.idx + size_of::<u16>() <= self.bytes.len() {
            let r = Some((self.bytes[self.idx] as u16 * 0x100 + self.bytes[self.idx + 1] as u16));
            self.idx += 2;
            r
        } else {
            None
        }
    }

    pub fn read_u8(&mut self) -> Option<u8> {
        if self.idx + size_of::<u8>() <= self.bytes.len() {
            let r = Some(self.bytes[self.idx]);
            self.idx += 1;
            r
        } else {
            None
        }
    }
}

impl Classfile {

    pub fn to_bytes(&self) -> Vec<u8> {
        let mut bytes: Vec<u8> = vec![ 0xCA, 0xFE, 0xBA, 0xBE ];

        // This is happening under the assumption that version numbers will not exceed 255 in the near future
        bytes.push(self.major_version as u8);
        bytes.push(self.minor_version as u8);

        bytes
    }
}

trait ReadChunks {
    fn read_u8(&self) -> u8;
    fn read_u16(&self) -> u16;
    fn read_u32(&self) -> u32;
}

impl ReadChunks for Vec<u8> {
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

trait ReadMapper {
    fn read_map<T, U>(&self, t: T) -> U where T: Fn(&Self) -> U {
        t(self)
    }

    fn read_map_if<T, U, V>(&self, fc: V, t: T) -> Option<U> where V: Fn(&Self) -> bool, T: Fn(&Self) -> U {
        match fc(self) {
            true => Some(t(self)),
            false => None
        }
    }

    fn read_map_len<T, U>(&self, size: usize, t: T) -> Option<U> where T: Fn(&Self) -> U;
}

impl ReadMapper for [u8] {
    fn read_map_len<T, U>(&self, size: usize, t: T) -> Option<U> where T: Fn(&Self) -> U {
        match self.len() >= size {
            true => Some(t(self)),
            false => None
        }
    }
}

impl ReadMapper for Vec<u8> {
    fn read_map_len<T, U>(&self, size: usize, t: T) -> Option<U> where T: Fn(&Self) -> U {
        match self.len() >= size {
            true => Some(t(self)),
            false => None
        }
    }
}

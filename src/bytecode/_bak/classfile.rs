use super::constants::*;

pub struct ConstantPoolIndex {
    pub id: usize
}

pub struct ClassAccessFlags {
    flag: u16
}

pub struct FieldAccessFlags {
    flag: u16
}

pub struct MethodAccessFlags {
    flag: u16
}

impl ClassAccessFlags {
    pub fn is_public(&self) -> bool { self.has_flag(ClassAccessFlagType::Public) }
    pub fn is_final(&self) -> bool { self.has_flag(ClassAccessFlagType::Final) }
    pub fn is_super(&self) -> bool { self.has_flag(ClassAccessFlagType::Super) }
    pub fn is_interface(&self) -> bool { self.has_flag(ClassAccessFlagType::Interface) }
    pub fn is_abstract(&self) -> bool { self.has_flag(ClassAccessFlagType::Abstract) }
    pub fn is_synthetic(&self) -> bool { self.has_flag(ClassAccessFlagType::Synthetic) }
    pub fn is_annotation(&self) -> bool { self.has_flag(ClassAccessFlagType::Annotation) }
    pub fn is_enum(&self) -> bool { self.has_flag(ClassAccessFlagType::Enum) }
    pub fn set(&mut self, flag: ClassAccessFlagType) { self.flag = self.flag | flag as u16 }
    pub fn clear(&mut self, flag: ClassAccessFlagType) { self.flag = self.flag & (0xFFFF ^ flag as u16) }
    pub fn has_flag(&self, flag: ClassAccessFlagType) -> bool { self.flag & flag as u16 > 0 }
    pub fn raw_flag(&self) -> u16 { self.flag }
    pub fn new(initial_flags: u16) -> ClassAccessFlags { ClassAccessFlags { flag: initial_flags } }
}

impl FieldAccessFlags {
    pub fn is_public(&self) -> bool { self.has_flag(FieldAccessFlagType::Public) }
    pub fn is_private(&self) -> bool { self.has_flag(FieldAccessFlagType::Private) }
    pub fn is_protected(&self) -> bool { self.has_flag(FieldAccessFlagType::Protected) }
    pub fn is_static(&self) -> bool { self.has_flag(FieldAccessFlagType::Static) }
    pub fn is_final(&self) -> bool { self.has_flag(FieldAccessFlagType::Final) }
    pub fn is_volatile(&self) -> bool { self.has_flag(FieldAccessFlagType::Volatile) }
    pub fn is_transient(&self) -> bool { self.has_flag(FieldAccessFlagType::Transient) }
    pub fn is_synthetic(&self) -> bool { self.has_flag(FieldAccessFlagType::Synthetic) }
    pub fn is_enum(&self) -> bool { self.has_flag(FieldAccessFlagType::Enum) }
    pub fn set(&mut self, flag: FieldAccessFlagType) { self.flag = self.flag | flag as u16 }
    pub fn clear(&mut self, flag: FieldAccessFlagType) { self.flag = self.flag & (0xFFFF ^ flag as u16) }
    pub fn has_flag(&self, flag: FieldAccessFlagType) -> bool { self.flag & flag as u16 > 0 }
    pub fn raw_flag(&self) -> u16 { self.flag }
    pub fn new(initial_flags: u16) -> ClassAccessFlags { ClassAccessFlags { flag: initial_flags } }
}

impl MethodAccessFlags {
    pub fn is_public(&self) -> bool { self.has_flag(MethodAccessFlagType::Public) }
    pub fn is_private(&self) -> bool { self.has_flag(MethodAccessFlagType::Private) }
    pub fn is_protected(&self) -> bool { self.has_flag(MethodAccessFlagType::Protected) }
    pub fn is_static(&self) -> bool { self.has_flag(MethodAccessFlagType::Static) }
    pub fn is_final(&self) -> bool { self.has_flag(MethodAccessFlagType::Final) }
    pub fn is_synchronized(&self) -> bool { self.has_flag(MethodAccessFlagType::Synchronized) }
    pub fn is_bridge(&self) -> bool { self.has_flag(MethodAccessFlagType::Bridge) }
    pub fn is_varargs(&self) -> bool { self.has_flag(MethodAccessFlagType::Varargs) }
    pub fn is_native(&self) -> bool { self.has_flag(MethodAccessFlagType::Native) }
    pub fn is_abstract(&self) -> bool { self.has_flag(MethodAccessFlagType::Abstract) }
    pub fn is_strict(&self) -> bool { self.has_flag(MethodAccessFlagType::Strict) }
    pub fn is_synthetic(&self) -> bool { self.has_flag(MethodAccessFlagType::Synthetic) }
    pub fn set(&mut self, flag: MethodAccessFlagType) { self.flag = self.flag | flag as u16 }
    pub fn clear(&mut self, flag: MethodAccessFlagType) { self.flag = self.flag & (0xFFFF ^ flag as u16) }
    pub fn has_flag(&self, flag: MethodAccessFlagType) -> bool { self.flag & flag as u16 > 0 }
    pub fn raw_flag(&self) -> u16 { self.flag }
    pub fn new(initial_flags: u16) -> ClassAccessFlags { ClassAccessFlags { flag: initial_flags } }
}

pub struct MethodInfo {
    pub access_flags: MethodAccessFlags,
    pub name_index: ConstantPoolIndex,
    pub descriptor_index: ConstantPoolIndex,
    pub attributes: Vec<AttributeInfo>
}

pub struct FieldInfo {
    pub access_flags: FieldAccessFlags,
    pub name_index: ConstantPoolIndex,
    pub descriptor_index: ConstantPoolIndex,
    pub attributes: Vec<AttributeInfo>
}

pub struct AttributeInfo {
    pub attribute_name_index: ConstantPoolIndex,
    pub info: Vec<u8>
}

#[derive(Default)]
pub struct ClassfileFragment {
    pub major_version: Option<u16>,
    pub minor_version: Option<u16>,
    pub constant_pool: Option<Vec<ConstantType>>,
    pub access_flags: Option<ClassAccessFlags>,
    pub this_class: Option<ConstantPoolIndex>,
    pub super_class: Option<ConstantPoolIndex>,
    pub interfaces: Option<Vec<ConstantPoolIndex>>,
    pub fields: Option<Vec<FieldInfo>>,
    pub methods: Option<Vec<MethodInfo>>,
    pub attributes: Option<Vec<AttributeInfo>>
}

pub struct Classfile {
    pub major_version: u16,
    pub minor_version: u16,
    pub constant_pool: Vec<ConstantType>,
    pub access_flags: ClassAccessFlags,
    pub this_class: ConstantPoolIndex,
    pub super_class: ConstantPoolIndex,
    pub interfaces: Vec<ConstantPoolIndex>,
    pub fields: Vec<FieldInfo>,
    pub methods: Vec<MethodInfo>,
    pub attributes: Vec<AttributeInfo>
}

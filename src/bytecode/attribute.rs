use super::collections::ReadChunks;
use super::stream::ClassStreamItem;
use super::ConstantPoolIndex;

pub trait Attribute {
    fn len() -> usize;
}

/// represents the value of a constant expression
pub struct ConstantValueAttribute {
    pub constantvalue_index: ConstantPoolIndex
}

impl Attribute for ConstantValueAttribute {
    fn len() -> usize { 6 }
}

/// Contains the JVM instructions and auxilliary information for  method
pub struct CodeAttribute {
    pub max_stack: u16,
    pub max_locals: u16
}

impl Attribute for CodeAttribute {
    fn len() -> usize { 8 }
}

pub enum AttributeType {
    ConstantValue,
    Code
}

impl AttributeType {

}

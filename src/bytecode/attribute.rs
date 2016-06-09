use super::collections::ReadChunks;

pub trait Attribute {
    fn len() -> usize;
}

pub struct ConstantValue {

}

impl Attribute for ConstantValue {
    fn len() -> usize { 6 }
}

pub enum AttributeType {
    ConstantValue
}

impl AttributeType {
    pub fn detect(stream: &ReadChunks, constant_pool: &Vec<u8>) -> AttributeType {
        AttributeType::ConstantValue
    }
}

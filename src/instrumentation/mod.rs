use super::bytecode::classfile::*;

mod asm;

pub struct JavaClass {

}

impl JavaClass {
    pub fn new() -> JavaClass {
        JavaClass {}
    }

    pub fn to_classfile(&self) -> Classfile {
        Classfile::new()
    }

    // TODO: this function should report errors better, instead of just returning nothing on error
    pub fn from_classfile(classfile: &Classfile) -> Option<JavaClass> {
        None
    }
}

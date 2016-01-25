use super::jvmti_native::jvmti_native::jobject;

pub struct Object {
    pub obj_ref: jobject
}

impl Object {

    pub fn new(obj_ref: jobject) -> Object {
        Object {
            obj_ref: obj_ref
        }
    }
}

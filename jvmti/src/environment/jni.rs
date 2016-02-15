use super::super::native::JavaObject;
use super::super::class::ClassId;

pub trait JNI {
    fn get_object_class(&self, object_id: JavaObject) -> ClassId;
}

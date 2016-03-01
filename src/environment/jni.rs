use super::super::native::{JavaObject, JNIEnvPtr};
use super::super::class::ClassId;
use std::ptr;

///
/// `JNI` defines a set of operatations the JVM offers through it's JNI interface.
///
pub trait JNI {

    /// Return an `ClassId` belonging to the given Java object instance.
    fn get_object_class(&self, object_id: &JavaObject) -> ClassId;
}

pub struct JNIEnvironment {

    jni: JNIEnvPtr
}

impl JNI for JNIEnvironment {

    fn get_object_class(&self, object_id: &JavaObject) -> ClassId {
        ClassId { native_id: ptr::null_mut() }
    }
}

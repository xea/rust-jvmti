use super::super::native::{JavaObject, JNIEnvPtr};
use super::super::class::ClassId;

///
/// `JNI` defines a set of operatations the JVM offers through it's JNI interface.
///
pub trait JNI {

    /// Return an `ClassId` belonging to the given Java object instance.
    fn get_object_class(&self, object_id: &JavaObject) -> ClassId;
}

///
/// This is the native implementation of the `JNI` trait. Each trait method call is delegated
/// to the represented JNI instance.
pub struct JNIEnvironment {

    jni: JNIEnvPtr
}

impl JNIEnvironment {

    pub fn new(jni: JNIEnvPtr) -> JNIEnvironment {
        JNIEnvironment { jni: jni }
    }
}

impl JNI for JNIEnvironment {

    fn get_object_class(&self, object_id: &JavaObject) -> ClassId {
        unsafe {
            let class_id = (**self.jni).GetObjectClass.unwrap()(self.jni, *object_id);

            ClassId { native_id: class_id }
        }
    }
}

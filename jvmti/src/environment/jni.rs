use super::super::native::JavaObject;
use super::super::class::ClassId;

///
/// `JNI` defines a set of operatations the JVM offers through it's JNI interface.
///
pub trait JNI {

    /// Return an `ClassId` belonging to the given Java object instance.
    fn get_object_class(&self, object_id: &JavaObject) -> ClassId;
}

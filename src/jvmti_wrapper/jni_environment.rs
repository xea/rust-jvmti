use super::JniPtr;
use super::class::Class;
use super::error::NativeError;
use super::jvmti_environment::JvmtiEnvironment;
use super::object::Object;

pub struct JniEnvironment {
    env: JniPtr
}

impl JniEnvironment {

    pub fn new(env_ptr: JniPtr) -> JniEnvironment {
        JniEnvironment {
            env: env_ptr
        }
    }

//  pub GetObjectClass: ::std::option::Option<unsafe extern "C" fn(env: *mut JNIEnv, obj: jobject) -> jclass>,
    pub fn get_object_class(&self, jvmti: JvmtiEnvironment, object: &Object) -> Class {
        unsafe {
            let class_id = (**self.env).GetObjectClass.unwrap()(self.env, object.obj_ref);
            let class = Class::new(&jvmti, class_id);

            class
        }
    }
}

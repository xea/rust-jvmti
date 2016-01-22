extern crate libc;

pub mod jvm_agent;
pub mod jvmti_environment;
pub mod agent_capabilities;
pub mod event_callbacks;
pub mod class;
pub mod method;
pub mod method_signature;
mod jvmti_native;
mod error;

pub use self::error::*;
use self::jvmti_native::jvmti_native::*;
use libc::c_void;

// Mutable pointer to a JVM Environment
pub type JavaVMPtr = *mut JavaVM;
// Mutable pointer to a JVMT Environment
pub type EnvPtr = *mut jvmtiEnv;
/// Standard return value type for JVMTI functions
pub type ReturnValue = jint;
/// Typed alias to C void *'s
pub type VoidPtr = *mut c_void;
/// Typed alias to pointers to Java objects
pub type ObjectPtr = *mut Struct__jobject;

pub type JObject = Struct__jobject;

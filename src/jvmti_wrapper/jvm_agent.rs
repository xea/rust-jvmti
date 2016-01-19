//use super::jvmti_native::jvmti_native::JavaVM;
use super::JavaVMPtr;

const JVM_AGENT_VERSION: u32 = 0x00000001;

/// Encapsulates a native JVMTI JVM environment structure for more Rust-idiomatic functionality
pub struct JvmAgent {
    version: u32,
    jvmPtr: JavaVMPtr
}

impl JvmAgent {
    /// Create a new JvmAgent instance.
    pub fn new(jvm_ptr: JavaVMPtr) -> JvmAgent {
        JvmAgent {
            version: JVM_AGENT_VERSION,
            jvmPtr: jvm_ptr
        }
    }

    /// Return a string representation of this instance
    pub fn to_string(&self) -> String {
        return format!("JVM Agent v{}", self.version);
    }
}

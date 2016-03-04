use super::capabilities::Capabilities;
use super::error::NativeError;
use super::environment::jvm::JVMF;
use super::environment::jvmti::{JVMTI};
use super::version::VersionNumber;

/// Allows testing of JVM and JVMTI-related functions by emulating (mocking) a JVM agent.
pub struct JVMEmulator {
    capabilities: Capabilities
}

impl JVMEmulator {
    pub fn new() -> JVMEmulator {
        JVMEmulator {
            capabilities: Capabilities::new()
        }
    }
}

impl JVMF for JVMEmulator {
    fn get_environment(&self) -> Result<Box<JVMTI>, NativeError> {
        Ok(Box::new(JVMEmulator::new()))
    }

    fn destroy(&self) -> Result<(), NativeError> {
        Ok(())
    }
}

impl JVMTI for JVMEmulator {

    fn get_version_number(&self) -> VersionNumber {
        VersionNumber::unknown()
    }

    fn add_capabilities(&self, new_capabilities: Capabilities) -> Result<Capabilities, NativeError> {
        Ok(self.capabilities.merge(&new_capabilities))
    }

    fn get_capabilities(&self) -> Capabilities {
        Capabilities::new()
    }
}

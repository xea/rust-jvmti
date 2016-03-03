use super::capabilities::Capabilities;
use super::error::NativeError;
use super::environment::jvm::JVMF;
use super::environment::jvmti::{JVMTI};
use super::version::VersionNumber;

pub struct JVMEmulator;

impl JVMF for JVMEmulator {
    fn get_environment(&self) -> Result<Box<JVMTI>, NativeError> {
        Ok(Box::new(JVMEmulator))
    }
}

impl JVMTI for JVMEmulator {

    fn get_version_number(&self) -> VersionNumber {
        VersionNumber::unknown()
    }

    fn get_capabilities(&self) -> Capabilities {
        Capabilities::new()
    }
}

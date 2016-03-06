use self::jvmti::{JVMTI, JVMTIEnvironment};
use self::jni::{JNI, JNIEnvironment};
use super::capabilities::Capabilities;
use super::class::ClassId;
use super::error::NativeError;
use super::event::{EventCallbacks, VMEvent};
use super::native::JavaObject;
use super::version::VersionNumber;


pub mod jni;
pub mod jvm;
pub mod jvmti;

pub struct Environment {

    jvmti: JVMTIEnvironment,
    jni: JNIEnvironment
}

impl JVMTI for Environment {

    fn get_version_number(&self) -> VersionNumber {
        self.jvmti.get_version_number()
    }

    fn add_capabilities(&mut self, new_capabilities: &Capabilities) -> Result<Capabilities, NativeError> {
        self.jvmti.add_capabilities(new_capabilities)
    }

    fn get_capabilities(&self) -> Capabilities {
        self.jvmti.get_capabilities()
    }

    fn set_event_callbacks(&mut self, callbacks: EventCallbacks) -> Option<NativeError> {
        self.jvmti.set_event_callbacks(callbacks)
    }

    fn set_event_notification_mode(&mut self, event: VMEvent, mode: bool) -> Option<NativeError> {
        self.jvmti.set_event_notification_mode(event, mode)
    }
}

impl JNI for Environment {

    fn get_object_class(&self, object_id: &JavaObject) -> ClassId {
        self.jni.get_object_class(object_id)
    }
}

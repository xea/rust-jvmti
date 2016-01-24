use super::jvmti_native::jvmti_native::jthread;

pub struct Thread {
    native_ptr: jthread,
    pub info: Option<ThreadInfo>
}

impl Thread {
    pub fn new(ptr: jthread) -> Thread {
        Thread {
            native_ptr: ptr,
            info: None
        }
    }
}

pub struct ThreadInfo {
    pub name: String,
    pub priority: u32,
    pub is_daemon: bool
}

use super::native::JavaThread;

///
/// Represents a link between a JVM thread and the Rust code calling the JVMTI API.
///
pub struct ThreadId {
    pub native_id: JavaThread,
}

pub struct Thread {
    pub id: ThreadId,
    pub name: String,
    pub priority: u32,
    pub is_daemon: bool
}

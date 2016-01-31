use super::native::JavaThread;

pub struct ThreadId {
    pub native_id: JavaThread,
}

pub struct Thread {
    pub id: ThreadId,
    pub name: String,
    pub priority: u32,
    pub is_daemon: bool
}

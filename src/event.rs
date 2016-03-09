use super::native::jvmti_native::*;
use super::thread::Thread;

pub type FnMethodEntry = fn() -> ();
pub type FnMethodExit = fn() -> ();
pub type FnVMInit = fn() -> ();
pub type FnVMDeath = fn() -> ();
pub type FnVMStart = fn() -> ();
pub type FnVMObjectAlloc = fn() -> ();
pub type FnThreadStart = fn(thread: Thread) -> ();
pub type FnThreadEnd = fn(thread: Thread) -> ();
pub type FnException = fn() -> ();
pub type FnExceptionCatch = fn() -> ();
pub type FnMonitorWait = fn(thread: Thread) -> ();
pub type FnMonitorWaited = fn(thread: Thread) -> ();
pub type FnMonitorContendedEnter = fn(thread: Thread) -> ();
pub type FnMonitorContendedEntered = fn(thread: Thread) -> ();
pub type FnFieldAccess = fn() -> ();
pub type FnFieldModification = fn() -> ();
pub type FnGarbageCollectionStart = fn() -> ();
pub type FnGarbageCollectionFinish = fn() -> ();

///
/// `VMEvent` represents events that can occur in JVM applications. These events can be handled
/// using event handlers. For each event a corresponding handler will be called.
///
#[allow(dead_code)]
#[derive(Hash, Eq, PartialEq)]
pub enum VMEvent {
    VMInit = JVMTI_EVENT_VM_INIT as isize,
    VMDeath = JVMTI_EVENT_VM_DEATH as isize,
    VMObjectAlloc = JVMTI_EVENT_VM_OBJECT_ALLOC as isize,
    VMStart = JVMTI_EVENT_VM_START as isize,
    MethodEntry = JVMTI_EVENT_METHOD_ENTRY as isize,
    MethodExit = JVMTI_EVENT_METHOD_EXIT as isize,
    ThreadStart = JVMTI_EVENT_THREAD_START as isize,
    ThreadEnd = JVMTI_EVENT_THREAD_END as isize,
    Exception = JVMTI_EVENT_EXCEPTION as isize,
    ExceptionCatch = JVMTI_EVENT_EXCEPTION_CATCH as isize,
    MonitorWait = JVMTI_EVENT_MONITOR_WAIT as isize,
    MonitorWaited = JVMTI_EVENT_MONITOR_WAITED as isize,
    MonitorContendedEnter = JVMTI_EVENT_MONITOR_CONTENDED_ENTER as isize,
    MonitorContendedEntered = JVMTI_EVENT_MONITOR_CONTENDED_ENTERED as isize,
    FieldAccess = JVMTI_EVENT_FIELD_ACCESS as isize,
    FieldModification = JVMTI_EVENT_FIELD_MODIFICATION as isize,
    GarbageCollectionStart = JVMTI_EVENT_GARBAGE_COLLECTION_START as isize,
    GarbageCollectionFinish = JVMTI_EVENT_GARBAGE_COLLECTION_FINISH as isize
    // TODO add remaining events
}

///
/// The `EventCallbacks` structure is used to define a set of event handlers that the JVM will call
/// when an event fires.
///
#[derive(Default, Clone)]
pub struct EventCallbacks {
    pub vm_init: Option<FnVMInit>,
    pub vm_death: Option<FnVMDeath>,
    pub vm_object_alloc: Option<FnVMObjectAlloc>,
    pub vm_start: Option<FnVMStart>,
    pub method_entry: Option<FnMethodEntry>,
    pub method_exit: Option<FnMethodExit>,
    pub thread_start: Option<FnThreadStart>,
    pub thread_end: Option<FnThreadEnd>,
    pub exception: Option<FnException>,
    pub exception_catch: Option<FnExceptionCatch>,
    pub monitor_wait: Option<FnMonitorWait>,
    pub monitor_waited: Option<FnMonitorWaited>,
    pub monitor_contended_enter: Option<FnMonitorContendedEnter>,
    pub monitor_contended_entered: Option<FnMonitorContendedEntered>,
    pub field_access: Option<FnFieldAccess>,
    pub field_modification: Option<FnFieldModification>,
    pub garbage_collection_start: Option<FnGarbageCollectionStart>,
    pub garbage_collection_finish: Option<FnGarbageCollectionFinish>
}

impl EventCallbacks {

    pub fn new() -> EventCallbacks {
        EventCallbacks { ..Default::default() }
    }
}

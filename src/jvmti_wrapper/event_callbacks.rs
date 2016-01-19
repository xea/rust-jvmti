use super::jvmti_native::jvmti_native::*;
use super::error::NativeError;

pub type FnVMInit = extern fn() -> ();
pub type FnMethodEntry = extern fn() -> ();
pub type FnVMObjectAlloc = extern fn() -> ();

pub static mut CALLBACK_TABLE: EventCallbacks = EventCallbacks {
    vm_init: None,
    vm_object_alloc: None,
    method_entry: None
};

#[derive(Default)]
pub struct EventCallbacks {
    pub vm_init: Option<FnVMInit>,
    pub vm_object_alloc: Option<FnVMObjectAlloc>,
    pub method_entry: Option<FnMethodEntry>
}

pub enum VMEvent {
    VMObjectAlloc = JVMTI_EVENT_VM_OBJECT_ALLOC as isize,
    VMStart = JVMTI_EVENT_VM_START as isize,
    MethodEntry = JVMTI_EVENT_METHOD_ENTRY as isize
}

impl EventCallbacks {

    pub fn new() -> EventCallbacks {
        EventCallbacks {
            ..Default::default()
        }
    }

    pub fn to_native(&self) -> jvmtiEventCallbacks {
        jvmtiEventCallbacks {
            VMInit: None, //jvmtiEventVMInit,
            VMDeath: None, //jvmtiEventVMDeath,
            ThreadStart: None, //jvmtiEventThreadStart,
            ThreadEnd: None, //jvmtiEventThreadEnd,
            ClassFileLoadHook: None, //jvmtiEventClassFileLoadHook,
            ClassLoad: None, //jvmtiEventClassLoad,
            ClassPrepare: None, //jvmtiEventClassPrepare,
            VMStart: None, //jvmtiEventVMStart,
            Exception: None, //jvmtiEventException,
            ExceptionCatch: None, //jvmtiEventExceptionCatch,
            SingleStep: None, //jvmtiEventSingleStep,
            FramePop: None, //jvmtiEventFramePop,
            Breakpoint: None, //jvmtiEventBreakpoint,
            FieldAccess: None, //jvmtiEventFieldAccess,
            FieldModification: None, //jvmtiEventFieldModification,
            MethodEntry: Some(local_cb_method_entry), //jvmtiEventMethodEntry,
            MethodExit: None, //jvmtiEventMethodExit,
            NativeMethodBind: None, //jvmtiEventNativeMethodBind,
            CompiledMethodLoad: None, //jvmtiEventCompiledMethodLoad,
            CompiledMethodUnload: None, //jvmtiEventCompiledMethodUnload,
            DynamicCodeGenerated: None, //jvmtiEventDynamicCodeGenerated,
            DataDumpRequest: None, //jvmtiEventDataDumpRequest,
            reserved72: None, //jvmtiEventReserved,
            MonitorWait: None, //jvmtiEventMonitorWait,
            MonitorWaited: None, //jvmtiEventMonitorWaited,
            MonitorContendedEnter: None, //jvmtiEventMonitorContendedEnter,
            MonitorContendedEntered: None, //jvmtiEventMonitorContendedEntered,
            reserved77: None, //jvmtiEventReserved,
            reserved78: None, //jvmtiEventReserved,
            reserved79: None, //jvmtiEventReserved,
            ResourceExhausted: None, //jvmtiEventResourceExhausted,
            GarbageCollectionStart: None, //jvmtiEventGarbageCollectionStart,
            GarbageCollectionFinish: None, //jvmtiEventGarbageCollectionFinish,
            ObjectFree: None, //jvmtiEventObjectFree,
            VMObjectAlloc: Some(local_cb_vm_object_alloc) //jvmtiEventVMObjectAlloc,
        }
    }
}

unsafe extern "C" fn local_cb_vm_object_alloc(jvmti_env: *mut jvmtiEnv, jni_env: *mut JNIEnv, thread: jthread, object: jobject, object_klass: jclass, size: jlong) -> () {
    println!("ALLOOALSDFASDF");

    match CALLBACK_TABLE.vm_object_alloc {
        Some(function) => function(),
        None => println!("No dynamic callback method was found")
    }
}

unsafe extern "C" fn local_cb_method_entry(jvmti_env: *mut jvmtiEnv, jni_env: *mut JNIEnv, thread: jthread, method: jmethodID) -> () {
    println!("METHOD ENTRY");

    match CALLBACK_TABLE.method_entry {
        Some(function) => function(),
        None => println!("No dynamic callback method was found")
    }
}

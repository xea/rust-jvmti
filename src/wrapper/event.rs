use super::native::jvmti_native::*;
use super::environment::{Environment, JVMTIEnvironment, JNIEnvironment, JVMTI};
use super::method::*;
use super::native::JavaThread;
use super::thread::Thread;

/// The following are function type declaration for wrapped callback methods
pub type FnException = fn() -> ();
pub type FnExceptionCatch = fn() -> ();
pub type FnMethodEntry = fn(method: Method, thread: Thread) -> ();
pub type FnMethodExit = fn(method: Method, thread: Thread) -> ();
//pub type FnMethodEntry = extern fn(method: Method, thread: Thread) -> ();
//pub type FnMethodExit = extern fn(method: Method) -> ();
pub type FnVMInit = fn() -> ();
pub type FnVMObjectAlloc = fn(size: u64) -> ();

pub static mut CALLBACK_TABLE: EventCallbacks = EventCallbacks {
    vm_init: None,
    vm_object_alloc: None,
    method_entry: None,
    method_exit: None,
    exception: None,
    exception_catch: None
};

#[allow(dead_code)]
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

#[derive(Default, Clone)]
pub struct EventCallbacks {
    pub vm_init: Option<FnVMInit>,
    pub vm_object_alloc: Option<FnVMObjectAlloc>,
    pub method_entry: Option<FnMethodEntry>,
    pub method_exit: Option<FnMethodExit>,
    pub exception: Option<FnException>,
    pub exception_catch: Option<FnExceptionCatch>
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
            Exception: Some(local_cb_exception), //jvmtiEventException,
            ExceptionCatch: Some(local_cb_exception_catch), //jvmtiEventExceptionCatch,
            SingleStep: None, //jvmtiEventSingleStep,
            FramePop: None, //jvmtiEventFramePop,
            Breakpoint: None, //jvmtiEventBreakpoint,
            FieldAccess: None, //jvmtiEventFieldAccess,
            FieldModification: None, //jvmtiEventFieldModification,
            MethodEntry: Some(local_cb_method_entry), //jvmtiEventMethodEntry,
            MethodExit: Some(local_cb_method_exit), //jvmtiEventMethodExit,
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

#[allow(unused_variables)]
unsafe extern "C" fn local_cb_vm_object_alloc(jvmti_env: *mut jvmtiEnv, jni_env: *mut JNIEnv, thread: jthread, object: jobject, object_klass: jclass, size: jlong) -> () {
    let env = Environment::new(JVMTIEnvironment::new(jvmti_env), JNIEnvironment::new(jni_env));

    match CALLBACK_TABLE.vm_object_alloc {
        Some(function) => function(size as u64),
        None => println!("No dynamic callback method was found for VM object allocation")
    }
}

#[allow(unused_variables)]
unsafe extern "C" fn local_cb_method_entry(jvmti_env: *mut jvmtiEnv, jni_env: *mut JNIEnv, thread: JavaThread, method: jmethodID) -> () {
    let env = Environment::new(JVMTIEnvironment::new(jvmti_env), JNIEnvironment::new(jni_env));
    let current_thread = env.get_thread_info(&thread).ok().unwrap();
    let method_id = MethodId { native_id : method };

    match CALLBACK_TABLE.method_entry {
        Some(function) => {
            match env.get_method_name(&method_id) {
                Ok(signature) => function(Method::new(method_id, signature), current_thread),
                Err(_) => function(Method::unknown(), current_thread)
            }
        },
        None => println!("No dynamic callback method was found for method entry")
    }
}

#[allow(unused_variables)]
unsafe extern "C" fn local_cb_method_exit(jvmti_env: *mut jvmtiEnv, jni_env: *mut JNIEnv, thread: jthread, method: jmethodID, was_popped_by_exception: jboolean, return_value: jvalue) -> () {
    let env = Environment::new(JVMTIEnvironment::new(jvmti_env), JNIEnvironment::new(jni_env));
    let current_thread = env.get_thread_info(&thread).ok().unwrap();
    let method_id = MethodId { native_id : method };

    match CALLBACK_TABLE.method_exit {
        Some(function) => {
            match env.get_method_name(&method_id) {
                Ok(signature) => function(Method::new(method_id, signature), current_thread),
                Err(_) => function(Method::unknown(), current_thread)
            }
        }
        None => println!("No dynamic callback method was found for method exit")
    }
}

#[allow(unused_variables)]
unsafe extern "C" fn local_cb_exception(jvmti_env: *mut jvmtiEnv, jni_env: *mut JNIEnv, thread: jthread, method: jmethodID, location: jlocation, exception: jobject, catch_method: jmethodID, catch_location: jlocation) -> () {
    let jni = JNIEnvironment::new(jni_env);
    let jvmti = JVMTIEnvironment::new(jvmti_env);
    let env = Environment::new(jvmti, jni);

    match CALLBACK_TABLE.exception {
        Some(function) => function(),
        None => println!("No dynamic callback method was found for exception")
    }
}

#[allow(unused_variables)]
unsafe extern "C" fn local_cb_exception_catch(jvmti_env: *mut jvmtiEnv, jni_env: *mut JNIEnv, thread: jthread, method: jmethodID, location: jlocation, exception: jobject) -> () {
    let jni = JNIEnvironment::new(jni_env);
    let jvmti = JVMTIEnvironment::new(jvmti_env);
    let env = Environment::new(jvmti, jni);

    match CALLBACK_TABLE.exception_catch {
        Some(function) => function(),
        None => println!("No dynamic callback method was found for exception catch")
    }
}

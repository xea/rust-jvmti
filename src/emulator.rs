use super::native::*;
use super::native::jvmti_native::*;
use super::error::NativeError;
use libc::{c_void, c_char, c_uchar};
use std::mem::transmute;
use std::ptr;

///
/// Allows emulating a Java Virtual Machine.
///
#[allow(non_snake_case)]
#[derive(Debug)]
pub struct Emulator {

    pub reserved0: *mut c_void,
    pub reserved1: *mut c_void,
    pub reserved2: *mut c_void,

    pub DestroyJavaVM: Option<fn(vm: JavaVMPtr) -> jint>,
    pub AttachCurrentThread: Option<fn(vm: JavaVMPtr, penv: *mut *mut c_void, args: *mut c_void) -> jint>,
    pub DetachCurrentThread: Option<fn(vm: JavaVMPtr) -> jint>,
    pub GetEnv: Option<fn(vm: JavaVMPtr, penv: *mut *mut c_void, version: jint) -> jint>,
    pub AttachCurrentThreadAsDaemon: Option<fn(vm: JavaVMPtr, penv: *mut *mut c_void, args: *mut c_void) -> jint>
}

impl Emulator {

    /// Create a new `Emulator` instance
    pub fn new() -> Emulator {

        let emulator = Emulator {
            reserved0: ptr::null_mut(),
            reserved1: ptr::null_mut(),
            reserved2: ptr::null_mut(),
            DestroyJavaVM: None,
            AttachCurrentThread: None,
            DetachCurrentThread: None,
            GetEnv: Some(Emulator::get_emulated_environment),
            AttachCurrentThreadAsDaemon: None
        };

        return emulator;
    }

    ///
    /// Convert a pointer to an `Emulator` instance to a `JavaVMPtr` instance.
    ///
    pub fn transmute(emu_ptr: *mut *mut Emulator) -> *mut *const JNIInvokeInterface {
        unsafe { transmute(emu_ptr) }
    }

    pub fn backmute<'a>(vm: JavaVMPtr) -> &'a Emulator {
        unsafe {
            let emu_ptr: *mut *mut Emulator = transmute(vm);
            return &(**emu_ptr);
        }
    }

    #[allow(unused_variables)]
    pub fn get_emulated_environment(vm: JavaVMPtr, penv: *mut *mut c_void, version: jint) -> jint {
        unsafe {
            *penv = (**vm).reserved0;
        }
/*
        let jvmti_emulator = static_instances::JVM;

        unsafe {
            *penv = transmute(Box::new(jvmti_emulator));
        }
        */

        NativeError::NoError as i32
    }
}

/// Emulates the JVM TI API for internal testing purposes
#[allow(non_snake_case)]
#[derive(Debug)]
pub struct JVMTIEmulator {
    pub reserved1: *mut c_void,
    pub SetEventNotificationMode: Option<fn(env: *mut jvmtiEnv, mode: jvmtiEventMode, event_type: jvmtiEvent, event_thread: jthread) -> jvmtiError>,
    pub GetAllThreads: Option<fn(env: *mut jvmtiEnv, threads_count_ptr: *mut jint, threads_ptr: *mut *mut jthread) -> jvmtiError>,
    pub reserved3: *mut c_void,
    pub SuspendThread: Option<fn(env: *mut jvmtiEnv, thread: jthread) -> jvmtiError>,
    pub ResumeThread: Option<fn(env: *mut jvmtiEnv, thread: jthread) -> jvmtiError>,
    pub StopThread: Option<fn(env: *mut jvmtiEnv, thread: jthread, exception: jobject) -> jvmtiError>,
    pub InterruptThread: Option<fn(env: *mut jvmtiEnv, thread: jthread) -> jvmtiError>,
    pub GetThreadInfo: Option<fn(env: *mut jvmtiEnv, thread: jthread, info_ptr: *mut jvmtiThreadInfo) -> jvmtiError>,
    pub GetOwnedMonitorInfo: Option<fn(env: *mut jvmtiEnv, thread: jthread, owned_monitor_count_ptr: *mut jint, owned_monitors_ptr: *mut *mut jobject) -> jvmtiError>,
    pub GetCurrentContendedMonitor: Option<fn(env: *mut jvmtiEnv, thread: jthread, monitor_ptr: *mut jobject) -> jvmtiError>,
    pub RunAgentThread: Option<fn(env: *mut jvmtiEnv, thread: jthread, _proc: jvmtiStartFunction, arg: *const c_void, priority: jint) -> jvmtiError>,
    pub GetTopThreadGroups: Option<fn(env: *mut jvmtiEnv, group_count_ptr: *mut jint, groups_ptr: *mut *mut jthreadGroup) -> jvmtiError>,
    pub GetThreadGroupInfo: Option<fn(env: *mut jvmtiEnv, group: jthreadGroup, info_ptr: *mut jvmtiThreadGroupInfo) -> jvmtiError>,
    pub GetThreadGroupChildren: Option<fn(env: *mut jvmtiEnv, group: jthreadGroup, thread_count_ptr: *mut jint, threads_ptr: *mut *mut jthread, group_count_ptr: *mut jint, groups_ptr: *mut *mut jthreadGroup) -> jvmtiError>,
    pub GetFrameCount: Option<fn(env: *mut jvmtiEnv, thread: jthread, count_ptr: *mut jint) -> jvmtiError>,
    pub GetThreadState: Option<fn(env: *mut jvmtiEnv, thread: jthread, thread_state_ptr: *mut jint) -> jvmtiError>,
    pub GetCurrentThread: Option<fn(env: *mut jvmtiEnv, thread_ptr: *mut jthread) -> jvmtiError>,
    pub GetFrameLocation: Option<fn(env: *mut jvmtiEnv, thread: jthread, depth: jint, method_ptr: *mut jmethodID, location_ptr: *mut jlocation) -> jvmtiError>,
    pub NotifyFramePop: Option<fn(env: *mut jvmtiEnv, thread: jthread, depth: jint) -> jvmtiError>,
    pub GetLocalObject: Option<fn(env: *mut jvmtiEnv, thread: jthread, depth: jint, slot: jint, value_ptr: *mut jobject) -> jvmtiError>,
    pub GetLocalInt: Option<fn(env: *mut jvmtiEnv, thread: jthread, depth: jint, slot: jint, value_ptr: *mut jint) -> jvmtiError>,
    pub GetLocalLong: Option<fn(env: *mut jvmtiEnv, thread: jthread, depth: jint, slot: jint, value_ptr: *mut jlong) -> jvmtiError>,
    pub GetLocalFloat: Option<fn(env: *mut jvmtiEnv, thread: jthread, depth: jint, slot: jint, value_ptr: *mut jfloat) -> jvmtiError>,
    pub GetLocalDouble: Option<fn(env: *mut jvmtiEnv, thread: jthread, depth: jint, slot: jint, value_ptr: *mut jdouble) -> jvmtiError>,
    pub SetLocalObject: Option<fn(env: *mut jvmtiEnv, thread: jthread, depth: jint, slot: jint, value: jobject) -> jvmtiError>,
    pub SetLocalInt: Option<fn(env: *mut jvmtiEnv, thread: jthread, depth: jint, slot: jint, value: jint) -> jvmtiError>,
    pub SetLocalLong: Option<fn(env: *mut jvmtiEnv, thread: jthread, depth: jint, slot: jint, value: jlong) -> jvmtiError>,
    pub SetLocalFloat: Option<fn(env: *mut jvmtiEnv, thread: jthread, depth: jint, slot: jint, value: jfloat) -> jvmtiError>,
    pub SetLocalDouble: Option<fn(env: *mut jvmtiEnv, thread: jthread, depth: jint, slot: jint, value: jdouble) -> jvmtiError>,
    pub CreateRawMonitor: Option<fn(env: *mut jvmtiEnv, name: *const c_char, monitor_ptr: *mut jrawMonitorID) -> jvmtiError>,
    pub DestroyRawMonitor: Option<fn(env: *mut jvmtiEnv, monitor: jrawMonitorID) -> jvmtiError>,
    pub RawMonitorEnter: Option<fn(env: *mut jvmtiEnv, monitor: jrawMonitorID) -> jvmtiError>,
    pub RawMonitorExit: Option<fn(env: *mut jvmtiEnv, monitor: jrawMonitorID) -> jvmtiError>,
    pub RawMonitorWait: Option<fn(env: *mut jvmtiEnv, monitor: jrawMonitorID, millis: jlong) -> jvmtiError>,
    pub RawMonitorNotify: Option<fn(env: *mut jvmtiEnv, monitor: jrawMonitorID) -> jvmtiError>,
    pub RawMonitorNotifyAll: Option<fn(env: *mut jvmtiEnv, monitor: jrawMonitorID) -> jvmtiError>,
    pub SetBreakpoint: Option<fn(env: *mut jvmtiEnv, method: jmethodID, location: jlocation) -> jvmtiError>,
    pub ClearBreakpoint: Option<fn(env: *mut jvmtiEnv, method: jmethodID, location: jlocation) -> jvmtiError>,
    pub reserved40: *mut c_void,
    pub SetFieldAccessWatch: Option<fn(env: *mut jvmtiEnv, klass: jclass, field: jfieldID) -> jvmtiError>,
    pub ClearFieldAccessWatch: Option<fn(env: *mut jvmtiEnv, klass: jclass, field: jfieldID) -> jvmtiError>,
    pub SetFieldModificationWatch: Option<fn(env: *mut jvmtiEnv, klass: jclass, field: jfieldID) -> jvmtiError>,
    pub ClearFieldModificationWatch: Option<fn(env: *mut jvmtiEnv, klass: jclass, field: jfieldID) -> jvmtiError>,
    pub IsModifiableClass: Option<fn(env: *mut jvmtiEnv, klass: jclass, is_modifiable_class_ptr: *mut jboolean) -> jvmtiError>,
    pub Allocate: Option<fn(env: *mut jvmtiEnv, size: jlong, mem_ptr: *mut *mut c_uchar) -> jvmtiError>,
    pub Deallocate: Option<fn(env: *mut jvmtiEnv, mem: *mut c_uchar) -> jvmtiError>,
    pub GetClassSignature: Option<fn(env: *mut jvmtiEnv, klass: jclass, signature_ptr: *mut *mut c_char, generic_ptr: *mut *mut c_char) -> jvmtiError>,
    pub GetClassStatus: Option<fn(env: *mut jvmtiEnv, klass: jclass, status_ptr: *mut jint) -> jvmtiError>,
    pub GetSourceFileName: Option<fn(env: *mut jvmtiEnv, klass: jclass, source_name_ptr: *mut *mut c_char) -> jvmtiError>,
    pub GetClassModifiers: Option<fn(env: *mut jvmtiEnv, klass: jclass, modifiers_ptr: *mut jint) -> jvmtiError>,
    pub GetClassMethods: Option<fn(env: *mut jvmtiEnv, klass: jclass, method_count_ptr: *mut jint, methods_ptr: *mut *mut jmethodID) -> jvmtiError>,
    pub GetClassFields: Option<fn(env: *mut jvmtiEnv, klass: jclass, field_count_ptr: *mut jint, fields_ptr: *mut *mut jfieldID) -> jvmtiError>,
    pub GetImplementedInterfaces: Option<fn(env: *mut jvmtiEnv, klass: jclass, interface_count_ptr: *mut jint, interfaces_ptr: *mut *mut jclass) -> jvmtiError>,
    pub IsInterface: Option<fn(env: *mut jvmtiEnv, klass: jclass, is_interface_ptr: *mut jboolean) -> jvmtiError>,
    pub IsArrayClass: Option<fn(env: *mut jvmtiEnv, klass: jclass, is_array_class_ptr: *mut jboolean) -> jvmtiError>,
    pub GetClassLoader: Option<fn(env: *mut jvmtiEnv, klass: jclass, classloader_ptr: *mut jobject) -> jvmtiError>,
    pub GetObjectHashCode: Option<fn(env: *mut jvmtiEnv, object: jobject, hash_code_ptr: *mut jint) -> jvmtiError>,
    pub GetObjectMonitorUsage: Option<fn(env: *mut jvmtiEnv, object: jobject, info_ptr: *mut jvmtiMonitorUsage) -> jvmtiError>,
    pub GetFieldName: Option<fn(env: *mut jvmtiEnv, klass: jclass, field: jfieldID, name_ptr: *mut *mut c_char, signature_ptr: *mut *mut c_char, generic_ptr: *mut *mut c_char) -> jvmtiError>,
    pub GetFieldDeclaringClass: Option<fn(env: *mut jvmtiEnv, klass: jclass, field: jfieldID, declaring_class_ptr: *mut jclass) -> jvmtiError>,
    pub GetFieldModifiers: Option<fn(env: *mut jvmtiEnv, klass: jclass, field: jfieldID, modifiers_ptr: *mut jint) -> jvmtiError>,
    pub IsFieldSynthetic: Option<fn(env: *mut jvmtiEnv, klass: jclass, field: jfieldID, is_synthetic_ptr: *mut jboolean) -> jvmtiError>,
    pub GetMethodName: Option<fn(env: *mut jvmtiEnv, method: jmethodID, name_ptr: *mut *mut c_char, signature_ptr: *mut *mut c_char, generic_ptr: *mut *mut c_char) -> jvmtiError>,
    pub GetMethodDeclaringClass: Option<fn(env: *mut jvmtiEnv, method: jmethodID, declaring_class_ptr: *mut jclass) -> jvmtiError>,
    pub GetMethodModifiers: Option<fn(env: *mut jvmtiEnv, method: jmethodID, modifiers_ptr: *mut jint) -> jvmtiError>,
    pub reserved67: *mut c_void,
    pub GetMaxLocals: Option<fn(env: *mut jvmtiEnv, method: jmethodID, max_ptr: *mut jint) -> jvmtiError>,
    pub GetArgumentsSize: Option<fn(env: *mut jvmtiEnv, method: jmethodID, size_ptr: *mut jint) -> jvmtiError>,
    pub GetLineNumberTable: Option<fn(env: *mut jvmtiEnv, method: jmethodID, entry_count_ptr: *mut jint, table_ptr: *mut *mut jvmtiLineNumberEntry) -> jvmtiError>,
    pub GetMethodLocation: Option<fn(env: *mut jvmtiEnv, method: jmethodID, start_location_ptr: *mut jlocation, end_location_ptr: *mut jlocation) -> jvmtiError>,
    pub GetLocalVariableTable: Option<fn(env: *mut jvmtiEnv, method: jmethodID, entry_count_ptr: *mut jint, table_ptr: *mut *mut jvmtiLocalVariableEntry) -> jvmtiError>,
    pub SetNativeMethodPrefix: Option<fn(env: *mut jvmtiEnv, prefix: *const c_char) -> jvmtiError>,
    pub SetNativeMethodPrefixes: Option<fn(env: *mut jvmtiEnv, prefix_count: jint, prefixes: *mut *mut c_char) -> jvmtiError>,
    pub GetBytecodes: Option<fn(env: *mut jvmtiEnv, method: jmethodID, bytecode_count_ptr: *mut jint, bytecodes_ptr: *mut *mut c_uchar) -> jvmtiError>,
    pub IsMethodNative: Option<fn(env: *mut jvmtiEnv, method: jmethodID, is_native_ptr: *mut jboolean) -> jvmtiError>,
    pub IsMethodSynthetic: Option<fn(env: *mut jvmtiEnv, method: jmethodID, is_synthetic_ptr: *mut jboolean) -> jvmtiError>,
    pub GetLoadedClasses: Option<fn(env: *mut jvmtiEnv, class_count_ptr: *mut jint, classes_ptr: *mut *mut jclass) -> jvmtiError>,
    pub GetClassLoaderClasses: Option<fn(env: *mut jvmtiEnv, initiating_loader: jobject, class_count_ptr: *mut jint, classes_ptr: *mut *mut jclass) -> jvmtiError>,
    pub PopFrame: Option<fn(env: *mut jvmtiEnv, thread: jthread) -> jvmtiError>,
    pub ForceEarlyReturnObject: Option<fn(env: *mut jvmtiEnv, thread: jthread, value: jobject) -> jvmtiError>,
    pub ForceEarlyReturnInt: Option<fn(env: *mut jvmtiEnv, thread: jthread, value: jint) -> jvmtiError>,
    pub ForceEarlyReturnLong: Option<fn(env: *mut jvmtiEnv, thread: jthread, value: jlong) -> jvmtiError>,
    pub ForceEarlyReturnFloat: Option<fn(env: *mut jvmtiEnv, thread: jthread, value: jfloat) -> jvmtiError>,
    pub ForceEarlyReturnDouble: Option<fn(env: *mut jvmtiEnv, thread: jthread, value: jdouble) -> jvmtiError>,
    pub ForceEarlyReturnVoid: Option<fn(env: *mut jvmtiEnv, thread: jthread) -> jvmtiError>,
    pub RedefineClasses: Option<fn(env: *mut jvmtiEnv, class_count: jint, class_definitions: *const jvmtiClassDefinition) -> jvmtiError>,
    pub GetVersionNumber: Option<fn(env: *mut jvmtiEnv, version_ptr: *mut jint) -> jvmtiError>,
    pub GetCapabilities: Option<fn(env: *mut jvmtiEnv, capabilities_ptr: *mut jvmtiCapabilities) -> jvmtiError>,
    pub GetSourceDebugExtension: Option<fn(env: *mut jvmtiEnv, klass: jclass, source_debug_extension_ptr: *mut *mut c_char) -> jvmtiError>,
    pub IsMethodObsolete: Option<fn(env: *mut jvmtiEnv, method: jmethodID, is_obsolete_ptr: *mut jboolean) -> jvmtiError>,
    pub SuspendThreadList: Option<fn(env: *mut jvmtiEnv, request_count: jint, request_list: *const jthread, results: *mut jvmtiError) -> jvmtiError>,
    pub ResumeThreadList: Option<fn(env: *mut jvmtiEnv, request_count: jint, request_list: *const jthread, results: *mut jvmtiError) -> jvmtiError>,
    pub reserved94: *mut c_void,
    pub reserved95: *mut c_void,
    pub reserved96: *mut c_void,
    pub reserved97: *mut c_void,
    pub reserved98: *mut c_void,
    pub reserved99: *mut c_void,
    pub GetAllStackTraces: Option<fn(env: *mut jvmtiEnv, max_frame_count: jint, stack_info_ptr: *mut *mut jvmtiStackInfo, thread_count_ptr: *mut jint) -> jvmtiError>,
    pub GetThreadListStackTraces: Option<fn(env: *mut jvmtiEnv, thread_count: jint, thread_list: *const jthread, max_frame_count: jint, stack_info_ptr: *mut *mut jvmtiStackInfo) -> jvmtiError>,
    pub GetThreadLocalStorage: Option<fn(env: *mut jvmtiEnv, thread: jthread, data_ptr: *mut *mut c_void) -> jvmtiError>,
    pub SetThreadLocalStorage: Option<fn(env: *mut jvmtiEnv, thread: jthread, data: *const c_void) -> jvmtiError>,
    pub GetStackTrace: Option<fn(env: *mut jvmtiEnv, thread: jthread, start_depth: jint, max_frame_count: jint, frame_buffer: *mut jvmtiFrameInfo, count_ptr: *mut jint) -> jvmtiError>,
    pub reserved105: *mut c_void,
    pub GetTag: Option<fn(env: *mut jvmtiEnv, object: jobject, tag_ptr: *mut jlong) -> jvmtiError>,
    pub SetTag: Option<fn(env: *mut jvmtiEnv, object: jobject, tag: jlong) -> jvmtiError>,
    pub ForceGarbageCollection: Option<fn(env: *mut jvmtiEnv) -> jvmtiError>,
    pub IterateOverObjectsReachableFromObject: Option<fn(env: *mut jvmtiEnv, object: jobject, object_reference_callback: jvmtiObjectReferenceCallback, user_data: *const c_void) -> jvmtiError>,
    pub IterateOverReachableObjects: Option<fn(env: *mut jvmtiEnv, heap_root_callback: jvmtiHeapRootCallback, stack_ref_callback: jvmtiStackReferenceCallback, object_ref_callback: jvmtiObjectReferenceCallback, user_data: *const c_void) -> jvmtiError>,
    pub IterateOverHeap: Option<fn(env: *mut jvmtiEnv, object_filter: jvmtiHeapObjectFilter, heap_object_callback: jvmtiHeapObjectCallback, user_data: *const c_void) -> jvmtiError>,
    pub IterateOverInstancesOfClass: Option<fn(env: *mut jvmtiEnv, klass: jclass, object_filter: jvmtiHeapObjectFilter, heap_object_callback: jvmtiHeapObjectCallback, user_data: *const c_void) -> jvmtiError>,
    pub reserved113: *mut c_void,
    pub GetObjectsWithTags: Option<fn(env: *mut jvmtiEnv, tag_count: jint, tags: *const jlong, count_ptr: *mut jint, object_result_ptr: *mut *mut jobject, tag_result_ptr: *mut *mut jlong) -> jvmtiError>,
    pub FollowReferences: Option<fn(env: *mut jvmtiEnv, heap_filter: jint, klass: jclass, initial_object: jobject, callbacks: *const jvmtiHeapCallbacks, user_data: *const c_void) -> jvmtiError>,
    pub IterateThroughHeap: Option<fn(env: *mut jvmtiEnv, heap_filter: jint, klass: jclass, callbacks: *const jvmtiHeapCallbacks, user_data: *const c_void) -> jvmtiError>,
    pub reserved117: *mut c_void,
    pub reserved118: *mut c_void,
    pub reserved119: *mut c_void,
    pub SetJNIFunctionTable: Option<fn(env: *mut jvmtiEnv, function_table: *const jniNativeInterface) -> jvmtiError>,
    pub GetJNIFunctionTable: Option<fn(env: *mut jvmtiEnv, function_table: *mut *mut jniNativeInterface) -> jvmtiError>,
    pub SetEventCallbacks: Option<fn(env: *mut jvmtiEnv, callbacks: *const jvmtiEventCallbacks, size_of_callbacks: jint) -> jvmtiError>,
    pub GenerateEvents: Option<fn(env: *mut jvmtiEnv, event_type: jvmtiEvent) -> jvmtiError>,
    pub GetExtensionFunctions: Option<fn(env: *mut jvmtiEnv, extension_count_ptr: *mut jint, extensions: *mut *mut jvmtiExtensionFunctionInfo) -> jvmtiError>,
    pub GetExtensionEvents: Option<fn(env: *mut jvmtiEnv, extension_count_ptr: *mut jint, extensions: *mut *mut jvmtiExtensionEventInfo) -> jvmtiError>,
    pub SetExtensionEventCallback: Option<fn(env: *mut jvmtiEnv, extension_event_index: jint, callback: jvmtiExtensionEvent) -> jvmtiError>,
    pub DisposeEnvironment: Option<fn(env: *mut jvmtiEnv) -> jvmtiError>,
    pub GetErrorName: Option<fn(env: *mut jvmtiEnv, error: jvmtiError, name_ptr: *mut *mut c_char) -> jvmtiError>,
    pub GetJLocationFormat: Option<fn(env: *mut jvmtiEnv, format_ptr: *mut jvmtiJlocationFormat) -> jvmtiError>,
    pub GetSystemProperties: Option<fn(env: *mut jvmtiEnv, count_ptr: *mut jint, property_ptr: *mut *mut *mut c_char) -> jvmtiError>,
    pub GetSystemProperty: Option<fn(env: *mut jvmtiEnv, property: *const c_char, value_ptr: *mut *mut c_char) -> jvmtiError>,
    pub SetSystemProperty: Option<fn(env: *mut jvmtiEnv, property: *const c_char, value: *const c_char) -> jvmtiError>,
    pub GetPhase: Option<fn(env: *mut jvmtiEnv, phase_ptr: *mut jvmtiPhase) -> jvmtiError>,
    pub GetCurrentThreadCpuTimerInfo: Option<fn(env: *mut jvmtiEnv, info_ptr: *mut jvmtiTimerInfo) -> jvmtiError>,
    pub GetCurrentThreadCpuTime: Option<fn(env: *mut jvmtiEnv, nanos_ptr: *mut jlong) -> jvmtiError>,
    pub GetThreadCpuTimerInfo: Option<fn(env: *mut jvmtiEnv, info_ptr: *mut jvmtiTimerInfo) -> jvmtiError>,
    pub GetThreadCpuTime: Option<fn(env: *mut jvmtiEnv, thread: jthread, nanos_ptr: *mut jlong) -> jvmtiError>,
    pub GetTimerInfo: Option<fn(env: *mut jvmtiEnv, info_ptr: *mut jvmtiTimerInfo) -> jvmtiError>,
    pub GetTime: Option<fn(env: *mut jvmtiEnv, nanos_ptr: *mut jlong) -> jvmtiError>,
    pub GetPotentialCapabilities: Option<fn(env: *mut jvmtiEnv, capabilities_ptr: *mut jvmtiCapabilities) -> jvmtiError>,
    pub reserved141: *mut c_void,
    pub AddCapabilities: Option<fn(env: *mut jvmtiEnv, capabilities_ptr: *const jvmtiCapabilities) -> jvmtiError>,
    pub RelinquishCapabilities: Option<fn(env: *mut jvmtiEnv, capabilities_ptr: *const jvmtiCapabilities) -> jvmtiError>,
    pub GetAvailableProcessors: Option<fn(env: *mut jvmtiEnv, processor_count_ptr: *mut jint) -> jvmtiError>,
    pub GetClassVersionNumbers: Option<fn(env: *mut jvmtiEnv, klass: jclass, minor_version_ptr: *mut jint, major_version_ptr: *mut jint) -> jvmtiError>,
    pub GetConstantPool: Option<fn(env: *mut jvmtiEnv, klass: jclass, constant_pool_count_ptr: *mut jint, constant_pool_byte_count_ptr: *mut jint, constant_pool_bytes_ptr: *mut *mut c_uchar) -> jvmtiError>,
    pub GetEnvironmentLocalStorage: Option<fn(env: *mut jvmtiEnv, data_ptr: *mut *mut c_void) -> jvmtiError>,
    pub SetEnvironmentLocalStorage: Option<fn(env: *mut jvmtiEnv, data: *const c_void) -> jvmtiError>,
    pub AddToBootstrapClassLoaderSearch: Option<fn(env: *mut jvmtiEnv, segment: *const c_char) -> jvmtiError>,
    pub SetVerboseFlag: Option<fn(env: *mut jvmtiEnv, flag: jvmtiVerboseFlag, value: jboolean) -> jvmtiError>,
    pub AddToSystemClassLoaderSearch: Option<fn(env: *mut jvmtiEnv, segment: *const c_char) -> jvmtiError>,
    pub RetransformClasses: Option<fn(env: *mut jvmtiEnv, class_count: jint, classes: *const jclass) -> jvmtiError>,
    pub GetOwnedMonitorStackDepthInfo: Option<fn(env: *mut jvmtiEnv, thread: jthread, monitor_info_count_ptr: *mut jint, monitor_info_ptr: *mut *mut jvmtiMonitorStackDepthInfo) -> jvmtiError>,
    pub GetObjectSize: Option<fn(env: *mut jvmtiEnv, object: jobject, size_ptr: *mut jlong) -> jvmtiError>,
    pub GetLocalInstance: Option<fn(env: *mut jvmtiEnv, thread: jthread, depth: jint, value_ptr: *mut jobject) -> jvmtiError>,
}

impl JVMTIEmulator {

    pub fn get_version_number(env: *mut jvmtiEnv, version_ptr: *mut jint) -> jvmtiError {
        unsafe {
            *version_ptr = 0x07FA3020;
        }
        NativeError::NoError as u32
    }

    ///
    /// Convert a pointer to an `Emulator` instance to a `JavaVMPtr` instance.
    ///
    pub fn transmute(emulator: *mut *mut JVMTIEmulator) -> JVMTIEnvPtr {
        unsafe { transmute(emulator) }
    }

    pub fn backmute<'a>(env: JVMTIEnvPtr) -> &'a JVMTIEmulator {
        unsafe {
            let env_ptr: *mut *mut JVMTIEmulator = transmute(env);
            return &(**env_ptr);
        }
    }

    pub fn new() -> JVMTIEmulator {
        JVMTIEmulator {
            reserved1: ptr::null_mut(),
            SetEventNotificationMode: None, //Option<fn(env: *mut jvmtiEnv, mode: jvmtiEventMode, event_type: jvmtiEvent, event_thread: jthread) -> jvmtiError>,
            GetAllThreads: None, //Option<fn(env: *mut jvmtiEnv, threads_count_ptr: *mut jint, threads_ptr: *mut *mut jthread) -> jvmtiError>,
            reserved3: ptr::null_mut(), //*mut c_void,
            SuspendThread: None, //Option<fn(env: *mut jvmtiEnv, thread: jthread) -> jvmtiError>,
            ResumeThread: None, //Option<fn(env: *mut jvmtiEnv, thread: jthread) -> jvmtiError>,
            StopThread: None, //Option<fn(env: *mut jvmtiEnv, thread: jthread, exception: jobject) -> jvmtiError>,
            InterruptThread: None, //Option<fn(env: *mut jvmtiEnv, thread: jthread) -> jvmtiError>,
            GetThreadInfo: None, //Option<fn(env: *mut jvmtiEnv, thread: jthread, info_ptr: *mut jvmtiThreadInfo) -> jvmtiError>,
            GetOwnedMonitorInfo: None, //Option<fn(env: *mut jvmtiEnv, thread: jthread, owned_monitor_count_ptr: *mut jint, owned_monitors_ptr: *mut *mut jobject) -> jvmtiError>,
            GetCurrentContendedMonitor: None, //Option<fn(env: *mut jvmtiEnv, thread: jthread, monitor_ptr: *mut jobject) -> jvmtiError>,
            RunAgentThread: None, //Option<fn(env: *mut jvmtiEnv, thread: jthread, _proc: jvmtiStartFunction, arg: *const c_void, priority: jint) -> jvmtiError>,
            GetTopThreadGroups: None, //Option<fn(env: *mut jvmtiEnv, group_count_ptr: *mut jint, groups_ptr: *mut *mut jthreadGroup) -> jvmtiError>,
            GetThreadGroupInfo: None, //Option<fn(env: *mut jvmtiEnv, group: jthreadGroup, info_ptr: *mut jvmtiThreadGroupInfo) -> jvmtiError>,
            GetThreadGroupChildren: None, //Option<fn(env: *mut jvmtiEnv, group: jthreadGroup, thread_count_ptr: *mut jint, threads_ptr: *mut *mut jthread, group_count_ptr: *mut jint, groups_ptr: *mut *mut jthreadGroup) -> jvmtiError>,
            GetFrameCount: None, //Option<fn(env: *mut jvmtiEnv, thread: jthread, count_ptr: *mut jint) -> jvmtiError>,
            GetThreadState: None, //Option<fn(env: *mut jvmtiEnv, thread: jthread, thread_state_ptr: *mut jint) -> jvmtiError>,
            GetCurrentThread: None, //Option<fn(env: *mut jvmtiEnv, thread_ptr: *mut jthread) -> jvmtiError>,
            GetFrameLocation: None, //Option<fn(env: *mut jvmtiEnv, thread: jthread, depth: jint, method_ptr: *mut jmethodID, location_ptr: *mut jlocation) -> jvmtiError>,
            NotifyFramePop: None, //Option<fn(env: *mut jvmtiEnv, thread: jthread, depth: jint) -> jvmtiError>,
            GetLocalObject: None, //Option<fn(env: *mut jvmtiEnv, thread: jthread, depth: jint, slot: jint, value_ptr: *mut jobject) -> jvmtiError>,
            GetLocalInt: None, //Option<fn(env: *mut jvmtiEnv, thread: jthread, depth: jint, slot: jint, value_ptr: *mut jint) -> jvmtiError>,
            GetLocalLong: None, //Option<fn(env: *mut jvmtiEnv, thread: jthread, depth: jint, slot: jint, value_ptr: *mut jlong) -> jvmtiError>,
            GetLocalFloat: None, //Option<fn(env: *mut jvmtiEnv, thread: jthread, depth: jint, slot: jint, value_ptr: *mut jfloat) -> jvmtiError>,
            GetLocalDouble: None, //Option<fn(env: *mut jvmtiEnv, thread: jthread, depth: jint, slot: jint, value_ptr: *mut jdouble) -> jvmtiError>,
            SetLocalObject: None, //Option<fn(env: *mut jvmtiEnv, thread: jthread, depth: jint, slot: jint, value: jobject) -> jvmtiError>,
            SetLocalInt: None, //Option<fn(env: *mut jvmtiEnv, thread: jthread, depth: jint, slot: jint, value: jint) -> jvmtiError>,
            SetLocalLong: None, //Option<fn(env: *mut jvmtiEnv, thread: jthread, depth: jint, slot: jint, value: jlong) -> jvmtiError>,
            SetLocalFloat: None, //Option<fn(env: *mut jvmtiEnv, thread: jthread, depth: jint, slot: jint, value: jfloat) -> jvmtiError>,
            SetLocalDouble: None, //Option<fn(env: *mut jvmtiEnv, thread: jthread, depth: jint, slot: jint, value: jdouble) -> jvmtiError>,
            CreateRawMonitor: None, //Option<fn(env: *mut jvmtiEnv, name: *const c_char, monitor_ptr: *mut jrawMonitorID) -> jvmtiError>,
            DestroyRawMonitor: None, //Option<fn(env: *mut jvmtiEnv, monitor: jrawMonitorID) -> jvmtiError>,
            RawMonitorEnter: None, //Option<fn(env: *mut jvmtiEnv, monitor: jrawMonitorID) -> jvmtiError>,
            RawMonitorExit: None, //Option<fn(env: *mut jvmtiEnv, monitor: jrawMonitorID) -> jvmtiError>,
            RawMonitorWait: None, //Option<fn(env: *mut jvmtiEnv, monitor: jrawMonitorID, millis: jlong) -> jvmtiError>,
            RawMonitorNotify: None, //Option<fn(env: *mut jvmtiEnv, monitor: jrawMonitorID) -> jvmtiError>,
            RawMonitorNotifyAll: None, //Option<fn(env: *mut jvmtiEnv, monitor: jrawMonitorID) -> jvmtiError>,
            SetBreakpoint: None, //Option<fn(env: *mut jvmtiEnv, method: jmethodID, location: jlocation) -> jvmtiError>,
            ClearBreakpoint: None, //Option<fn(env: *mut jvmtiEnv, method: jmethodID, location: jlocation) -> jvmtiError>,
            reserved40: ptr::null_mut(), //*mut c_void,
            SetFieldAccessWatch: None, //Option<fn(env: *mut jvmtiEnv, klass: jclass, field: jfieldID) -> jvmtiError>,
            ClearFieldAccessWatch: None, //Option<fn(env: *mut jvmtiEnv, klass: jclass, field: jfieldID) -> jvmtiError>,
            SetFieldModificationWatch: None, //Option<fn(env: *mut jvmtiEnv, klass: jclass, field: jfieldID) -> jvmtiError>,
            ClearFieldModificationWatch: None, //Option<fn(env: *mut jvmtiEnv, klass: jclass, field: jfieldID) -> jvmtiError>,
            IsModifiableClass: None, //Option<fn(env: *mut jvmtiEnv, klass: jclass, is_modifiable_class_ptr: *mut jboolean) -> jvmtiError>,
            Allocate: None, //Option<fn(env: *mut jvmtiEnv, size: jlong, mem_ptr: *mut *mut c_uchar) -> jvmtiError>,
            Deallocate: None, //Option<fn(env: *mut jvmtiEnv, mem: *mut c_uchar) -> jvmtiError>,
            GetClassSignature: None, //Option<fn(env: *mut jvmtiEnv, klass: jclass, signature_ptr: *mut *mut c_char, generic_ptr: *mut *mut c_char) -> jvmtiError>,
            GetClassStatus: None, //Option<fn(env: *mut jvmtiEnv, klass: jclass, status_ptr: *mut jint) -> jvmtiError>,
            GetSourceFileName: None, //Option<fn(env: *mut jvmtiEnv, klass: jclass, source_name_ptr: *mut *mut c_char) -> jvmtiError>,
            GetClassModifiers: None, //Option<fn(env: *mut jvmtiEnv, klass: jclass, modifiers_ptr: *mut jint) -> jvmtiError>,
            GetClassMethods: None, //Option<fn(env: *mut jvmtiEnv, klass: jclass, method_count_ptr: *mut jint, methods_ptr: *mut *mut jmethodID) -> jvmtiError>,
            GetClassFields: None, //Option<fn(env: *mut jvmtiEnv, klass: jclass, field_count_ptr: *mut jint, fields_ptr: *mut *mut jfieldID) -> jvmtiError>,
            GetImplementedInterfaces: None, //Option<fn(env: *mut jvmtiEnv, klass: jclass, interface_count_ptr: *mut jint, interfaces_ptr: *mut *mut jclass) -> jvmtiError>,
            IsInterface: None, //Option<fn(env: *mut jvmtiEnv, klass: jclass, is_interface_ptr: *mut jboolean) -> jvmtiError>,
            IsArrayClass: None, //Option<fn(env: *mut jvmtiEnv, klass: jclass, is_array_class_ptr: *mut jboolean) -> jvmtiError>,
            GetClassLoader: None, //Option<fn(env: *mut jvmtiEnv, klass: jclass, classloader_ptr: *mut jobject) -> jvmtiError>,
            GetObjectHashCode: None, //Option<fn(env: *mut jvmtiEnv, object: jobject, hash_code_ptr: *mut jint) -> jvmtiError>,
            GetObjectMonitorUsage: None, //Option<fn(env: *mut jvmtiEnv, object: jobject, info_ptr: *mut jvmtiMonitorUsage) -> jvmtiError>,
            GetFieldName: None, //Option<fn(env: *mut jvmtiEnv, klass: jclass, field: jfieldID, name_ptr: *mut *mut c_char, signature_ptr: *mut *mut c_char, generic_ptr: *mut *mut c_char) -> jvmtiError>,
            GetFieldDeclaringClass: None, //Option<fn(env: *mut jvmtiEnv, klass: jclass, field: jfieldID, declaring_class_ptr: *mut jclass) -> jvmtiError>,
            GetFieldModifiers: None, //Option<fn(env: *mut jvmtiEnv, klass: jclass, field: jfieldID, modifiers_ptr: *mut jint) -> jvmtiError>,
            IsFieldSynthetic: None, //Option<fn(env: *mut jvmtiEnv, klass: jclass, field: jfieldID, is_synthetic_ptr: *mut jboolean) -> jvmtiError>,
            GetMethodName: None, //Option<fn(env: *mut jvmtiEnv, method: jmethodID, name_ptr: *mut *mut c_char, signature_ptr: *mut *mut c_char, generic_ptr: *mut *mut c_char) -> jvmtiError>,
            GetMethodDeclaringClass: None, //Option<fn(env: *mut jvmtiEnv, method: jmethodID, declaring_class_ptr: *mut jclass) -> jvmtiError>,
            GetMethodModifiers: None, //Option<fn(env: *mut jvmtiEnv, method: jmethodID, modifiers_ptr: *mut jint) -> jvmtiError>,
            reserved67: ptr::null_mut(), //*mut c_void,
            GetMaxLocals: None, //Option<fn(env: *mut jvmtiEnv, method: jmethodID, max_ptr: *mut jint) -> jvmtiError>,
            GetArgumentsSize: None, //Option<fn(env: *mut jvmtiEnv, method: jmethodID, size_ptr: *mut jint) -> jvmtiError>,
            GetLineNumberTable: None, //Option<fn(env: *mut jvmtiEnv, method: jmethodID, entry_count_ptr: *mut jint, table_ptr: *mut *mut jvmtiLineNumberEntry) -> jvmtiError>,
            GetMethodLocation: None, //Option<fn(env: *mut jvmtiEnv, method: jmethodID, start_location_ptr: *mut jlocation, end_location_ptr: *mut jlocation) -> jvmtiError>,
            GetLocalVariableTable: None, //Option<fn(env: *mut jvmtiEnv, method: jmethodID, entry_count_ptr: *mut jint, table_ptr: *mut *mut jvmtiLocalVariableEntry) -> jvmtiError>,
            SetNativeMethodPrefix: None, //Option<fn(env: *mut jvmtiEnv, prefix: *const c_char) -> jvmtiError>,
            SetNativeMethodPrefixes: None, //Option<fn(env: *mut jvmtiEnv, prefix_count: jint, prefixes: *mut *mut c_char) -> jvmtiError>,
            GetBytecodes: None, //Option<fn(env: *mut jvmtiEnv, method: jmethodID, bytecode_count_ptr: *mut jint, bytecodes_ptr: *mut *mut c_uchar) -> jvmtiError>,
            IsMethodNative: None, //Option<fn(env: *mut jvmtiEnv, method: jmethodID, is_native_ptr: *mut jboolean) -> jvmtiError>,
            IsMethodSynthetic: None, //Option<fn(env: *mut jvmtiEnv, method: jmethodID, is_synthetic_ptr: *mut jboolean) -> jvmtiError>,
            GetLoadedClasses: None, //Option<fn(env: *mut jvmtiEnv, class_count_ptr: *mut jint, classes_ptr: *mut *mut jclass) -> jvmtiError>,
            GetClassLoaderClasses: None, //Option<fn(env: *mut jvmtiEnv, initiating_loader: jobject, class_count_ptr: *mut jint, classes_ptr: *mut *mut jclass) -> jvmtiError>,
            PopFrame: None, //Option<fn(env: *mut jvmtiEnv, thread: jthread) -> jvmtiError>,
            ForceEarlyReturnObject: None, //Option<fn(env: *mut jvmtiEnv, thread: jthread, value: jobject) -> jvmtiError>,
            ForceEarlyReturnInt: None, //Option<fn(env: *mut jvmtiEnv, thread: jthread, value: jint) -> jvmtiError>,
            ForceEarlyReturnLong: None, //Option<fn(env: *mut jvmtiEnv, thread: jthread, value: jlong) -> jvmtiError>,
            ForceEarlyReturnFloat: None, //Option<fn(env: *mut jvmtiEnv, thread: jthread, value: jfloat) -> jvmtiError>,
            ForceEarlyReturnDouble: None, //Option<fn(env: *mut jvmtiEnv, thread: jthread, value: jdouble) -> jvmtiError>,
            ForceEarlyReturnVoid: None, //Option<fn(env: *mut jvmtiEnv, thread: jthread) -> jvmtiError>,
            RedefineClasses: None, //Option<fn(env: *mut jvmtiEnv, class_count: jint, class_definitions: *const jvmtiClassDefinition) -> jvmtiError>,
            GetVersionNumber: Some(JVMTIEmulator::get_version_number), //Option<fn(env: *mut jvmtiEnv, version_ptr: *mut jint) -> jvmtiError>,
            GetCapabilities: None, //Option<fn(env: *mut jvmtiEnv, capabilities_ptr: *mut jvmtiCapabilities) -> jvmtiError>,
            GetSourceDebugExtension: None, //Option<fn(env: *mut jvmtiEnv, klass: jclass, source_debug_extension_ptr: *mut *mut c_char) -> jvmtiError>,
            IsMethodObsolete: None, //Option<fn(env: *mut jvmtiEnv, method: jmethodID, is_obsolete_ptr: *mut jboolean) -> jvmtiError>,
            SuspendThreadList: None, //Option<fn(env: *mut jvmtiEnv, request_count: jint, request_list: *const jthread, results: *mut jvmtiError) -> jvmtiError>,
            ResumeThreadList: None, //Option<fn(env: *mut jvmtiEnv, request_count: jint, request_list: *const jthread, results: *mut jvmtiError) -> jvmtiError>,
            reserved94: ptr::null_mut(), //*mut c_void,
            reserved95: ptr::null_mut(), //*mut c_void,
            reserved96: ptr::null_mut(), //*mut c_void,
            reserved97: ptr::null_mut(), //*mut c_void,
            reserved98: ptr::null_mut(), //*mut c_void,
            reserved99: ptr::null_mut(), //*mut c_void,
            GetAllStackTraces: None, //Option<fn(env: *mut jvmtiEnv, max_frame_count: jint, stack_info_ptr: *mut *mut jvmtiStackInfo, thread_count_ptr: *mut jint) -> jvmtiError>,
            GetThreadListStackTraces: None, //Option<fn(env: *mut jvmtiEnv, thread_count: jint, thread_list: *const jthread, max_frame_count: jint, stack_info_ptr: *mut *mut jvmtiStackInfo) -> jvmtiError>,
            GetThreadLocalStorage: None, //Option<fn(env: *mut jvmtiEnv, thread: jthread, data_ptr: *mut *mut c_void) -> jvmtiError>,
            SetThreadLocalStorage: None, //Option<fn(env: *mut jvmtiEnv, thread: jthread, data: *const c_void) -> jvmtiError>,
            GetStackTrace: None, //Option<fn(env: *mut jvmtiEnv, thread: jthread, start_depth: jint, max_frame_count: jint, frame_buffer: *mut jvmtiFrameInfo, count_ptr: *mut jint) -> jvmtiError>,
            reserved105: ptr::null_mut(), //*mut c_void,
            GetTag: None, //Option<fn(env: *mut jvmtiEnv, object: jobject, tag_ptr: *mut jlong) -> jvmtiError>,
            SetTag: None, //Option<fn(env: *mut jvmtiEnv, object: jobject, tag: jlong) -> jvmtiError>,
            ForceGarbageCollection: None, //Option<fn(env: *mut jvmtiEnv) -> jvmtiError>,
            IterateOverObjectsReachableFromObject: None, //Option<fn(env: *mut jvmtiEnv, object: jobject, object_reference_callback: jvmtiObjectReferenceCallback, user_data: *const c_void) -> jvmtiError>,
            IterateOverReachableObjects: None, //Option<fn(env: *mut jvmtiEnv, heap_root_callback: jvmtiHeapRootCallback, stack_ref_callback: jvmtiStackReferenceCallback, object_ref_callback: jvmtiObjectReferenceCallback, user_data: *const c_void) -> jvmtiError>,
            IterateOverHeap: None, //Option<fn(env: *mut jvmtiEnv, object_filter: jvmtiHeapObjectFilter, heap_object_callback: jvmtiHeapObjectCallback, user_data: *const c_void) -> jvmtiError>,
            IterateOverInstancesOfClass: None, //Option<fn(env: *mut jvmtiEnv, klass: jclass, object_filter: jvmtiHeapObjectFilter, heap_object_callback: jvmtiHeapObjectCallback, user_data: *const c_void) -> jvmtiError>,
            reserved113: ptr::null_mut(), //*mut c_void,
            GetObjectsWithTags: None, //Option<fn(env: *mut jvmtiEnv, tag_count: jint, tags: *const jlong, count_ptr: *mut jint, object_result_ptr: *mut *mut jobject, tag_result_ptr: *mut *mut jlong) -> jvmtiError>,
            FollowReferences: None, //Option<fn(env: *mut jvmtiEnv, heap_filter: jint, klass: jclass, initial_object: jobject, callbacks: *const jvmtiHeapCallbacks, user_data: *const c_void) -> jvmtiError>,
            IterateThroughHeap: None, //Option<fn(env: *mut jvmtiEnv, heap_filter: jint, klass: jclass, callbacks: *const jvmtiHeapCallbacks, user_data: *const c_void) -> jvmtiError>,
            reserved117: ptr::null_mut(), //*mut c_void,
            reserved118: ptr::null_mut(), //*mut c_void,
            reserved119: ptr::null_mut(), //*mut c_void,
            SetJNIFunctionTable: None, //Option<fn(env: *mut jvmtiEnv, function_table: *const jniNativeInterface) -> jvmtiError>,
            GetJNIFunctionTable: None, //Option<fn(env: *mut jvmtiEnv, function_table: *mut *mut jniNativeInterface) -> jvmtiError>,
            SetEventCallbacks: None, //Option<fn(env: *mut jvmtiEnv, callbacks: *const jvmtiEventCallbacks, size_of_callbacks: jint) -> jvmtiError>,
            GenerateEvents: None, //Option<fn(env: *mut jvmtiEnv, event_type: jvmtiEvent) -> jvmtiError>,
            GetExtensionFunctions: None, //Option<fn(env: *mut jvmtiEnv, extension_count_ptr: *mut jint, extensions: *mut *mut jvmtiExtensionFunctionInfo) -> jvmtiError>,
            GetExtensionEvents: None, //Option<fn(env: *mut jvmtiEnv, extension_count_ptr: *mut jint, extensions: *mut *mut jvmtiExtensionEventInfo) -> jvmtiError>,
            SetExtensionEventCallback: None, //Option<fn(env: *mut jvmtiEnv, extension_event_index: jint, callback: jvmtiExtensionEvent) -> jvmtiError>,
            DisposeEnvironment: None, //Option<fn(env: *mut jvmtiEnv) -> jvmtiError>,
            GetErrorName: None, //Option<fn(env: *mut jvmtiEnv, error: jvmtiError, name_ptr: *mut *mut c_char) -> jvmtiError>,
            GetJLocationFormat: None, //Option<fn(env: *mut jvmtiEnv, format_ptr: *mut jvmtiJlocationFormat) -> jvmtiError>,
            GetSystemProperties: None, //Option<fn(env: *mut jvmtiEnv, count_ptr: *mut jint, property_ptr: *mut *mut *mut c_char) -> jvmtiError>,
            GetSystemProperty: None, //Option<fn(env: *mut jvmtiEnv, property: *const c_char, value_ptr: *mut *mut c_char) -> jvmtiError>,
            SetSystemProperty: None, //Option<fn(env: *mut jvmtiEnv, property: *const c_char, value: *const c_char) -> jvmtiError>,
            GetPhase: None, //Option<fn(env: *mut jvmtiEnv, phase_ptr: *mut jvmtiPhase) -> jvmtiError>,
            GetCurrentThreadCpuTimerInfo: None, //Option<fn(env: *mut jvmtiEnv, info_ptr: *mut jvmtiTimerInfo) -> jvmtiError>,
            GetCurrentThreadCpuTime: None, //Option<fn(env: *mut jvmtiEnv, nanos_ptr: *mut jlong) -> jvmtiError>,
            GetThreadCpuTimerInfo: None, //Option<fn(env: *mut jvmtiEnv, info_ptr: *mut jvmtiTimerInfo) -> jvmtiError>,
            GetThreadCpuTime: None, //Option<fn(env: *mut jvmtiEnv, thread: jthread, nanos_ptr: *mut jlong) -> jvmtiError>,
            GetTimerInfo: None, //Option<fn(env: *mut jvmtiEnv, info_ptr: *mut jvmtiTimerInfo) -> jvmtiError>,
            GetTime: None, //Option<fn(env: *mut jvmtiEnv, nanos_ptr: *mut jlong) -> jvmtiError>,
            GetPotentialCapabilities: None, //Option<fn(env: *mut jvmtiEnv, capabilities_ptr: *mut jvmtiCapabilities) -> jvmtiError>,
            reserved141: ptr::null_mut(),
            AddCapabilities: None, //Option<fn(env: *mut jvmtiEnv, capabilities_ptr: *const jvmtiCapabilities) -> jvmtiError>,
            RelinquishCapabilities: None, //Option<fn(env: *mut jvmtiEnv, capabilities_ptr: *const jvmtiCapabilities) -> jvmtiError>,
            GetAvailableProcessors: None, //Option<fn(env: *mut jvmtiEnv, processor_count_ptr: *mut jint) -> jvmtiError>,
            GetClassVersionNumbers: None, //Option<fn(env: *mut jvmtiEnv, klass: jclass, minor_version_ptr: *mut jint, major_version_ptr: *mut jint) -> jvmtiError>,
            GetConstantPool: None, //Option<fn(env: *mut jvmtiEnv, klass: jclass, constant_pool_count_ptr: *mut jint, constant_pool_byte_count_ptr: *mut jint, constant_pool_bytes_ptr: *mut *mut c_uchar) -> jvmtiError>,
            GetEnvironmentLocalStorage: None, //Option<fn(env: *mut jvmtiEnv, data_ptr: *mut *mut c_void) -> jvmtiError>,
            SetEnvironmentLocalStorage: None, //Option<fn(env: *mut jvmtiEnv, data: *const c_void) -> jvmtiError>,
            AddToBootstrapClassLoaderSearch: None, //Option<fn(env: *mut jvmtiEnv, segment: *const c_char) -> jvmtiError>,
            SetVerboseFlag: None, //Option<fn(env: *mut jvmtiEnv, flag: jvmtiVerboseFlag, value: jboolean) -> jvmtiError>,
            AddToSystemClassLoaderSearch: None, //Option<fn(env: *mut jvmtiEnv, segment: *const c_char) -> jvmtiError>,
            RetransformClasses: None, //Option<fn(env: *mut jvmtiEnv, class_count: jint, classes: *const jclass) -> jvmtiError>,
            GetOwnedMonitorStackDepthInfo: None, //Option<fn(env: *mut jvmtiEnv, thread: jthread, monitor_info_count_ptr: *mut jint, monitor_info_ptr: *mut *mut jvmtiMonitorStackDepthInfo) -> jvmtiError>,
            GetObjectSize: None,//Option<fn(env: *mut jvmtiEnv, object: jobject, size_ptr: *mut jlong) -> jvmtiError>,
            GetLocalInstance: None //Option<fn(env: *mut jvmtiEnv, thread: jthread, depth: jint, value_ptr: *mut jobject) -> jvmtiError>,
        }
    }
}

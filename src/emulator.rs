use super::native::*;
use super::native::jvmti_native::*;
use libc::c_void;
use std::mem::transmute;
use std::ptr;
///
/// Allows emulating a Java Virtual Machine.
///
#[allow(non_snake_case)]
pub struct Emulator {

    pub reserved0: *mut c_void,
    pub reserved1: *mut c_void,
    pub reserved2: *mut c_void,

    pub DestroyJavaVM: Option<fn(vm: JavaVMPtr) -> jint>,
    pub AttachCurrentThread: Option<fn(vm: JavaVMPtr, penv: *mut *mut c_void, args: *mut c_void) -> jint>,
    pub DetachCurrentThread: Option<fn(vm: JavaVMPtr) -> jint>,
    pub GetEnv: Option<fn(vm: JavaVMPtr, penv: *mut *mut c_void, version: jint) -> jint>,
    pub AttachCurrentThreadAsDaemon: Option<fn(vm: JavaVMPtr, penv: *mut *mut c_void, args: *mut c_void) -> jint>,
    pub jvmti_env: JVMTIEmulator
}

impl Emulator {

    /// Create a new `Emulator` instance
    pub fn new() -> Emulator {
        let jvmti = JVMTIEmulator { id: 0xCAFE };

        let emulator = Emulator {
            reserved0: ptr::null_mut(),
            reserved1: ptr::null_mut(),
            reserved2: ptr::null_mut(),
            DestroyJavaVM: None,
            AttachCurrentThread: None,
            DetachCurrentThread: None,
            GetEnv: Some(Emulator::get_emulated_environment),
            AttachCurrentThreadAsDaemon: None,
            jvmti_env: jvmti
        };

        return emulator;
    }

    pub fn transmute(emulator: *mut *mut Emulator) -> *mut *const JNIInvokeInterface {
        unsafe { transmute(emulator) }
    }

    #[allow(unused_variables)]
    pub fn get_emulated_environment(vm: JavaVMPtr, penv: *mut *mut c_void, version: jint) -> jint {
        Emulator::backmute(vm).jvmti_env.id
    }

    fn backmute<'a>(vm: JavaVMPtr) -> &'a Emulator {
        unsafe {
            let emu_ptr: *mut *mut Emulator = transmute(vm);
            return &(**emu_ptr);
        }
    }
}

/// Emulates the JVM TI API for internal testing purposes
pub struct JVMTIEmulator {
    pub id: i32
}

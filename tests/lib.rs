extern crate jvmti;
extern crate libc;

pub use jvmti::agent::Agent;
pub use jvmti::wrapper::thread::Thread;
pub use jvmti::wrapper::native::jvmti_native::*;

pub mod util;

mod app;

#[test]
fn test_test() {
    assert_eq!(true, true);
}

pub struct JVMTITestbed {
     pub DestroyJavaVM: Option<fn(vm: *mut JavaVM) -> jint>,
     pub AttachCurrentThread: Option<fn(vm: *mut JavaVM, penv: *mut *mut ::libc::c_void, args: *mut ::libc::c_void) -> jint>,
     pub DetachCurrentThread: Option<fn(vm: *mut JavaVM) -> jint>,
     pub GetEnv: Option<fn(vm: *mut JavaVM, penv: *mut *mut ::libc::c_void, version: jint) -> jint>,
     pub AttachCurrentThreadAsDaemon: Option<fn(vm: *mut JavaVM, penv: *mut *mut ::libc::c_void, args: *mut ::libc::c_void) -> jint>,
}

impl JVMTITestbed {

    pub fn new() -> JVMTITestbed {
        JVMTITestbed {
            DestroyJavaVM: None,
            AttachCurrentThread: None,
            DetachCurrentThread: None,
            GetEnv: Some(GetEnv),
            AttachCurrentThreadAsDaemon: None
        }
     }

}


fn GetEnv(vm: *mut JavaVM, penv: *mut *mut ::libc::c_void, version: jint) -> jint {
    println!("CALLED");
    0 as jint
}

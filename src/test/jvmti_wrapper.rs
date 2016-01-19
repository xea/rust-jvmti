extern crate libc;

mod jvmti_native;

use jvmti_native::jvmti_native::JavaVM;

pub type Wrapper = u32;

#[no_mangle]
#[allow(non_snake_case)]
pub extern fn Agent_OnLoad(vm: *mut JavaVM, options: *mut ::libc::c_char, reserved: *mut ::libc::c_void) -> jint {
    0;
}


/*
extern crate libc;

mod jvmti_native;
mod jvmti;

use jvmti_native::jvmti_native::*;
use libc::c_void;
use libc::c_uint;
use std::ptr;
use std::mem::size_of;

#[link(name = "clib")] //, kind = "static")]
extern "C" {
    fn test_clib() -> c_uint;
    fn fwd_enable_capabilities(envPtr: *mut jvmtiEnv) -> c_uint;
    fn fwd_enable_notifications(envPtr: *mut jvmtiEnv) -> c_uint;
    fn cbVMObjectAlloc(jvmti_env: *mut jvmtiEnv, jni_env: *mut JNIEnv, thread: jthread, object: jobject, object_klass: jclass, size: jlong) -> ();
    fn print_callbacks(callbacks: *mut jvmtiEventCallbacks) -> ();
}

#[no_mangle]
#[allow(non_snake_case)]
pub extern fn Agent_OnLoad(vm: *mut JavaVM, options: *mut ::libc::c_char, reserved: *mut ::libc::c_void) -> jint {

    unsafe {
        println!("R: Agent OnLoad(vm: {:p})", vm);
//        println!("Testing C library (Should be 14): {}", test_clib());
        // ------

        let mut voidPtr = ptr::null_mut() as *mut c_void;
        let ptrBox = Box::from_raw(&mut voidPtr);

        (**vm).GetEnv.unwrap()(vm, Box::into_raw(ptrBox), JVMTI_VERSION);
        println!("Env ptr {:p} -> {:p}", &voidPtr, voidPtr);

        //let result = fwd_enable_capabilities(voidPtr as *mut jvmtiEnv);
        let mut result = local_enable_capabilities(voidPtr as *mut jvmtiEnv);

        if result == 0 {
            println!("YAY");
        } else {
            println!("Boo: {}", result);
        }

        // ----------------------------

//        result = fwd_enable_notifications(voidPtr as *mut jvmtiEnv);

        if result == 0 {
            println!("YAY");
        } else {
            println!("BOO {}", result);
        }

        result = local_enable_notifications(voidPtr as *mut jvmtiEnv);

        if result == 0 {
            println!("YAY");
        } else {
            println!("BOO {}", result);
        }
    }

    return 0;
}

fn local_enable_capabilities(envPtr: *mut jvmtiEnv) -> u32 {
    unsafe {
        let caps = jvmtiCapabilities {
            _bindgen_bitfield_1_: 0xffffffff,
            _bindgen_bitfield_2_: 0x000000ff,
            _bindgen_bitfield_3_: 0x0,
            _bindgen_bitfield_4_: 0
        };
        let result = (**envPtr).AddCapabilities.unwrap()(envPtr, &caps);

        return result;
    }
}

fn local_enable_notifications(envPtr: *mut jvmtiEnv) -> u32 {
    unsafe {
        let mut callbacks = jvmtiEventCallbacks {
            ..Default::default()
        };

//        println!("RUST CALLBACK {:p}", cbVMObjectAlloc);
        callbacks.VMObjectAlloc = Some(local_callback_object_alloc);

        let mut pbox = Box::new(callbacks);

        let result = (**envPtr).SetEventCallbacks.unwrap()(envPtr, &callbacks, size_of::<jvmtiEventCallbacks>() as i32);
//        print_callbacks(&mut *pbox);

        let sptr: *mut Struct__jobject = ptr::null_mut();
        let mut a = 0;
        a = (**envPtr).SetEventNotificationMode.unwrap()(envPtr, JVMTI_ENABLE, JVMTI_EVENT_VM_START, sptr);
        a = (**envPtr).SetEventNotificationMode.unwrap()(envPtr, JVMTI_ENABLE, JVMTI_EVENT_VM_OBJECT_ALLOC, sptr);

        println!("Notification result: {}", a);
        return a;
    }
}

unsafe extern "C" fn local_callback_object_alloc(jvmti_env: *mut jvmtiEnv, jni_env: *mut JNIEnv, thread: jthread, object: jobject, object_klass: jclass, size: jlong) -> () {
    println!("ALLOK");
}

*/

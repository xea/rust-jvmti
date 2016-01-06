extern crate libc;

mod jvmti_native;

use jvmti_native::jvmti_native::*;
use libc::c_void;
use libc::c_uint;
use std::ptr;

#[no_mangle]
pub extern fn test_call(callback: extern "C" fn() -> c_uint) -> c_uint {
    println!("RUST: callback address {:p}", callback);
    return callback();
}

/**
 *
 */
#[no_mangle]
#[allow(non_snake_case)]
pub extern fn Agent_OnLoad(vm: *mut JavaVM, options: *mut ::libc::c_char, reserved: *mut ::libc::c_void) -> jint {
    println!("RUST Load (vmptr: {:p} opts {:p} reserved {:p})", vm, options, reserved);

    unsafe {
        // TODO consider Box()-ing this, if possible
        let mut voidPtr: *mut c_void = ptr::null_mut() as *mut c_void;
        let mut penvPtr: *mut *mut c_void = &mut voidPtr as *mut *mut c_void; // <- this has been verified as correct conversion
        println!("RUST Fptr {:p}", vm);
        let fnp = (**vm).GetEnv.unwrap();
        let dnp = (**vm).DestroyJavaVM.unwrap();
        println!("FNP {:p}", fnp);
        println!("DNP {:p}", dnp);
        println!("PENVPTR {:p} {:p}", penvPtr, *penvPtr);
        let result = fnp(vm, penvPtr, JVMTI_VERSION);
        println!("Maybe? {:p} {:p}", penvPtr, *penvPtr);

        if result == 0 {
            // --------
            let envPtr: *mut jvmtiEnv = *penvPtr as *mut jvmtiEnv;
            println!("Got environment ptr: {:p} / {:p}", envPtr, *envPtr);
            jvmti::on_init(*envPtr);
        }
    }

    return jvmti::RESULT_OK;
}

#[no_mangle]
#[allow(non_snake_case)]
pub extern fn Agent_OnUnload(vm: *mut JavaVM) -> () {
    println!("RUST Unload vmptr: {:p}", vm);
}

#[allow(unused_variables)]
#[allow(non_snake_case)]
pub mod jvmti {
    use libc::c_uint;
    use std::ptr;
    use jvmti_native::jvmti_native::*;
    use std::collections::HashMap;

    pub const RESULT_OK: jint = 0;

    pub fn on_init(env: jvmtiEnv) -> () {
        println!("on_init");

        let caps = AgentCapabilities {
            // TODO this capability setting is here for testing purposes only
            can_get_line_numbers: true,
            ..Default::default()
        };

        let env = Environment::new(env);
        env.add_capabilities(caps);

        let mut callbacks = EventCallbacks { ..Default::default() };
        callbacks.on_vm_init(|| 13 as i64);
        env.set_event_callbacks(callbacks);

        env.set_event_notification_mode();
    }

    pub struct Environment {
        nativeEnv: jvmtiEnv
    }

    impl Environment {
        fn new(nativeEnv: jvmtiEnv) -> Environment {
            Environment {
                nativeEnv: nativeEnv
            }
        }

        fn add_capabilities(&self, caps: AgentCapabilities) -> () {
            unsafe {
                let pbox = Box::new(self.nativeEnv);
                let result = (*self.nativeEnv).AddCapabilities.unwrap()(Box::into_raw(pbox), &caps.to_native());

                if result != JVMTI_ERROR_NONE {
                    println!("Add capabilities result: {}", error_code(result));
                }
            }
        }

        fn set_event_callbacks(&self, callbacks: EventCallbacks) -> () {
            unsafe {
                let pbox = Box::new(self.nativeEnv);
                let result: c_uint = (*self.nativeEnv).SetEventCallbacks.unwrap()(Box::into_raw(pbox), &callbacks.to_native(), 1);

                if result != JVMTI_ERROR_NONE {
                    println!("Result: {}", result);
                }
            }
        }

        fn set_event_notification_mode(&self) -> () {
            unsafe {
                let pbox = Box::new(self.nativeEnv);
                (*self.nativeEnv).SetEventNotificationMode.unwrap()(Box::into_raw(pbox), JVMTI_ENABLE, JVMTI_EVENT_VM_INIT, ptr::null_mut());
            }
        }
    }

    #[derive(Default)]
    pub struct AgentCapabilities {
        can_tag_objects: bool,
        can_generate_field_modification_events: bool,
        can_generate_field_access_events: bool,
        can_get_bytecodes: bool,
        can_get_synthetic_attribute: bool,
        can_get_owned_monitor_info: bool,
        can_get_current_contended_monitor: bool,
        can_get_monitor_info: bool,
        can_pop_frame: bool,
        can_redefine_classes: bool,
        can_signal_thread: bool,
        can_get_source_file_name: bool,
        can_get_line_numbers: bool,
        can_get_source_debug_extension: bool,
        can_access_local_variables: bool,
        can_maintain_original_method_order: bool,
        can_generate_single_step_events: bool,
        can_generate_exception_events: bool,
        can_generate_frame_pop_events: bool,
        can_generate_breakpoint_events: bool,
        can_suspend: bool,
        can_redefine_any_class: bool,
        can_get_current_thread_cpu_time: bool,
        can_get_thread_cpu_time: bool,
        can_generate_method_entry_events: bool,
        can_generate_method_exit_events: bool,
        can_generate_all_class_hook_events: bool,
        can_generate_compiled_method_load_events: bool,
        can_generate_monitor_events: bool,
        can_generate_vm_object_alloc_events: bool,
        can_generate_native_method_bind_events: bool,
        can_generate_garbage_collection_events: bool,
        can_generate_object_free_events: bool,
        can_force_early_return: bool,
        can_get_owned_monitor_stack_depth_info: bool,
        can_get_constant_pool: bool,
        can_set_native_method_prefix: bool,
        can_retransform_classes: bool,
        can_retransform_any_class: bool,
        can_generate_resource_exhaustion_heap_events: bool,
        can_generate_resource_exhaustion_threads_events: bool
    }

    impl AgentCapabilities {

        pub fn to_native(&self) -> jvmtiCapabilities {
            let mut field_map1 = HashMap::new();
            let mut field_map2 = HashMap::new();
            let field_map3 = HashMap::new();
            let field_map4 = HashMap::new();

            // TODO this is probably not idiomatic Rust but this is the best I could come up with at them moment
            field_map1.insert(0x00000001, self.can_tag_objects);
            field_map1.insert(0x00000002, self.can_generate_field_modification_events);
            field_map1.insert(0x00000004, self.can_generate_field_access_events);
            field_map1.insert(0x00000008, self.can_get_bytecodes);
            field_map1.insert(0x00000010, self.can_get_synthetic_attribute);
            field_map1.insert(0x00000020, self.can_get_owned_monitor_info);
            field_map1.insert(0x00000040, self.can_get_current_contended_monitor);
            field_map1.insert(0x00000080, self.can_get_monitor_info);
            field_map1.insert(0x00000100, self.can_pop_frame);
            field_map1.insert(0x00000200, self.can_redefine_classes);
            field_map1.insert(0x00000400, self.can_signal_thread);
            field_map1.insert(0x00000800, self.can_get_source_file_name);
            field_map1.insert(0x00001000, self.can_get_line_numbers);
            field_map1.insert(0x00002000, self.can_get_source_debug_extension);
            field_map1.insert(0x00004000, self.can_access_local_variables);
            field_map1.insert(0x00008000, self.can_maintain_original_method_order);
            field_map1.insert(0x00010000, self.can_generate_single_step_events);
            field_map1.insert(0x00020000, self.can_generate_exception_events);
            field_map1.insert(0x00040000, self.can_generate_frame_pop_events);
            field_map1.insert(0x00080000, self.can_generate_breakpoint_events);
            field_map1.insert(0x00100000, self.can_suspend);
            field_map1.insert(0x00200000, self.can_redefine_any_class);
            field_map1.insert(0x00400000, self.can_get_current_thread_cpu_time);
            field_map1.insert(0x00800000, self.can_get_thread_cpu_time);
            field_map1.insert(0x01000000, self.can_generate_method_entry_events);
            field_map1.insert(0x02000000, self.can_generate_method_exit_events);
            field_map1.insert(0x04000000, self.can_generate_all_class_hook_events);
            field_map1.insert(0x08000000, self.can_generate_compiled_method_load_events);
            field_map1.insert(0x10000000, self.can_generate_monitor_events);
            field_map1.insert(0x20000000, self.can_generate_vm_object_alloc_events);
            field_map1.insert(0x40000000, self.can_generate_native_method_bind_events);
            field_map1.insert(0x80000000, self.can_generate_garbage_collection_events);

            field_map2.insert(0x00000001, self.can_generate_object_free_events);
            field_map2.insert(0x00000002, self.can_force_early_return);
            field_map2.insert(0x00000004, self.can_get_owned_monitor_stack_depth_info);
            field_map2.insert(0x00000008, self.can_get_constant_pool);
            field_map2.insert(0x00000010, self.can_set_native_method_prefix);
            field_map2.insert(0x00000020, self.can_retransform_classes);
            field_map2.insert(0x00000040, self.can_retransform_any_class);
            field_map2.insert(0x00000080, self.can_generate_resource_exhaustion_heap_events);
            field_map2.insert(0x00000100, self.can_generate_resource_exhaustion_threads_events);

            let fields = vec![ field_map1, field_map2, field_map3, field_map4 ];
            let result:Vec<u32> = fields.iter().map(|f| f.iter().map(|(&value, &switch)| if switch { value } else { 0 }).fold(0, |acc, item| acc | item) ).collect();

            /*
            println!("{}", result[0]);
            println!("{}", result[1]);
            println!("{}", result[2]);
            println!("{}", result[3]);
            */

            let nativeStruct = jvmtiCapabilities {
                _bindgen_bitfield_1_: result[0],
                _bindgen_bitfield_2_: result[1],
                _bindgen_bitfield_3_: result[2],
                _bindgen_bitfield_4_: result[3]
            };

            return nativeStruct;
        }
    }

    #[derive(Default)]
    pub struct EventCallbacks {
        eventMap: jvmtiEventCallbacks
    }

    impl EventCallbacks {
        /*::std::option::Option<unsafe extern "C" fn(jvmti_env: *mut jvmtiEnv, jni_env: *mut JNIEnv, thread: jthread) -> ()>;*/
        pub fn on_vm_init<F>(&mut self, callback: F) -> () where F: Fn() -> i64 {
            let fn_ptr: unsafe extern "C" fn(jvmti_env: *mut jvmtiEnv, jni_env: *mut JNIEnv, thread: jthread) -> () = EventCallbacks::ftft_callback;
            println!("{:p}", fn_ptr);
            let vminit_callback: jvmtiEventVMInit = Some(fn_ptr);
            self.eventMap.VMInit = vminit_callback;
        }

        #[no_mangle]
        pub unsafe extern "C" fn ftft_callback(jvmti_env: *mut jvmtiEnv, jni_env: *mut JNIEnv, thread: jthread) -> () {
            println!("FTFTFT");
        }

        pub fn to_native(&self) -> jvmtiEventCallbacks {
            let callbacks = jvmtiEventCallbacks {
                ..Default::default()
            };

            return callbacks;
        }
    }

    pub fn error_code(code: c_uint) -> String {
        match code {
            0 => "No error has occurred.",
            100 => "Pointer is unexpectedly NULL.",
            110 => "The function attempted to allocate memory and no more memory was available for allocation.",
            111 => "The desired functionality has not been enabled in this virtual machine.",
            112 => "The desired functionality is not available in the current phase. Always returned if the virtual machine has completed running.",
            113 => "An unexpected internal error has occurred.",
            115 => "The thread being used to call this function is not attached to the virtual machine. Calls must be made from attached threads.",
            116 => "The JVM TI environment provided is no longer connected or is not an environment.",
            _ => "Unknown error"
        }.to_string()
    }
}

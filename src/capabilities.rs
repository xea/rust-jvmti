use std::collections::HashMap;
use super::native::jvmti_native::*;

#[derive(Default, Clone)]
pub struct Capabilities {
    /// Can set and get tags
    pub can_tag_objects: bool,
    /// Can set watchpoints on field modification
    pub can_generate_field_modification_events: bool,
    /// Can set watchpoints on field access
    pub can_generate_field_access_events: bool,
    /// Can get bytecodes of a method
    pub can_get_bytecodes: bool,
    /// Can test if a field or method is synthetic
    pub can_get_synthetic_attribute: bool,
    /// Can get information about ownership of monitors
    pub can_get_owned_monitor_info: bool,
    /// Can GetCurrentContendedMonitor
    pub can_get_current_contended_monitor: bool,
    /// Can GetObjectMonitorUsage
    pub can_get_monitor_info: bool,
    /// Can pop frames off the stack
    pub can_pop_frame: bool,
    /// Can redefine classes with RedefineClasses
    pub can_redefine_classes: bool,
    /// Can send stop or interrupt to threads
    pub can_signal_thread: bool,
    /// Can get the source file name of a class
    pub can_get_source_file_name: bool,
    /// Can get the line number table of a method
    pub can_get_line_numbers: bool,
    /// Can get the source debug extension of a class
    pub can_get_source_debug_extension: bool,
    /// Can set and get local variables
    pub can_access_local_variables: bool,
    /// Can return methods in the order they occur in the class file
    pub can_maintain_original_method_order: bool,
    /// Can get single step events
    pub can_generate_single_step_events: bool,
    /// Can get exception thrown and exception catch events
    pub can_generate_exception_events: bool,
    /// Can set and thus get FramePop events
    pub can_generate_frame_pop_events: bool,
    /// Can set and thus get Breakpoint events
    pub can_generate_breakpoint_events: bool,
    /// Can suspend and resume threads
    pub can_suspend: bool,
    /// Can modify (retransform or redefine) any non-primitive non-array class.
    pub can_redefine_any_class: bool,
    /// Can get current thread CPU time
    pub can_get_current_thread_cpu_time: bool,
    /// Can get thread CPU time
    pub can_get_thread_cpu_time: bool,
    /// Can generate method entry events on entering a method
    pub can_generate_method_entry_events: bool,
    /// Can generate method exit events on leaving a method
    pub can_generate_method_exit_events: bool,
    /// Can generate ClassFileLoadHook events for every loaded class.
    pub can_generate_all_class_hook_events: bool,
    /// Can generate events when a method is compiled or unloaded
    pub can_generate_compiled_method_load_events: bool,
    /// Can generate events on monitor activity
    pub can_generate_monitor_events: bool,
    /// Can generate events on VM allocation of an object
    pub can_generate_vm_object_alloc_events: bool,
    /// Can generate events when a native method is bound to its implementation
    pub can_generate_native_method_bind_events: bool,
    /// Can generate events when garbage collection begins or ends
    pub can_generate_garbage_collection_events: bool,
    /// Can generate events when the garbage collector frees an object
    pub can_generate_object_free_events: bool,
    /// Can return early from a method
    pub can_force_early_return: bool,
    /// Can get information about owned monitors with stack depth
    pub can_get_owned_monitor_stack_depth_info: bool,
    /// Can get the constant pool of a class
    pub can_get_constant_pool: bool,
    /// Can set prefix to be applied when native method cannot be resolved
    pub can_set_native_method_prefix: bool,
    /// Can retransform classes with RetransformClasses. In addition to the restrictions imposed by the specific
    /// implementation on this capability (see the Capability section), this capability must be set before the
    /// ClassFileLoadHook event is enabled for the first time in this environment. An environment that possesses
    /// this capability at the time that ClassFileLoadHook is enabled for the first time is said to be
    /// retransformation capable. An environment that does not possess this capability at the time that
    /// ClassFileLoadHook is enabled for the first time is said to be retransformation incapable
    pub can_retransform_classes: bool,
    /// RetransformClasses can be called on any class (can_retransform_classes must also be set)
    pub can_retransform_any_class: bool,
    /// Can generate events when the VM is unable to allocate memory from the JavaTM platform heap.
    pub can_generate_resource_exhaustion_heap_events: bool,
    /// Can generate events when the VM is unable to create a thread.
    pub can_generate_resource_exhaustion_threads_events: bool
}

impl Capabilities {

    pub fn new() -> Capabilities {
        Capabilities {
            ..Default::default()
        }
    }

    pub fn from_native(native_caps: &jvmtiCapabilities) -> Capabilities {
        Capabilities {
            can_tag_objects:                            native_caps._bindgen_bitfield_1_ & 0x00000001 > 0,
            can_generate_field_modification_events:     native_caps._bindgen_bitfield_1_ & 0x00000002 > 0,
            can_generate_field_access_events:           native_caps._bindgen_bitfield_1_ & 0x00000004 > 0,
            can_get_bytecodes:                          native_caps._bindgen_bitfield_1_ & 0x00000008 > 0,
            can_get_synthetic_attribute:                native_caps._bindgen_bitfield_1_ & 0x00000010 > 0,
            can_get_owned_monitor_info:                 native_caps._bindgen_bitfield_1_ & 0x00000020 > 0,
            can_get_current_contended_monitor:          native_caps._bindgen_bitfield_1_ & 0x00000040 > 0,
            can_get_monitor_info:                       native_caps._bindgen_bitfield_1_ & 0x00000080 > 0,
            can_pop_frame:                              native_caps._bindgen_bitfield_1_ & 0x00000100 > 0,
            can_redefine_classes:                       native_caps._bindgen_bitfield_1_ & 0x00000200 > 0,
            can_signal_thread:                          native_caps._bindgen_bitfield_1_ & 0x00000400 > 0,
            can_get_source_file_name:                   native_caps._bindgen_bitfield_1_ & 0x00000800 > 0,
            can_get_line_numbers:                       native_caps._bindgen_bitfield_1_ & 0x00001000 > 0,
            can_get_source_debug_extension:             native_caps._bindgen_bitfield_1_ & 0x00002000 > 0,
            can_access_local_variables:                 native_caps._bindgen_bitfield_1_ & 0x00004000 > 0,
            can_maintain_original_method_order:         native_caps._bindgen_bitfield_1_ & 0x00008000 > 0,
            can_generate_single_step_events:            native_caps._bindgen_bitfield_1_ & 0x00010000 > 0,
            can_generate_exception_events:              native_caps._bindgen_bitfield_1_ & 0x00020000 > 0,
            can_generate_frame_pop_events:              native_caps._bindgen_bitfield_1_ & 0x00040000 > 0,
            can_generate_breakpoint_events:             native_caps._bindgen_bitfield_1_ & 0x00080000 > 0,
            can_suspend:                                native_caps._bindgen_bitfield_1_ & 0x00100000 > 0,
            can_redefine_any_class:                     native_caps._bindgen_bitfield_1_ & 0x00200000 > 0,
            can_get_current_thread_cpu_time:            native_caps._bindgen_bitfield_1_ & 0x00400000 > 0,
            can_get_thread_cpu_time:                    native_caps._bindgen_bitfield_1_ & 0x00800000 > 0,
            can_generate_method_entry_events:           native_caps._bindgen_bitfield_1_ & 0x01000000 > 0,
            can_generate_method_exit_events:            native_caps._bindgen_bitfield_1_ & 0x02000000 > 0,
            can_generate_all_class_hook_events:         native_caps._bindgen_bitfield_1_ & 0x04000000 > 0,
            can_generate_compiled_method_load_events:   native_caps._bindgen_bitfield_1_ & 0x08000000 > 0,
            can_generate_monitor_events:                native_caps._bindgen_bitfield_1_ & 0x10000000 > 0,
            can_generate_vm_object_alloc_events:        native_caps._bindgen_bitfield_1_ & 0x20000000 > 0,
            can_generate_native_method_bind_events:     native_caps._bindgen_bitfield_1_ & 0x40000000 > 0,
            can_generate_garbage_collection_events:     native_caps._bindgen_bitfield_1_ & 0x80000000 > 0,

            can_generate_object_free_events:            native_caps._bindgen_bitfield_2_ & 0x00000001 > 0,
            can_force_early_return:                     native_caps._bindgen_bitfield_2_ & 0x00000002 > 0,
            can_get_owned_monitor_stack_depth_info:     native_caps._bindgen_bitfield_2_ & 0x00000004 > 0,
            can_get_constant_pool:                      native_caps._bindgen_bitfield_2_ & 0x00000008 > 0,
            can_set_native_method_prefix:               native_caps._bindgen_bitfield_2_ & 0x00000010 > 0,
            can_retransform_classes:                    native_caps._bindgen_bitfield_2_ & 0x00000020 > 0,
            can_retransform_any_class:                  native_caps._bindgen_bitfield_2_ & 0x00000040 > 0,
            can_generate_resource_exhaustion_heap_events: native_caps._bindgen_bitfield_2_ & 0x00000080 > 0,
            can_generate_resource_exhaustion_threads_events: native_caps._bindgen_bitfield_2_ & 0x00000100 > 0,
        }
    }

    /// Convert this instance into a native jvmtiCapabilities instance that can be passwd to the
    /// native JVMTI interface
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

        let native_struct = jvmtiCapabilities {
            _bindgen_bitfield_1_: result[0],
            _bindgen_bitfield_2_: result[1],
            _bindgen_bitfield_3_: result[2],
            _bindgen_bitfield_4_: result[3]
        };

        return native_struct;
    }
}

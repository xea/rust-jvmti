use std::collections::HashMap;
use super::jvmti_native::jvmti_native::*;

#[derive(Default)]
pub struct AgentCapabilities {
    pub can_tag_objects: bool,
    pub can_generate_field_modification_events: bool,
    pub can_generate_field_access_events: bool,
    pub can_get_bytecodes: bool,
    pub can_get_synthetic_attribute: bool,
    pub can_get_owned_monitor_info: bool,
    pub can_get_current_contended_monitor: bool,
    pub can_get_monitor_info: bool,
    pub can_pop_frame: bool,
    pub can_redefine_classes: bool,
    pub can_signal_thread: bool,
    pub can_get_source_file_name: bool,
    pub can_get_line_numbers: bool,
    pub can_get_source_debug_extension: bool,
    pub can_access_local_variables: bool,
    pub can_maintain_original_method_order: bool,
    pub can_generate_single_step_events: bool,
    pub can_generate_exception_events: bool,
    pub can_generate_frame_pop_events: bool,
    pub can_generate_breakpoint_events: bool,
    pub can_suspend: bool,
    pub can_redefine_any_class: bool,
    pub can_get_current_thread_cpu_time: bool,
    pub can_get_thread_cpu_time: bool,
    pub can_generate_method_entry_events: bool,
    pub can_generate_method_exit_events: bool,
    pub can_generate_all_class_hook_events: bool,
    pub can_generate_compiled_method_load_events: bool,
    pub can_generate_monitor_events: bool,
    pub can_generate_vm_object_alloc_events: bool,
    pub can_generate_native_method_bind_events: bool,
    pub can_generate_garbage_collection_events: bool,
    pub can_generate_object_free_events: bool,
    pub can_force_early_return: bool,
    pub can_get_owned_monitor_stack_depth_info: bool,
    pub can_get_constant_pool: bool,
    pub can_set_native_method_prefix: bool,
    pub can_retransform_classes: bool,
    pub can_retransform_any_class: bool,
    pub can_generate_resource_exhaustion_heap_events: bool,
    pub can_generate_resource_exhaustion_threads_events: bool
}

impl AgentCapabilities {

    pub fn new() -> AgentCapabilities {
        AgentCapabilities {
            ..Default::default()
        }
    }

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

        let native_struct = jvmtiCapabilities {
            _bindgen_bitfield_1_: result[0],
            _bindgen_bitfield_2_: result[1],
            _bindgen_bitfield_3_: result[2],
            _bindgen_bitfield_4_: result[3]
        };

        return native_struct;
    }
}

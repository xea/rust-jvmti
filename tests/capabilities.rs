extern crate jvmti;

#[cfg(test)]
mod tests {

    use jvmti::capabilities::Capabilities;

    #[test]
    fn agent_capabilities_are_generated_with_capabilities_off() {
        let caps = Capabilities::new();
        assert_eq!(false, caps.can_pop_frame);
        assert_eq!(false, caps.can_redefine_classes);
        assert_eq!(false, caps.can_get_bytecodes);
        assert_eq!(false, caps.can_generate_monitor_events);
        assert_eq!(false, caps.can_generate_exception_events);
    }

    #[test]
    fn agent_capabilities_are_reflected_in_native_capabilities() {
        let mut caps = Capabilities::new();
        caps.can_pop_frame = true;

        let native_caps = caps.to_native();
        let recaps = Capabilities::from_native(&native_caps);

        assert_eq!(true, recaps.can_pop_frame);
        assert_eq!(false, recaps.can_redefine_classes);
    }

    #[test]
    fn merge_combines_enabled_flags_from_both_capabilities() {
        let mut caps1 = Capabilities::new();
        let mut caps2 = Capabilities::new();

        caps1.can_pop_frame = true;
        caps2.can_generate_monitor_events = true;

        let caps_result = caps1.merge(&caps2);

        assert_eq!(true, caps_result.can_pop_frame);
        assert_eq!(true, caps_result.can_generate_monitor_events);
    }
}

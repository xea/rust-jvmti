extern crate jvmti;

#[cfg(test)]
mod tests {

    use jvmti::event::EventCallbacks;

    #[test]
    fn empty_event_callbacks_are_instantiatable_using_new() {
        let ec = EventCallbacks::new();
        assert_eq!(None, ec.method_entry);
    }
}

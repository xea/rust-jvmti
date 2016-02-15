extern crate jvmti;


#[cfg(test)]
mod test {

    use jvmti::stringify;
    use jvmti::native::MutString;
    use std::ptr;

    #[test]
    fn stringify_returns_a_meaningful_value_on_null_ptr() {
        let tv: MutString = ptr::null_mut();
        let expected = "(NULL)".to_string();
        assert_eq!(expected, stringify(tv));
    }

    #[test]
    fn stringify_returns_the_stringified_content_if_its_a_valid_utf8_string() {
    }
}

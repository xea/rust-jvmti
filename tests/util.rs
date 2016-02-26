extern crate jvmti;

#[cfg(test)]
mod tests {

    use jvmti::util::stringify;
    use jvmti::native::MutString;
    use std::ptr;
    use std::ffi::CString;

    #[test]
    fn stringify_returns_a_meaningful_value_on_null_ptr() {
        let tv: MutString = ptr::null_mut();
        let expected = "(NULL)".to_string();
        assert_eq!(expected, stringify(tv));
    }

    #[test]
    fn stringify_returns_the_stringified_content_if_its_a_valid_utf8_string() {
        let expected = "test";
        let s: MutString = CString::new(expected).unwrap().as_ptr() as *mut i8;
        assert_eq!(expected, stringify(s));
    }

    #[test]
    fn stringify_returns_an_empty_string_if_the_input_was_an_empty_string() {
        let expected = "";
        let s: MutString = CString::new(expected).unwrap().as_ptr() as *mut i8;
        assert_eq!(expected, stringify(s));
    }
}

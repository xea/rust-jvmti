extern crate jvmti;


#[cfg(test)]
mod tests {
    use jvmti::instrumentation::JavaClass;

    #[test]
    fn can_create_empty_class() {
        let new_class = JavaClass::new();

        let classfile = new_class.to_classfile();

        assert_eq!(classfile, classfile);
    }
}

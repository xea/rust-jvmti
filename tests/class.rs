extern crate jvmti;

#[cfg(test)]
mod tests {

    use jvmti::class::{Class, ClassId, JavaType};
    use std::ptr;

    #[test]
    fn primitive_types_are_parsed_correctly() {
        assert_eq!(Some(JavaType::Void), JavaType::parse("V"));
        assert_eq!(Some(JavaType::Int), JavaType::parse("I"));
        assert_eq!(Some(JavaType::Boolean), JavaType::parse("Z"));
        assert_eq!(Some(JavaType::Short), JavaType::parse("S"));
        assert_eq!(Some(JavaType::Long), JavaType::parse("J"));
    }

    #[test]
    fn arrays_are_parsed_correctly() {
        assert_eq!(Some(JavaType::Array(Box::new(JavaType::Int))), JavaType::parse("[I"));
        assert_eq!(Some(JavaType::Array(Box::new(JavaType::Boolean))), JavaType::parse("[Z"));
        assert_eq!(Some(JavaType::Array(Box::new(JavaType::Array(Box::new(JavaType::Int))))), JavaType::parse("[[I"));
    }

    #[test]
    fn classes_are_parsed_correctly() {
        assert_eq!(Some(JavaType::Class("Lso/blacklight/Test;")), JavaType::parse("Lso/blacklight/Test;"));
        assert_eq!(Some(JavaType::Class("LTest;")), JavaType::parse("LTest;"));
    }

    #[test]
    fn arrays_of_classes_are_parsed() {
        assert_eq!(Some(JavaType::Array(Box::new(JavaType::Class("Lso/blacklight/Test;")))), JavaType::parse("[Lso/blacklight/Test;"));
    }

    #[test]
    fn java_types_are_stringified() {
        assert_eq!("void", JavaType::to_string(&JavaType::Void));
        assert_eq!("int", JavaType::to_string(&JavaType::Int));
        assert_eq!("int[]", JavaType::to_string(&JavaType::Array(Box::new(JavaType::Int))));
        assert_eq!("so.blacklight.Test", JavaType::to_string(&JavaType::Class("Lso/blacklight/Test;")));
        assert_eq!("so.blacklight.Test[]", JavaType::to_string(&JavaType::Array(Box::new(JavaType::Class("Lso/blacklight/Test;")))));
        assert_eq!("short[][]", JavaType::to_string(&JavaType::Array(Box::new(JavaType::Array(Box::new(JavaType::Short))))));
    }

    #[test]
    fn class_to_string_returns_the_fully_qualified_class_name() {
        assert_eq!("so.blacklight.Test", Class::new(ClassId { native_id: ptr::null_mut() }, JavaType::Class("Lso/blacklight/Test;")).to_string());
        assert_eq!("so.blacklight.Test$1", Class::new(ClassId { native_id: ptr::null_mut() }, JavaType::Class("Lso/blacklight/Test$1;")).to_string());
    }
}

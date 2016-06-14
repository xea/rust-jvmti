extern crate jvmti;

#[cfg(test)]
mod tests {

    mod constant_pool {

        use jvmti::bytecode::constant::*;

        #[test]
        fn get_idx_should_consider_long_entries() {
            let cp_empty = ConstantPool::from_vec(vec![]);
            assert_eq!(0, cp_empty.len());

            // Inputs starting with a placeholder should be unmodified
            let cp_ph = ConstantPool::from_vec(vec![ Constant::Placeholder ]);
            assert_eq!(1, cp_ph.len());

            let cp_valid1 = ConstantPool::from_vec(vec![
                Constant::Placeholder,
                Constant::Integer(14),
                Constant::Long(3),
                Constant::String(9)
            ]);

            assert_eq!(4, cp_valid1.len());
            assert!(cp_valid1.get_idx(0).is_none());
            assert!(cp_valid1.get_idx(5).is_none());
            assert!(cp_valid1.get_idx(1).is_some());

            match cp_valid1.get_idx(1) {
                Some(&Constant::Placeholder) => assert!(true),
                _ => assert!(false)
            }
            match cp_valid1.get_idx(2) {
                Some(&Constant::Integer(14)) => assert!(true),
                _ => assert!(false)
            }
            match cp_valid1.get_idx(3) {
                Some(&Constant::Long(3)) => assert!(true),
                _ => assert!(false)
            }
            match cp_valid1.get_idx(4) {
                Some(&Constant::String(9)) => assert!(true),
                _ => assert!(false)
            }

            let cp_valid2 = ConstantPool::from_vec(vec![
                Constant::Integer(7),
                Constant::Long(15),
                Constant::String(3)
            ]);

            assert_eq!(4, cp_valid2.len());

            match cp_valid2.get_idx(1) {
                Some(&Constant::Integer(7)) => assert!(true),
                _ => assert!(false)
            }
            match cp_valid2.get_idx(2) {
                Some(&Constant::Long(15)) => assert!(true),
                _ => assert!(false)
            }
            match cp_valid2.get_idx(3) {
                Some(&Constant::Placeholder) => assert!(true),
                _ => assert!(false)
            }
            match cp_valid2.get_idx(4) {
                Some(&Constant::String(3)) => assert!(true),
                _ => assert!(false)
            }
        }
    }

}

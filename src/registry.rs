use std::sync::{Once, ONCE_INIT};
use std::collections::HashMap;
use jvmti_wrapper::method::Method;
use jvmti_wrapper::thread::Thread;


pub struct Registry {
    instance_id: u64,
    database: HashMap<u64, u64>
}

impl Registry {

    pub fn new() -> Registry {
        Registry {
            instance_id: 0,
            database: HashMap::new()
        }
    }

}

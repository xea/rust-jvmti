use super::runtime::RuntimeEvent;

pub mod memory;

pub trait DataStore {
    fn store(&self, event: &RuntimeEvent) -> ();
}

pub trait MethodCallStore {

}

/*
pub trait MethodFactory {
    fn create(&self) -> Box<MethodTimer>;
}

pub struct MemoryMethodFactory {
    id: u32
}

impl MethodFactory for MemoryMethodFactory {

    fn create(&self) -> Box<MethodTimer> {
        Box::new(MemoryMethodTimer { id: 7 }) as Box<MethodTimer>
    }
}

// -------------------------------------

pub trait MethodTimer {

    fn enter(&self) -> Box<MethodTimer>;
}

pub struct MemoryMethodTimer {
    id: u32
}

impl MemoryMethodTimer {
    pub fn new() -> MemoryMethodTimer {
        MemoryMethodTimer { id: 23 }
    }
}

impl MethodTimer for MemoryMethodTimer {

    fn enter(&self) -> Box<MethodTimer> {
        Box::new(MemoryMethodTimer {id: 3}) as Box<MethodTimer>
    }
}

pub fn do_something(m: &MethodTimer) -> () {
    m.enter();
}
*/

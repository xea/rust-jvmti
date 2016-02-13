use super::DataStore;
use super::super::runtime::RuntimeEvent;

pub struct MemoryDataStore;

impl DataStore for MemoryDataStore {

    fn store(&self, event: &RuntimeEvent) -> () {
        
    }
}

use std::collections::HashMap;
use std::sync::RwLock;

lazy_static! {
    static ref BENCHMARK_DATA: RwLock<HashMap<BenchmarkKey, BenchmarkValue>> = RwLock::new(HashMap::new());
    static ref BENCHMARK_TIME: RwLock<HashMap<BenchmarkKey, Benchmark>> = RwLock::new(HashMap::new());
}

#[derive(Eq, PartialEq, Hash, Clone)]
pub struct BenchmarkKey {
    pub category: String,
    pub id: String
}

#[derive(Clone)]
pub struct BenchmarkValue {
    pub value: u64
}

#[derive(Clone)]
pub struct Benchmark {
    pub key: BenchmarkKey,
}

impl Benchmark {

    pub fn update(key: BenchmarkKey, value: BenchmarkValue) -> () {
        BENCHMARK_DATA.write().unwrap().insert(key, value);
    }

    pub fn get(key: &BenchmarkKey) -> Option<BenchmarkValue> {
        match BENCHMARK_DATA.read().unwrap().get(key) {
            Some(val) => Some((*val).clone()),
            None => None
        }
    }

    fn has_key(key: &BenchmarkKey) -> bool {
        BENCHMARK_DATA.read().unwrap().contains_key(key)
    }
}

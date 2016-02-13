use std::collections::HashMap;
use std::sync::RwLock;
use super::time::{now, Tm, Duration};

lazy_static! {
    static ref METHOD_TIMERS: RwLock<HashMap<BenchmarkKey, Vec<Tm>>> = RwLock::new(HashMap::new());
    static ref METHOD_COUNTER: RwLock<HashMap<BenchmarkKey, u64>> = RwLock::new(HashMap::new());
}

///
/// A composite key for identifying elements in benchamrks or identifying benchmarks within the
/// benchmark registry (as soon as there is one).
///
#[derive(Eq, PartialEq, Hash, Clone)]
pub struct BenchmarkKey {
    pub category: String,
    pub key: String
}

impl BenchmarkKey {
    pub fn new(category: String, key: String) -> BenchmarkKey {
        BenchmarkKey { category: category, key: key }
    }
}

pub struct MethodTimer;
pub struct MethodCounter;

impl MethodTimer {

    pub fn enter(key: &BenchmarkKey) -> () {
        let now = now();

        let mut vec = match MethodTimer::get(key) {
            Some(vec) => vec,
            None => vec![]
        };

        vec.push(now);

        METHOD_TIMERS.write().unwrap().insert((*key).clone(), vec);
    }

    pub fn exit(key: &BenchmarkKey) -> Option<(usize, Duration)> {
        let now = now();

        let mut vec = match MethodTimer::get(key) {
            Some(vec) => vec,
            None => vec![]
        };

        let result = match vec.pop() {
            Some(tm) => Some((vec.len(), now - tm)),
            None => None
        };

        METHOD_TIMERS.write().unwrap().insert((*key).clone(), vec);

        result
    }

    pub fn get(key: &BenchmarkKey) -> Option<Vec<Tm>> {
        match METHOD_TIMERS.read().unwrap().get(key) {
            Some(val) => Some((*val).clone()),
            None => None
        }
    }
}

impl MethodCounter {

    pub fn enter(key: &BenchmarkKey) -> () {
        let ctr = match MethodCounter::get(key) {
            Some(ctr) => ctr,
            None => 0
        };

        METHOD_COUNTER.write().unwrap().insert((*key).clone(), ctr + 1);
    }

    pub fn get(key: &BenchmarkKey) -> Option<u64> {
        match METHOD_COUNTER.read().unwrap().get(key) {
            Some(val) => Some((*val).clone()),
            None => None
        }
    }

    pub fn get_all() -> HashMap<BenchmarkKey, u64> {
        METHOD_COUNTER.read().unwrap().clone()
    }
}

extern crate time;

use time::now;

#[derive(Clone)]
struct Test {
    id: u64,
    name: String
}

impl Test {
    pub fn new(id: u64, name: String) -> Test {
        Test { id: id, name: name }
    }
}


fn main() -> () {
    let start = now();
    let a = Test::new(13, "asdfasdf".to_string());
    for x in 1..100000000 {
        let b = a.clone();
        //let c = Test::new(13, "asdfasdf".to_string());
    }
    let end = now();
    println!("It took {} lofasz", end - start);
}

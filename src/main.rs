#[macro_use]
extern crate lazy_static;
use std::thread;
use std::sync::{Arc, Mutex, Once, ONCE_INIT, RwLock};
use std::collections::HashMap;
use std::time::Duration;

lazy_static! {
    static ref DATA: RwLock<HashMap<String, u32>> = RwLock::new(HashMap::new());
}

fn main() -> () {
    for i in 1..5 {
        thread::spawn(move ||{
            println!("Spawn yay {}", i);

            let key = "A".to_string();

            thread::sleep(Duration::new(0, i * 1000000));

            let mut contains = false;
            {
                contains = DATA.read().unwrap().contains_key(&key);
                contains = DATA.write().unwrap().contains_key(&key);
                contains = DATA.write().unwrap().contains_key(&key);

            }

            if contains {
                println!("Van benne");
            } else {
                {
                    DATA.write().unwrap().insert(key, 1);
                }
                println!("Megirtam");
            }
            /*
            let result =  DATA.lock().unwrap().get(&"A".to_string());
            match result {
                Some(val) => { DATA.lock().unwrap().insert("A".to_string(), val + 1); () },
                None => { DATA.lock().unwrap().insert("A".to_string(), 1); () },
            }*/
            println!("Kijutott yay {}", i);
        });
    }

    thread::sleep(Duration::new(5, 0));
//    println!("A: {}", DATA.lock().unwrap().get(&"A".to_string()).unwrap());
}

/*
extern crate regex;

use regex::Regex;

fn main() -> () {
    //let mstr = "([I[L/java/lang/String;ZIL/java/lang/Long;)[Ljava/lang/Exception;";
    let mstr = "()[Ljava/lang/Exception;";

    let re = Regex::new(r"^\((([[]?([ZBCSIJFD]|(L[^;]+;)))*)\)(([[]?([ZBCSIJFD]|(L[^;]+;)))*)").unwrap();

    match re.captures(mstr) {
        Some(cap) => {
            println!("1: {}", cap.at(1).unwrap_or("<>"));
            println!("2: {}", cap.at(2).unwrap_or("<>"));
        },
        None => println!("ERRRO")
    }
//    }
}
*/

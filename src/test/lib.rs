extern crate libc;

mod jvmti_wrapper;

use jvmti_wrapper::Wrapper;


pub fn sg() -> () {
    let a:Wrapper = 13;
}

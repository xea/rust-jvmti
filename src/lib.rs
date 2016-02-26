extern crate libc;

use std::ptr;
use std::ffi::CStr;
use self::native::MutString;

pub mod agent;
pub mod capabilities;
pub mod class;
pub mod emulator;
pub mod environment;
pub mod event;
pub mod native;
pub mod util;

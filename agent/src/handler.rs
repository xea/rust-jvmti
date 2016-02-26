use super::context::Context;

pub type FnNativeMethodEntry = fn(context: Context) -> ();
pub type FnMethodExit = fn(context: Context) -> ();

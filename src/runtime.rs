use super::wrapper::class::Class;
use super::wrapper::thread::Thread;
use super::wrapper::method::Method;
use time::Tm;

///
/// Represents a single Java method invocation
///
pub struct MethodInvocation {
    pub class: Class,
    pub method: Method,
    pub thread: Thread,
    pub at: Tm
}

pub trait RuntimeEvent {

}

impl RuntimeEvent for MethodInvocation {
    
}

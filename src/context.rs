use super::runtime::MethodInvocationEvent;
use super::thread::ThreadId;
use std::collections::HashMap;
use std::sync::{Arc, RwLock};
use time::Duration;
use time::Tm;
use time::now;

lazy_static! {
    static ref STATIC_CONTEXT: AgentContext = AgentContext::new();
}

pub fn static_context() -> &'static AgentContext {
    &STATIC_CONTEXT
}

pub struct AgentContext {
    context: Arc<RwLock<Context>>
}

impl AgentContext {
    pub fn new() -> AgentContext {
        AgentContext {
            context: Arc::new(RwLock::new(Context::new()))
        }
    }

    pub fn thread_start(&self, thread_id: &ThreadId) {
        match self.context.write() {
            Ok(mut ctx) => {
                (*ctx).thread_lifetime.insert((*thread_id).clone(), now());
            },
            Err(_) => { /* TODO: Ignore for now */ }
        }
    }

    pub fn thread_end(&self, thread_id: &ThreadId) -> Option<Duration> {
        match self.context.write() {
            Ok(mut ctx) => {
                let now = now();
                Some((*ctx).thread_lifetime.remove(thread_id).unwrap_or(now) - now)
            },
            Err(_) => { None /* TODO: Ignore for now */ }
        }
    }

    pub fn monitor_enter(&self, thread_id: &ThreadId) {
        match self.context.write() {
            Ok(mut ctx) => {
                (*ctx).monitor_queue.insert((*thread_id).clone(), now());
            },
            Err(_) => {
                // TODO: Ignore this
            }
        }
    }

    pub fn monitor_entered(&self, thread_id: &ThreadId) -> Option<Duration> {
        match self.context.write() {
            Ok(mut ctx) => {
                let now = now();
                Some((*ctx).monitor_queue.remove(thread_id).unwrap_or(now) - now)
            },
            Err(_) => { None /* TODO: Ignore for now */ }
        }
    }

    pub fn wait_start(&self, thread_id: &ThreadId) {
        match self.context.write() {
            Ok(mut ctx) => {
                (*ctx).thread_wait.insert((*thread_id).clone(), now());
            },
            Err(_) => { /* TODO: Ignore for now */ }
        }
    }

    pub fn wait_end(&self, thread_id: &ThreadId) -> Option<Duration> {
        match self.context.write() {
            Ok(mut ctx) => {
                let now = now();
                Some((*ctx).thread_wait.remove(thread_id).unwrap_or(now) - now)
            },
            Err(_) => { None /* TODO: Ignoring for now */ }
        }
    }
}

pub struct Context {
    pub thread_lifetime: HashMap<ThreadId, Tm>,
    pub monitor_queue: HashMap<ThreadId, Tm>,
    pub thread_wait: HashMap<ThreadId, Tm>,
    pub method_times: HashMap<ThreadId, Vec<String>>
}

impl Context {
    pub fn new() -> Context {
        Context {
            thread_lifetime: HashMap::new(),
            monitor_queue: HashMap::new(),
            thread_wait: HashMap::new(),
            method_times: HashMap::new()
        }
    }
}

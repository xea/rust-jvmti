use super::thread::ThreadId;
use std::collections::HashMap;
use std::sync::{Arc, RwLock};
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
            Err(err) => {
                // TODO: Ignore for now
            }
        }
    }

}

pub struct Context {
    pub thread_lifetime: HashMap<ThreadId, Tm>
}

impl Context {
    pub fn new() -> Context {
        Context {
            thread_lifetime: HashMap::new()
        }
    }

}

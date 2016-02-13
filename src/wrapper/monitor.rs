use super::native::jvmti_native::jrawMonitorID;
use std::fmt;
use std::fmt::Display;

pub struct MonitorId {
    pub native_id: jrawMonitorID
}

/// Represents a Java raw monitor
pub struct Monitor {
    id: MonitorId
}

impl Display for Monitor {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({:p})", self.id.native_id)
    }
}

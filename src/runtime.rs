use super::class::ClassId;
use super::method::MethodId;

pub trait RuntimeEvent {
}

pub struct ObjectAllocationEvent {
    pub class_id: ClassId,
    pub size: i64
}

pub struct MethodInvocationEvent {
    pub method_id: MethodId
}

impl RuntimeEvent for ObjectAllocationEvent {}
impl RuntimeEvent for MethodInvocationEvent {}

use super::native::{JavaObject, TagId};

pub struct ObjectId {
    pub native_id: JavaObject
}

pub struct Tag {
    pub native_id: TagId
}

impl Tag {

    pub fn new(id: TagId) -> Tag {
        Tag { native_id: id }
    }
}

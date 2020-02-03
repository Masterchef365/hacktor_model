use crate::anonymous::{AnonymousData, HasTypeID, IntoAnon};
use serde::{Serialize, Deserialize};

/// Unique ID representing a specific System
pub type TopicID = u64;

#[derive(Serialize, Deserialize)]
pub struct TopicSub(pub TopicID);

/// Untyped Message for System-System interchange
#[derive(Clone, Debug)]
pub struct Message {
    /// Receivers or transmitter for this message
    pub topic: TopicID,
    pub data: AnonymousData,
}

impl Message {
    pub fn new<T: HasTypeID + Serialize>(
        topic: TopicID,
        data: T,
    ) -> Result<Self, bincode::Error> {
        Ok(Self {
            topic,
            data: data.into_anon()?,
        })
    }
}

/// Traits implemented by a System
pub trait System {
    /// Run this system
    fn run(&mut self, inbox: &[Message]) -> Box<[Message]>;
}

impl HasTypeID for TopicSub {
    const TYPE_ID: TopicID = 0xd3810409802fae6f;
}

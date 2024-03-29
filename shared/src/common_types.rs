use crate::anonymous::{AnonymousData, HasTypeID, IntoAnon};
use serde::{Serialize, Deserialize};

/// Unique ID representing a specific System
pub type TopicID = u64;

/// Message for SystemManager, subscribes this system to a topic
#[derive(Serialize, Deserialize)]
pub struct TopicSub(pub TopicID); // TODO: Add unsubscribe?

/// Untyped Message for System-System interchange
#[derive(Clone, Debug)]
pub struct Message {
    /// Receivers or transmitter for this message
    pub topic: TopicID,
    pub data: AnonymousData,
}


// TODO: This feels a little gross?
impl Message {
    pub fn topic_sub(topic_id: TopicID) -> Self {
        const SYSTEM_MANAGER_TOPIC_ID: TopicID = 0xe2cb565b9147616c;
        Self::new(SYSTEM_MANAGER_TOPIC_ID, TopicSub(topic_id)).unwrap()
    }
}

impl Message {
    /// Construct a new Message with the specified data and destination.
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

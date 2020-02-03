use crate::anonymous::*;
use crate::common_types::{Message, System, TopicID};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

pub type EntityID = u32;
pub type ComponentID = u32;
type ECData = HashMap<EntityID, HashMap<ComponentID, AnonymousData>>;

#[derive(Debug, Serialize, Deserialize)]
pub struct ECDatabaseQuery {
    entities: Vec<EntityID>,
    components: Vec<ComponentID>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ECDatabaseAvailable {
    results: Vec<(EntityID, Vec<ComponentID>)>,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum ECDatabaseMessage {
    QueryAvailable,
    Read(ECDatabaseQuery),
    Write(ECData),
}

#[derive(Default, Clone, Debug)]
pub struct ECDatabase {
    pub data: ECData,
    pub has_subbed: bool,
}

impl ECDatabase {
    pub const TOPIC_ID: TopicID = 0xbdbe41313b4c4600;
    pub fn new() -> Box<Self> {
        Box::new(Default::default())
    }
}

impl System for ECDatabase {
    fn run(&mut self, inbox: &[Message]) -> Box<[Message]> {
        let mut outbox = Vec::new();
        for message in inbox {
            if let Ok(ec_database_msg) = message.data.as_type::<ECDatabaseMessage>() {
                use crate::log_system::LogSystem;
                outbox.push(
                    Message::new(LogSystem::TOPIC_ID, format!("{:?}", ec_database_msg)).unwrap(),
                );
            }
        }

        if !self.has_subbed {
            // Subscribe to our own topic
            self.has_subbed = true;
            outbox.push(Message::topic_sub(Self::TOPIC_ID))
        }

        outbox.into_boxed_slice()
    }
}

// Type IDs
impl HasTypeID for ECDatabaseQuery {
    const TYPE_ID: u64 = 0xae6b43076062d4cb;
}

impl HasTypeID for ECDatabaseAvailable {
    const TYPE_ID: u64 = 0xdb6a6aa144747010;
}

impl HasTypeID for ECDatabaseMessage {
    const TYPE_ID: u64 = 0xb6cb54761470a3a9;
}

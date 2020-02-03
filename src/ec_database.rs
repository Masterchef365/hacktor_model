use crate::anonymous::*;
use crate::common_types::{Message, System, SystemID};
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
}

impl ECDatabase {
    pub const SYSTEM_ID: SystemID = 0xbdbe41313b4c4600;
    pub fn new() -> Box<Self> {
        Box::new(Default::default())
    }
}

impl System for ECDatabase {
    fn get_system_id(&self) -> SystemID {
        Self::SYSTEM_ID
    }

    fn run(&mut self, inbox: &[Message]) -> Box<[Message]> {
        let mut outbox = Vec::new();
        for message in inbox {
            if let Ok(ec_database_msg) = message.data.deserialize::<ECDatabaseMessage>() {
                use crate::log_system::LogSystem;
                outbox.push(
                    Message::new(LogSystem::SYSTEM_ID, format!("{:?}", ec_database_msg)).unwrap(),
                );
            }
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

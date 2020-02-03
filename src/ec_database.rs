use crate::anonymous::*;
use crate::common_types::{Message, System, SystemID};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

pub type EntityID = u32;
pub type ComponentID = u32;
type ECData = HashMap<EntityID, HashMap<ComponentID, AnonymousData>>;

#[derive(Serialize, Deserialize)]
pub struct ECDatabaseQuery {
    entities: Vec<EntityID>,
    components: Vec<ComponentID>,
}

#[derive(Serialize, Deserialize)]
pub struct ECDatabaseAvailable {
    results: Vec<(EntityID, Vec<ComponentID>)>,
}

#[derive(Serialize, Deserialize)]
pub enum ECDatabaseMessage {
    QueryAvailable,
    Read(ECDatabaseQuery),
    Write(ECData),
}

#[derive(Default, Clone, Debug)]
pub struct ECDatabase {
    pub data: ECData,
}

impl System for ECDatabase {
    fn get_system_id(&self) -> SystemID {
        0xbdbe41313b4c4600
    }

    fn run(&mut self, inbox: &[Message]) -> Box<[Message]> {
        inbox.to_vec().into_boxed_slice()
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

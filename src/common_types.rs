use crate::anonymous::AnonymousData;

#[derive(Clone, Debug)]
pub struct Message {
    /// Receivers or transmitter for this message
    pub transceivers: Vec<SystemID>,
    pub data: AnonymousData,
}

pub type SystemID = u64;

pub trait System {
    // const SYSTEM_ID: SystemID;

    /// Get this system's ID
    fn get_system_id(&self) -> SystemID;

    /// Run this system
    fn run(&mut self, inbox: &[Message]) -> Box<[Message]>;
}

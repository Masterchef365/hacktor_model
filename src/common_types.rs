use crate::anonymous::AnonymousData;

/// Untyped Message for System-System interchange
#[derive(Clone, Debug)]
pub struct Message {
    /// Receivers or transmitter for this message
    pub transceivers: Vec<SystemID>,
    pub data: AnonymousData,
}

/// Unique ID representing a specific System
pub type SystemID = u64;

/// Traits implemented by a System
pub trait System {
    /// Get this system's ID
    fn get_system_id(&self) -> SystemID;

    /// Run this system
    fn run(&mut self, inbox: &[Message]) -> Box<[Message]>;
}

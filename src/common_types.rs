use crate::anonymous::{AnonymousData, HasTypeID, IntoAnon};
use serde::Serialize;

/// Untyped Message for System-System interchange
#[derive(Clone, Debug)]
pub struct Message {
    /// Receivers or transmitter for this message
    pub transceivers: Vec<SystemID>,
    pub data: AnonymousData,
}

impl Message {
    pub fn new<T: HasTypeID + Serialize>(
        receivers: Vec<SystemID>,
        data: T,
    ) -> Result<Self, bincode::Error> {
        Ok(Self {
            transceivers: receivers,
            data: data.into_anon()?,
        })
    }
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

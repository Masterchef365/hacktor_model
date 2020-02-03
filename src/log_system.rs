use crate::common_types::{Message, System, SystemID};

/// Log message presentation System
pub struct LogSystem;

impl LogSystem {
    pub const SYSTEM_ID: SystemID = 0xdc23e00f290c8fdb;
    pub fn new() -> Box<Self> {
        Box::new(Self)
    }
}

impl System for LogSystem {
    fn get_system_id(&self) -> SystemID {
        Self::SYSTEM_ID
    }

    fn run(&mut self, inbox: &[Message]) -> Box<[Message]> {
        for msg in inbox {
            if let Ok(log_msg) = msg.data.deserialize::<&str>() {
                println!("[LOG] ({:x?}): {}", msg.transceivers[0], log_msg);
            }
        }
        Box::new([])
    }
}

use crate::anonymous::{DataTypeID, HasTypeID};
use crate::common_types::{Message, System, SystemID};
use serde::{Deserialize, Serialize};

/// Log message presentation System
pub struct LogSystem;

impl System for LogSystem {
    fn get_system_id(&self) -> SystemID {
        0xdc23e00f290c8fdb
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

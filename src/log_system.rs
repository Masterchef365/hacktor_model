use crate::anonymous::{DataTypeID, HasTypeID};
use crate::common_types::{Message, System, SystemID};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct LogMessage {
    pub text: String,
}

pub struct LogSystem;

impl System for LogSystem {
    fn get_system_id(&self) -> SystemID {
        0xdc23e00f290c8fdb
    }

    fn run(&mut self, inbox: &[Message]) -> Box<[Message]> {
        for msg in inbox {
            if let Ok(log_msg) = msg.data.as_type::<LogMessage>() {
                println!("[LOG] ({:x?}): {}", msg.transceivers[0], log_msg.text);
            }
        }
        Box::new([])
    }
}

impl HasTypeID for LogMessage {
    const TYPE_ID: DataTypeID = 0xed182c9ac7baff9d;
}

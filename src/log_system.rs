use crate::common_types::{Message, System, TopicID};

/// Log message presentation System
pub struct LogSystem {
    has_subbed: bool,
}

impl LogSystem {
    pub const TOPIC_ID: TopicID = 0xdc23e00f290c8fdb;
    pub fn new() -> Box<Self> {
        Box::new(Self { has_subbed: false })
    }
}

impl System for LogSystem {
    fn run(&mut self, inbox: &[Message]) -> Box<[Message]> {
        for msg in inbox {
            if let Ok(log_msg) = msg.data.as_type::<String>() {
                println!("[LOG] (Topic: {:x?}): {}", msg.topic, log_msg);
            }
        }

        if !self.has_subbed {
            // Subscribe to our own topic
            self.has_subbed = true;
            Box::new([Message::topic_sub(Self::TOPIC_ID)])
        } else {
            Box::new([])
        }
    }
}

use crate::common_types::{Message, System, TopicID, TopicSub};
use std::collections::HashSet;

pub struct SystemManager {
    managed_systems: Vec<ManagedSystem>, //TODO: Use slotmap here? (Might wanna remove systems during runtime)
}

struct ManagedSystem {
    pub system: Box<dyn System>,
    pub subscriptions: HashSet<TopicID>,
    pub inbox_buffer: Vec<Message>,
}

impl SystemManager {
    /// The system manager's topic id, for sending TopicSub() messages to it
    pub const TOPIC_ID: TopicID = 0xe2cb565b9147616c;

    /// Create a new SystemManager
    pub fn new() -> Self {
        Self {
            managed_systems: Vec::new(),
        }
    }

    /// Add a new system to the internal pool
    pub fn add_system(&mut self, system: Box<dyn System>) {
        self.managed_systems.push(ManagedSystem {
            system,
            subscriptions: HashSet::new(),
            inbox_buffer: Vec::new(),
        });
    }

    /// Perform a single-threaded step for all systems
    pub fn step(&mut self) {
        let mut global_outbox: Vec<Message> = Vec::new();

        // Run systems, collect outbox messages
        for managed in self.managed_systems.iter_mut() {
            // Run the system
            let outbox = managed.system.run(&managed.inbox_buffer);

            // Check for messages to the system manager itself
            for message in outbox.iter() {
                if message.topic == Self::TOPIC_ID {
                    if let Ok(TopicSub(topic_sub)) = message.data.as_type() {
                        managed.subscriptions.insert(topic_sub);
                    }
                }
            }

            // Push this system's outbox to the global outbox
            global_outbox.extend(outbox.to_vec().drain(..));
            managed.inbox_buffer.clear();
        }

        // Redistribute messages
        for message in global_outbox {
            self.distribute_message(&message);
        }
    }

    /// Distribute a message to all systems subscribed to its topic
    pub fn distribute_message(&mut self, message: &Message) {
        for managed in self.managed_systems.iter_mut() {
            if managed.subscriptions.contains(&message.topic) {
                managed.inbox_buffer.push(message.clone());
            }
        }
    }
}

// TODO: This feels a little gross?
impl Message {
    pub fn topic_sub(topic_id: TopicID) -> Self {
        Self::new(SystemManager::TOPIC_ID, TopicSub(topic_id)).unwrap()
    }
}

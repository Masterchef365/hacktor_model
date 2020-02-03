use crate::common_types::{Message, System, SystemID};
use std::collections::HashMap;

type StepPersist = HashMap<SystemID, Vec<Message>>;

/// Manages System execution and message distribution
#[derive(Default)]
pub struct SystemManager {
    systems: HashMap<SystemID, Box<dyn System>>,
    last_step_data: StepPersist,
}

impl SystemManager {
    /// Add a system to the system manager
    pub fn add_system(&mut self, system: Box<dyn System>) {
        self.systems.insert(system.get_system_id(), system);
    }

    /// Insert a message for distribution
    pub fn insert_msg(&mut self, msg: &Message) {
        Self::push_msg(msg, &mut self.last_step_data);
    }

    /// Perform execution on all systems and distribute messages
    pub fn step(&mut self) {
        let mut destinations: StepPersist = Default::default();

        for (id, system) in self.systems.iter_mut() {
            let last_data = match self.last_step_data.get(id) {
                Some(d) => d,
                None => continue,
            };

            for msg in system.run(last_data).iter() {
                Self::push_msg(msg, &mut destinations);
            }
        }

        std::mem::swap(&mut self.last_step_data, &mut destinations);
    }

    fn push_msg(msg: &Message, persist: &mut StepPersist) {
        persist
            .entry(msg.transceiver)
            .or_insert(Vec::new())
            .push(msg.clone());
    }
}

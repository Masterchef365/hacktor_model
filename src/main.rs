use hacktor::log_system::LogSystem;
use hacktor::ec_database::{ECDatabase, ECDatabaseMessage};
use hacktor::{Message, SystemManager};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut manager = SystemManager::new();
    manager.add_system(LogSystem::new());
    manager.add_system(ECDatabase::new());
    manager.step(); // Let systems init, subscribe to everything

    manager.distribute_message(&Message::new(LogSystem::TOPIC_ID, "Logging works! Yay!".to_string())?);
    manager.distribute_message(&Message::new(ECDatabase::TOPIC_ID, ECDatabaseMessage::QueryAvailable)?);
    manager.step(); // Display "Logging works! Yay!", distribute QueryAvailable to ECDatabase and get response
    manager.step(); // distribute ECDatabase message to LogSystem
    Ok(())
}

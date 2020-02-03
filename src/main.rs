use hacktor::log_system::LogSystem;
use hacktor::ec_database::{ECDatabase, ECDatabaseMessage};
use hacktor::{Message, SystemManager};

// TODO:
// Implement the ECS as a system of its own.
//     Might be an optimized bit in the server engine's binary
// Possible attacks by systems:
//     * Shittons of messages
//     * Very large messages
//     * Blocking for a long time
//     * Sending erroneous/malformed messages to other systems
//     * Faking their SystemID
//     * Growing their memory forever

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut manager = SystemManager::default();
    manager.add_system(LogSystem::new());
    manager.add_system(ECDatabase::new());
    manager.insert_msg(&Message::new(LogSystem::SYSTEM_ID, "Fuck yeah bitch".to_string())?);
    manager.insert_msg(&Message::new(ECDatabase::SYSTEM_ID, ECDatabaseMessage::QueryAvailable)?);
    manager.step();
    manager.step();
    Ok(())
}

use hacktor::anonymous::AnonymousData;
use hacktor::common_types::Message;
use hacktor::log_system::{LogMessage, LogSystem};
use hacktor::system_manager::SystemManager;

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
    manager.add_system(Box::new(LogSystem));
    manager.insert_msg(&Message {
        transceivers: vec![0xdc23e00f290c8fdb],
        data: AnonymousData::into_type(&LogMessage {
            text: "Fuck yeah bitch".to_string(),
        })
        .unwrap(),
    });
    manager.step();
    Ok(())
}
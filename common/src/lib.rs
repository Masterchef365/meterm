pub use egui;

use serde::{de::DeserializeOwned, Deserialize, Serialize};

#[derive(Clone, Serialize, Deserialize)]
pub struct ServerToClient {
    pub rendered: egui::FullOutput,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ClientToServer {
    pub raw_input: egui::RawInput,
}

pub fn serialize<T: Serialize>(val: T) -> bincode::Result<Vec<u8>> {
    bincode::serialize(&val)
}

pub fn deserialize<T: DeserializeOwned>(bytes: &[u8]) -> bincode::Result<T> {
    bincode::deserialize(bytes)
}

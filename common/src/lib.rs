pub use egui;
pub mod delta_encoding;
mod hash_abuse;

use serde::{de::DeserializeOwned, Deserialize, Serialize};

#[derive(Clone, Serialize, Deserialize)]
pub struct ServerToClient {
    pub update: delta_encoding::UpdateData,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ClientToServer {
    pub raw_input: egui::RawInput,
}

#[cfg(feature = "bincode")]
pub fn serialize<T: Serialize>(val: &T) -> bincode::Result<Vec<u8>> {
    bincode::serialize(val)
}

#[cfg(feature = "bincode")]
pub fn deserialize<T: DeserializeOwned>(bytes: &[u8]) -> bincode::Result<T> {
    bincode::deserialize(bytes)
}

#[cfg(feature = "json")]
pub fn serialize<T: Serialize>(val: &T) -> serde_json::Result<Vec<u8>> {
    serde_json::to_vec(val)
}

#[cfg(feature = "json")]
pub fn deserialize<T: DeserializeOwned>(bytes: &[u8]) -> serde_json::Result<T> {
    serde_json::from_slice(bytes)
}


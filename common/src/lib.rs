pub use egui;
pub mod delta_encoding;
mod hash_abuse;
use anyhow::Result;

use serde::{de::DeserializeOwned, Deserialize, Serialize};

#[derive(Clone, Serialize, Deserialize)]
pub struct ServerToClient {
    pub update: delta_encoding::UpdateData,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ClientToServer {
    pub raw_input: egui::RawInput,
}

#[cfg(not(feature = "json"))]
pub fn serialize<T: Serialize>(val: &T) -> bincode::Result<Vec<u8>> {
    let before = bincode::serialize(val)?;
    let after = lz4_flex::compress_prepend_size(&before);
    //dbg!(before.len(), after.len(), std::any::type_name_of_val(val));
    Ok(after)
}

#[cfg(not(feature = "json"))]
pub fn deserialize<T: DeserializeOwned>(bytes: &[u8]) -> Result<T> {
    Ok(bincode::deserialize(&lz4_flex::decompress_size_prepended(&bytes)?)?)
}

#[cfg(feature = "json")]
pub fn serialize<T: Serialize>(val: &T) -> serde_json::Result<Vec<u8>> {
    serde_json::to_vec(val)
}

#[cfg(feature = "json")]
pub fn deserialize<T: DeserializeOwned>(bytes: &[u8]) -> serde_json::Result<T> {
    serde_json::from_slice(bytes)
}


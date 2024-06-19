pub use egui;

use serde::{Deserialize, Serialize};

#[derive(Clone, Serialize, Deserialize)]
pub struct ServerToClient {
    pub rendered: egui::FullOutput,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ClientToServer {
    pub raw_input: egui::RawInput,
}

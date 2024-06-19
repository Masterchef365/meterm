pub use egui;

use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ServerToClient {
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ClientToServer {
}

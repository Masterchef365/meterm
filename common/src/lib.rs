use egui::epaint::ClippedShape;
use serde::{Deserialize, Serialize};

#[derive(Clone, Serialize, Deserialize)]
pub struct ServerToClient {
    gfx: Vec<ClippedShape>,
}

pub struct ClientToServer {
}

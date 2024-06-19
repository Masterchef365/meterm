use metacontrols_common::{ClientToServer, ServerToClient, egui};
use egui::Ui;

#[derive(Default)]
pub struct ClientGuiHandler {}

impl ClientGuiHandler {
    pub fn new() -> Self {
        Self {
        }
    }

    pub fn handle_packet_in_ui(
        &mut self,
        ui_func: &mut dyn FnMut(&mut Ui) -> (),
        packet: ClientToServer,
    ) -> ServerToClient {
        todo!()
    }
}


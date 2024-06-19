use egui::Context;
use metacontrols_common::{egui, ClientToServer, ServerToClient};

#[derive(Default)]
pub struct ClientGuiHandler {
    ctx: egui::Context,
}

impl ClientGuiHandler {
    pub fn new() -> Self {
        let ctx = Context::default();
        Self { ctx }
    }

    pub fn handle_packet_in_ui(
        &mut self,
        ui_func: &mut dyn FnMut(&Context) -> (),
        packet: ClientToServer,
    ) -> ServerToClient {
        let ClientToServer { raw_input } = packet;
        let rendered = self.ctx.run(raw_input, |ctx| ui_func(ctx));

        ServerToClient { rendered }
    }
}

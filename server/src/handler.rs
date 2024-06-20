use egui::Context;
use metacontrols_common::{egui::{self, RawInput}, ClientToServer, ServerToClient};

#[derive(Default)]
pub struct ClientGuiHandler {
    ctx: egui::Context,
    latest_blank_input: Option<RawInput>,
}

impl ClientGuiHandler {
    pub fn new() -> Self {
        let ctx = Context::default();
        Self { ctx, latest_blank_input: None }
    }

    pub fn handle_packet_in_ui(
        &mut self,
        ui_func: &mut dyn FnMut(&Context) -> (),
        packet: ClientToServer,
    ) -> Option<ServerToClient> {
        let ClientToServer { raw_input } = packet;

        // Blank input, used to send updates to clients which need updating 
        // due to activity from other clients
        let mut blank = raw_input.clone();
        blank.events.clear();
        self.latest_blank_input = Some(blank);

        let server_to_client = self.handle_raw_input_in_ui(ui_func, raw_input);

        self.ctx.has_requested_repaint().then(|| server_to_client)
    }

    pub fn handle_blank_packet_in_ui(
        &mut self,
        ui_func: &mut dyn FnMut(&Context) -> (),
    ) -> Option<ServerToClient> {
        self.latest_blank_input.clone().map(|raw_input| {
            self.handle_raw_input_in_ui(ui_func, raw_input)
        })
    }

    fn handle_raw_input_in_ui(
        &mut self,
        ui_func: &mut dyn FnMut(&Context) -> (),
        raw_input: RawInput,
    ) -> ServerToClient {
        let rendered = self.ctx.run(raw_input, |ctx| ui_func(ctx));
        ServerToClient { rendered }
    }

}

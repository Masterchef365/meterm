use std::any::Any;

use egui::Context;
use metacontrols_common::{delta_encoding, egui::{self, ahash::HashMap, RawInput}, ClientToServer, ServerToClient};

pub type UserStore = HashMap<&'static str, Box<dyn Any + Send + Sync + 'static>>;

#[derive(Default)]
pub struct ClientGuiHandler {
    ctx: egui::Context,
    encoder: delta_encoding::Encoder,
    latest_blank_input: Option<RawInput>,
    user_storage: UserStore,
}

impl ClientGuiHandler {
    pub fn new() -> Self {
        let ctx = Context::default();
        Self { 
            ctx, 
            latest_blank_input: None, 
            user_storage: Default::default(),
            encoder: Default::default(),
        }
    }

    pub fn handle_packet_in_ui(
        &mut self,
        ui_func: &mut dyn FnMut(&Context, &mut UserStore) -> (),
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
        ui_func: &mut dyn FnMut(&Context, &mut UserStore) -> (),
    ) -> Option<ServerToClient> {
        self.latest_blank_input.clone().map(|raw_input| {
            self.handle_raw_input_in_ui(ui_func, raw_input)
        })
    }

    fn handle_raw_input_in_ui(
        &mut self,
        ui_func: &mut dyn FnMut(&Context, &mut UserStore) -> (),
        raw_input: RawInput,
    ) -> ServerToClient {

        let full_output = self.ctx.run(raw_input, |ctx| ui_func(ctx, &mut self.user_storage));

        let update = self.encoder.encode(&full_output);

        ServerToClient { update }
    }

}

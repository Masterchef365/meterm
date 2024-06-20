use egui::{mutex::Mutex, Event, Id, InputState, RawInput, Rect, Sense, Ui, Vec2, Widget};
use ewebsock::{WsEvent, WsMessage};
use log::{info, trace};
use metacontrols_common::{
    delta_encoding::{self, Decoder},
    deserialize,
    egui::{self, epaint::ClippedShape, Context, FullOutput},
    serialize, ClientToServer, ServerToClient,
};
use std::sync::Arc;

#[derive(Clone, Debug)]
pub struct ServerWidget {
    pub addr: String,
    pub desired_size: Vec2,
}

impl ServerWidget {
    pub fn new(addr: impl Into<String>) -> Self {
        Self {
            addr: addr.into(),
            desired_size: Vec2::new(200., 200.),
        }
    }

    pub fn with_desired_size(mut self, size: Vec2) -> Self {
        self.desired_size = size;
        self
    }
}

impl Widget for ServerWidget {
    fn ui(self, ui: &mut Ui) -> egui::Response {
        let client = ui.ctx().memory_mut(|mem| {
            mem.data
                .get_temp_mut_or_insert_with(Id::new(&self.addr), || {
                    Arc::new(Mutex::new(Client::new(self.clone(), ui.ctx())))
                })
                .clone()
        });

        let mut lck = client.lock();
        lck.show(ui)
    }
}

enum Client {
    Success(ClientImpl),
    Failure { error: String },
}

#[cfg(target_arch = "wasm32")]
unsafe impl Sync for Client {}

#[cfg(target_arch = "wasm32")]
unsafe impl Send for Client {}

impl Client {
    fn new(view: ServerWidget, ctx: &Context) -> Self {
        let ctx = ctx.clone();
        match ewebsock::connect_with_wakeup(&view.addr, Default::default(), move || {
            ctx.request_repaint()
        }) {
            Ok((tx, rx)) => Self::Success(ClientImpl::new(tx, rx, view)),
            Err(e) => Self::Failure {
                error: format!("{:?}", e),
            },
        }
    }

    fn show(&mut self, ui: &mut Ui) -> egui::Response {
        match self {
            Self::Failure { error } => ui.label(format!("Error; {error}")),
            Self::Success(client) => match client.show(ui) {
                Err(error) => {
                    let mut fail = Self::Failure { error };
                    let resp = fail.show(ui);
                    *self = fail;
                    resp
                }
                Ok(resp) => resp,
            },
        }
    }
}

struct ClientImpl {
    tx: ewebsock::WsSender,
    rx: ewebsock::WsReceiver,
    view: ServerWidget,
    latest_frame: Option<FullOutput>,
    open: bool,
    decoder: delta_encoding::Decoder,
}

impl ClientImpl {
    fn new(tx: ewebsock::WsSender, rx: ewebsock::WsReceiver, view: ServerWidget) -> Self {
        Self {
            tx,
            rx,
            view,
            latest_frame: None,
            open: false,
            decoder: Decoder::new(),
        }
    }

    fn show(&mut self, ui: &mut Ui) -> Result<egui::Response, String> {
        // Receive messages from server
        loop {
            match self.rx.try_recv() {
                Some(WsEvent::Opened) => dbg!(self.open = true),
                Some(WsEvent::Message(WsMessage::Binary(msg))) => {
                    info!("Length {}", msg.len());
                    let packet: ServerToClient = deserialize(&msg).unwrap();
                    if let Some(full_output) = self.decoder.decode(packet.update.clone()) {
                        self.latest_frame = Some(full_output);
                    }
                }
                Some(WsEvent::Error(e)) => return Err(format!("{e:#?}")),
                _ => break,
            }
        }

        // Allocate some space
        let resp = ui.allocate_response(self.view.desired_size, Sense::click_and_drag());

        // Draw the server contents
        if let Some(full_output) = &self.latest_frame {
            for ClippedShape { clip_rect, shape } in &full_output.shapes {
                let offset = resp.rect.left_top().to_vec2();
                let mut shape = shape.clone();
                shape.translate(offset);
                ui.set_clip_rect(clip_rect.translate(offset));
                ui.painter().add(shape.clone());
            }
        }

        // Capture input
        let raw_input = ui
            .ctx()
            .input(|input_state| convert_subwindow_input(input_state, resp.rect));

        // Send response
        if self.open {
            self.tx.send(WsMessage::Binary(
                serialize(&ClientToServer { raw_input }).unwrap(),
            ))
        }

        Ok(resp)
    }
}

fn convert_subwindow_input(input_state: &InputState, rect: Rect) -> RawInput {
    let mut raw = input_state.raw.clone();
    for ev in &mut raw.events {
        match ev {
            Event::PointerMoved(new_pos) => {
                *new_pos -= rect.left_top().to_vec2();
            }
            Event::PointerButton { pos, .. } => {
                *pos -= rect.left_top().to_vec2();
            }
            _ => (),
        }
    }

    raw
}

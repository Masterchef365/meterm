use egui::{mutex::Mutex, Event, Id, InputState, RawInput, Rect, Sense, Ui, Vec2, Widget};
use ewebsock::{WsEvent, WsMessage};
use metacontrols_common::{
    deserialize,
    egui::{self, epaint::ClippedShape, FullOutput},
    serialize, ClientToServer,
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
                    Arc::new(Mutex::new(Client::new(self.clone())))
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

impl Client {
    fn new(view: ServerWidget) -> Self {
        match ewebsock::connect(&view.addr, Default::default()) {
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
    draw: Option<FullOutput>,
    open: bool,
}

impl ClientImpl {
    fn new(tx: ewebsock::WsSender, rx: ewebsock::WsReceiver, view: ServerWidget) -> Self {
        Self {
            tx,
            rx,
            view,
            draw: None,
            open: false,
        }
    }

    fn show(&mut self, ui: &mut Ui) -> Result<egui::Response, String> {
        // Receive messages from server
        match self.rx.try_recv() {
            Some(WsEvent::Opened) => dbg!(self.open = true),
            Some(WsEvent::Message(WsMessage::Binary(msg))) => {
                self.draw = Some(deserialize(&msg).unwrap())
            }
            Some(WsEvent::Error(e)) => return Err(format!("{e:#?}")),
            _ => (),
        }

        // Allocate some space
        let resp = ui.allocate_response(self.view.desired_size, Sense::click_and_drag());

        // Draw the server contents
        if let Some(full_output) = &self.draw {
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
                serialize(ClientToServer { raw_input }).unwrap(),
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

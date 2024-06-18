use std::sync::Arc;
use egui::{mutex::Mutex, Id, Sense, Ui, Vec2, Widget};

#[derive(Clone)]
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
    Failure { addr: String, error: String },
}

impl Client {
    fn new(view: ServerWidget) -> Self {
        match ewebsock::connect(&view.addr, Default::default()) {
            Ok((tx, rx)) => Self::Success(ClientImpl { rx, tx, view }),
            Err(e) => Self::Failure {
                addr: view.addr,
                error: format!("{:?}", e),
            },
        }
    }

    fn show(&mut self, ui: &mut Ui) -> egui::Response {
        match self {
            Self::Failure { addr, error } => {
                ui.label(format!("Error connecting to {addr}; {error}"))
            }
            Self::Success(client) => client.show(ui),
        }
    }
}

struct ClientImpl {
    tx: ewebsock::WsSender,
    rx: ewebsock::WsReceiver,
    view: ServerWidget,
}

impl ClientImpl {
    fn show(&mut self, ui: &mut Ui) -> egui::Response {
        let resp = ui.allocate_response(self.view.desired_size, Sense::click_and_drag());
        ui.label("TODO")
    }
}

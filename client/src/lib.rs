use std::{net::ToSocketAddrs, sync::Arc};

use egui::{mutex::Mutex, Id, Ui, Vec2, Widget};

pub struct ServerView {
    addr: String,
    size: Vec2,
}

impl ServerView {
    pub fn new(addr: String) -> Self {
        Self {
            addr,
            size: Vec2::new(200., 200.),
        }
    }
}

impl Widget for ServerView {
    fn ui(self, ui: &mut Ui) -> egui::Response {
        let client = ui.ctx().memory_mut(|mem| {
            mem.data
                .get_temp_mut_or_insert_with(Id::new(&self.addr), || {
                    Arc::new(Mutex::new(Client::new(self.addr.clone())))
                })
                .clone()
        });

        let mut lck = client.lock();
        lck.show(ui)
    }
}

pub struct Client {

}

impl Client {
    fn new(addr: String) -> Self {
    }

    fn show(&mut self, ui: &mut Ui) -> egui::Response {
        todo!()
    }
}

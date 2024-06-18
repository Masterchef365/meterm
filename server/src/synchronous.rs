use super::*;

pub struct Server(Arc<Mutex<ServerImpl>>);

impl Server {
    pub fn start_server(addr: impl ToSocketAddrs + 'static) -> Self {
        tokio::spawn(move || server_loop(addr));
    }

    pub fn show_on_clients(&self, ui: &Ui, userfunc: &mut dyn FnMut(&Ui) -> ()) {
        tokio::task::spawn_blocking(async {
            self.0.lock().await.show_on_clients(ui, userfunc).await
        });
    }
}

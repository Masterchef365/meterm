use std::sync::Arc;

use egui::Ui;
use futures_util::{stream::StreamExt, TryStreamExt};
use log::info;
use tokio::{
    net::{TcpListener, TcpStream, ToSocketAddrs},
    sync::RwLock,
};

pub async fn start_server_loop(addr: impl ToSocketAddrs + 'static + Sync + Send) -> Server {
    //tokio::spawn(server_loop(addr));
    todo!()
}

pub struct Server {
    client_states: HashMap
}

impl Server {
    pub async fn show_on_clients(&mut self, mut f: impl FnMut(&mut Ui)) {
        todo!()
    }
}

/*
async fn accept_connection(stream: TcpStream) {
    let addr = stream
        .peer_addr()
        .expect("connected streams should have a peer address");
    info!("Peer address: {}", addr);

    let ws_stream = tokio_tungstenite::accept_async(stream)
        .await
        .expect("Error during the websocket handshake occurred");

    info!("New WebSocket connection: {}", addr);

    let (write, read) = ws_stream.split();

    // We should not forward messages other than text or binary.
    read.try_filter(|msg| futures_util::future::ready(msg.is_text() || msg.is_binary()))
        .forward(write)
        .await
        .expect("Failed to forward messages")
}

async fn server_loop(addr: impl ToSocketAddrs) {
    let try_socket = TcpListener::bind(&addr).await;

    let listener = try_socket.expect("Failed to bind");
    //info!("Listening on: {}", addr.to_socket_addrs(internal));

    while let Ok((stream, _)) = listener.accept().await {
        tokio::spawn(accept_connection(stream));
    }
}

impl ServerImpl {
    pub async fn show_on_clients(&mut self, userfunc: &mut dyn FnMut(&Ui) -> ()) {
        for client in &mut self.clients {
            client.write().await.handle_userfunc(userfunc).await;
        }
    }
}

impl Client {
    async fn handle_userfunc(&mut self, userfunc: &mut dyn FnMut(&Ui) -> ()) {
        todo!()
    }
}
*/

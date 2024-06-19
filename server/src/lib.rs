use std::sync::Arc;

use egui::{ahash::HashMap, Ui};
use futures_util::SinkExt;
use futures_util::{stream::StreamExt, TryStreamExt};
use log::info;
use metacontrols_common::{ClientToServer, ServerToClient};
use tokio::net::{TcpListener, TcpStream, ToSocketAddrs};
use tokio_tungstenite::tungstenite::Message;

/*
enum CompressionLevel {
    /// See for example 33% of original bandwidth use and like no lag, dude
    Normal,
    /// See for example 28% of original bandwidth use and maybe a little lag
    Maximum,
}

struct ServerConfig {
    // Optional compression (can get good rates with
    compression: Option<CompressionLevel>,
}
*/

pub struct Server {
    new_client_rx: std::sync::mpsc::Receiver<Client>,
    clients: Vec<Client>,
}

pub struct Client {
    rx: std::sync::mpsc::Receiver<ClientToServer>,
    tx: tokio::sync::mpsc::Sender<ServerToClient>,
    gui_handler: ClientGuiHandler,
    // TODO: task join handle here
}

#[derive(Default)]
struct ClientGuiHandler {}

impl Server {
    pub fn new(addr: impl ToSocketAddrs + 'static + Sync + Send) -> Self {
        let (new_client_tx, new_client_rx) = std::sync::mpsc::channel();
        tokio::spawn(server_loop(addr, new_client_tx));

        Self {
            new_client_rx,
            clients: vec![],
        }
    }

    pub fn show_on_clients(&mut self, ui_func: &mut dyn FnMut(&mut Ui)) {
        // Register new clients
        self.clients.extend(self.new_client_rx.try_iter());

        for client in &mut self.clients {
            client.handle_ui(ui_func);
        }
    }
}

async fn accept_connection(
    stream: TcpStream,
    mut tx: std::sync::mpsc::Sender<ClientToServer>,
    mut rx: tokio::sync::mpsc::Receiver<ServerToClient>,
) {
    let mut ws_stream = tokio_tungstenite::accept_async(stream)
        .await
        .expect("Error during the websocket handshake occurred");

    info!("New WebSocket connection");

    loop {
        tokio::select! {
            msg = ws_stream.next() => {
                if let Some(Ok(Message::Binary(msg))) = msg {
                    tx.send(bincode::deserialize(&msg).unwrap()).unwrap();
                }
            },
            val = rx.recv() => {
                ws_stream.send(Message::Binary(bincode::serialize(&val).unwrap())).await.unwrap();
            },
        }
    }
}

async fn server_loop(addr: impl ToSocketAddrs, new_client_tx: std::sync::mpsc::Sender<Client>) {
    let try_socket = TcpListener::bind(&addr).await;

    let listener = try_socket.expect("Failed to bind");

    while let Ok((stream, _)) = listener.accept().await {
        let (client_to_server_tx, client_to_server_rx) = std::sync::mpsc::channel();
        let (server_to_client_tx, server_to_client_rx) = tokio::sync::mpsc::channel(100);

        new_client_tx
            .send(Client {
                rx: client_to_server_rx,
                tx: server_to_client_tx,
                gui_handler: ClientGuiHandler::default(),
            })
            .unwrap();

        tokio::spawn(accept_connection(
            stream,
            client_to_server_tx,
            server_to_client_rx,
        ));
    }
}

impl Client {
    fn handle_ui(&mut self, ui_func: &mut dyn FnMut(&mut Ui) -> ()) {
        for packet in self.rx.try_iter() {
            let return_packet = self.gui_handler.handle_packet_in_ui(ui_func, packet);
            self.tx.blocking_send(return_packet).unwrap();
        }
    }
}

impl ClientGuiHandler {
    fn handle_packet_in_ui(
        &mut self,
        ui_func: &mut dyn FnMut(&mut Ui) -> (),
        packet: ClientToServer,
    ) -> ServerToClient {
        todo!()
    }
}

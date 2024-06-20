use std::sync::Arc;

use egui::{ahash::HashMap, Context, Ui};
use futures_util::SinkExt;
use futures_util::{stream::StreamExt, TryStreamExt};
use handler::ClientGuiHandler;
use log::{error, info, warn};
use metacontrols_common::{ClientToServer, ServerToClient};
use tokio::net::{TcpListener, TcpStream, ToSocketAddrs};
use tokio_tungstenite::tungstenite::Message;

pub use metacontrols_common::egui;

mod handler;

pub struct Server {
    new_client_rx: std::sync::mpsc::Receiver<Client>,
    clients: Vec<Client>,
    runtime: tokio::runtime::Runtime,
}

pub struct Client {
    rx: std::sync::mpsc::Receiver<ClientToServer>,
    tx: tokio::sync::mpsc::Sender<ServerToClient>,
    gui_handler: ClientGuiHandler,
    // TODO: task join handle here
}

impl Server {
    pub fn new(addr: impl Into<String>) -> Self {
        let (new_client_tx, new_client_rx) = std::sync::mpsc::channel();

        let runtime = tokio::runtime::Runtime::new().unwrap();
        runtime.spawn(server_loop(addr.into(), new_client_tx));

        Self {
            runtime,
            new_client_rx,
            clients: vec![],
        }
    }

    pub fn show_on_clients(&mut self, mut ui_func: impl FnMut(&Context)) {
        // Register new clients
        self.clients.extend(self.new_client_rx.try_iter());

        // Handle each client
        for client in &mut self.clients {
            client.handle_ctx(&mut ui_func);
        }
    }
}

async fn accept_connection(
    stream: TcpStream,
    tx: std::sync::mpsc::Sender<ClientToServer>,
    mut rx: tokio::sync::mpsc::Receiver<ServerToClient>,
) {
    let mut ws_stream = tokio_tungstenite::accept_async(stream)
        .await
        .expect("Error during the websocket handshake occurred");

    info!("New WebSocket connection");

    loop {
        tokio::select! {
            msg = ws_stream.next() => {
                match msg {
                    Some(Ok(Message::Binary(msg))) => tx.send(
                        metacontrols_common::deserialize::<ClientToServer>(&msg).unwrap()
                    ).unwrap(),
                    Some(Err(e)) => {
                        warn!("Receiving from stream; {}", e);
                        break;
                    }
                    _ => (),
                }
            },
            Some(val) = rx.recv() => {
                let ser = metacontrols_common::serialize::<ServerToClient>(&val).unwrap();
                ws_stream.send(Message::Binary(
                        ser
                )).await.unwrap();
            },
        }
        // Always await on at least something
        tokio::task::yield_now().await;
    }
}

async fn server_loop(addr: String, new_client_tx: std::sync::mpsc::Sender<Client>) {
    let try_socket = TcpListener::bind(&addr).await;

    let listener = try_socket.expect("Failed to bind");

    while let Ok((stream, _)) = listener.accept().await {
        let (client_to_server_tx, client_to_server_rx) = std::sync::mpsc::channel();
        let (server_to_client_tx, server_to_client_rx) = tokio::sync::mpsc::channel(100);

        new_client_tx
            .send(Client {
                rx: client_to_server_rx,
                tx: server_to_client_tx,
                gui_handler: ClientGuiHandler::new(),
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
    fn handle_ctx(&mut self, ui_func: &mut dyn FnMut(&Context) -> ()) {
        for packet in self.rx.try_iter() {
            if let Some(return_packet) = self.gui_handler.handle_packet_in_ui(ui_func, packet) {
                let _ = self.tx.blocking_send(return_packet);
            }
        }
    }
}

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

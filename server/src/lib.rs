use std::sync::Arc;

use egui::{ahash::HashMap, Context, Ui};
use futures_util::SinkExt;
use futures_util::{stream::StreamExt, TryStreamExt};
use handler::ClientGuiHandler;
use log::{error, info, warn};
use meterm_common::delta_encoding::Encoder;
use meterm_common::{delta_encoding, ClientToServer, ServerToClient};
use tokio::net::{TcpListener, TcpStream, ToSocketAddrs};
use tokio_tungstenite::tungstenite::Message;

pub use meterm_common::egui;

pub mod utils;

mod handler;

pub struct Server {
    new_client_rx: std::sync::mpsc::Receiver<Client>,
    clients: Vec<Client>,
    runtime: tokio::runtime::Runtime,
    force_repaint: bool,
}

pub struct Client {
    rx: std::sync::mpsc::Receiver<ClientToServer>,
    tx: tokio::sync::mpsc::Sender<ServerToClient>,
    gui_handler: ClientGuiHandler,
    encoder: delta_encoding::Encoder,
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
            force_repaint: false,
        }
    }

    pub fn for_each_client(&mut self, mut ui_func: impl FnMut(&Context)) {
        // Register new clients
        self.clients.extend(self.new_client_rx.try_iter());

        // Drop disconnected clients
        self.clients.retain(|client| client.is_alive());

        // Handle each client
        let mut any_requested_repaint = false;
        for client in &mut self.clients {
            any_requested_repaint |= client.handle_ctx(&mut ui_func, self.force_repaint);
        }

        self.force_repaint = any_requested_repaint;
    }
}

async fn accept_connection(
    stream: TcpStream,
    tx: std::sync::mpsc::Sender<ClientToServer>,
    mut rx: tokio::sync::mpsc::Receiver<ServerToClient>,
) {
    let mut ws_stream = match tokio_tungstenite::accept_async(stream).await {
        Ok(stream) => stream,
        Err(e) => {
            warn!("Error during the websocket handshake occurred; {e}");
            return;
        }
    };

    info!("New WebSocket connection");

    loop {
        tokio::select! {
            msg = ws_stream.next() => {
                match msg {
                    Some(Ok(Message::Binary(msg))) => tx.send(
                        meterm_common::deserialize::<ClientToServer>(&msg).unwrap()
                    ).unwrap(),
                    Some(Ok(Message::Close(_))) => {
                        info!("Graceful shutdown");
                        break;
                    },
                    Some(Err(e)) => {
                        warn!("Receiving from stream; {}", e);
                        break;
                    }
                    _ => (),
                }
            },
            Some(val) = rx.recv() => {
                let ser = meterm_common::serialize::<ServerToClient>(&val).unwrap();
                let _ = ws_stream.send(Message::Binary(
                        ser.into()
                )).await;
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
                encoder: Encoder::new(),
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
    fn handle_ctx(
        &mut self,
        ui_func: &mut dyn FnMut(&Context),
        force_update: bool,
    ) -> bool {
        let mut any_requested_repaint = false;

        // Update clients which updated
        let mut needs_blank_update = force_update;
        for packet in self.rx.try_iter() {
            needs_blank_update = false;
            if let Some(return_packet) = self.gui_handler.handle_packet_in_ui(ui_func, packet) {
                any_requested_repaint = true;
                let _ = self.tx.blocking_send(ServerToClient {
                    update: self.encoder.encode(&return_packet),
                });
            }
        }

        // Use an eventless version of the last raw input to generate an update
        if needs_blank_update {
            if let Some(return_packet) = self.gui_handler.handle_blank_packet_in_ui(ui_func) {
                let _ = self.tx.blocking_send(ServerToClient {
                    update: self.encoder.encode(&return_packet),
                });
            }
        }

        any_requested_repaint
    }

    fn is_alive(&self) -> bool {
        !self.tx.is_closed()
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

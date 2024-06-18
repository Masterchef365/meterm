#[tokio::main]
async fn main() {
    env_logger::try_init().unwrap();

    let mut server = metacontrols_server::start_server_loop("0.0.0.0:5000").await;

    let mut counter = 0;

    loop {
        server.show_on_clients(|ui| {
            if ui.button("Hello world!").clicked() {
                counter += 1;
            }
        }).await;
    }
}

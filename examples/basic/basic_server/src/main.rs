use metacontrols_server::Server;

fn main() {
    env_logger::try_init().unwrap();

    let mut server = Server::new("0.0.0.0:5000");

    let mut counter = 0;

    loop {
        server.show_on_clients(&mut |ui| {
            if ui.button("Hello world!").clicked() {
                counter += 1;
            }
        });
    }
}

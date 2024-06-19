use metacontrols_server::{egui, Server};

fn main() {
    env_logger::try_init().unwrap();

    let mut server = Server::new("0.0.0.0:5000");

    let mut counter = 0;

    loop {
        server.show_on_clients(|ctx| {
            dbg!("thing");
            egui::CentralPanel::default().show(ctx, |ui| {
                if ui.button("Hello world!").clicked() {
                    counter += 1;
                }
            });
        });

        std::thread::sleep(std::time::Duration::from_micros(100));
    }
}

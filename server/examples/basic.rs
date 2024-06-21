use meterm_server::{Server, egui};

fn main() {
    let mut server = Server::new("0.0.0.0:5000");
    let mut counter = 0;

    loop {
        server.for_each_client(|ctx| {
            egui::CentralPanel::default().show(ctx, |ui| {
                if ui.button(format!("Click to increase! ({})", counter)).clicked() {
                    counter += 1;
                }
            });
        });
    }
}

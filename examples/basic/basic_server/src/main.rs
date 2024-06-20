use std::time::Instant;

use metacontrols_server::{egui, Server};

fn main() {
    env_logger::try_init().unwrap();

    let mut server = Server::new("0.0.0.0:5000");

    let mut counter = 0;

    let mut user_counter: usize = 0;

    // We want 20 ticks per second
    let desired_tickrate = 90.0;

    loop {
        let tick_start = Instant::now();

        server.show_on_clients(|ctx, user| {
            let number = user
                .entry("number")
                .or_insert_with(|| {
                    user_counter += 1;
                    Box::new(user_counter)
                })
                .downcast_ref::<usize>()
                .unwrap();

            egui::CentralPanel::default().show(ctx, |ui| {
                ui.label(format!("You are user #{}", number));
                if ui.button(format!("Hello world! {}", counter)).clicked() {
                    counter += 1;
                }
            });
        });

        let tick_time = tick_start.elapsed();
        let remaining_time = (1. / desired_tickrate - tick_time.as_secs_f32()).max(0.0);
        std::thread::sleep(std::time::Duration::from_secs_f32(remaining_time));
    }
}

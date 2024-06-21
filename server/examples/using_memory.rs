use std::time::Instant;

use metacontrols_common::egui::Id;
use metacontrols_server::{
    egui::{self, DragValue, Slider},
    Server,
};

struct UserNumber(usize);

fn main() {
    env_logger::try_init().unwrap();

    let mut server = Server::new("0.0.0.0:5000");

    let mut counter = 0;
    let mut drag = 0.0;

    let mut user_counter: usize = 0;

    // We want 20 ticks per second
    let desired_tickrate = 90.0;

    loop {
        let tick_start = Instant::now();

        server.for_each_client(|ctx| {
            let user_number = ctx.memory_mut(|mem| {
                *mem.data
                    .get_temp_mut_or_insert_with(Id::new("user_number"), || {
                        user_counter += 1;
                        user_counter
                    })
            });

            egui::CentralPanel::default().show(ctx, |ui| {
                ui.label(format!("You are user #{}", user_number));
                if ui.button(format!("Hello world! {}", counter)).clicked() {
                    counter += 1;
                }

                ui.add(DragValue::new(&mut drag));
                ui.add(Slider::new(&mut drag, 0.0..=1000.0));
            });
        });

        let tick_time = tick_start.elapsed();
        let remaining_time = (1. / desired_tickrate - tick_time.as_secs_f32()).max(0.0);
        std::thread::sleep(std::time::Duration::from_secs_f32(remaining_time));
    }
}

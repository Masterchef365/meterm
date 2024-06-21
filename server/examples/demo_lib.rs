use std::{
    sync::{Arc, Mutex},
    time::Instant,
};

use egui_demo_lib::DemoWindows;
use metacontrols_common::egui::Id;
use metacontrols_server::{egui, Server};

#[derive(Default)]
struct SafeDemo(DemoWindows);

// I'm sure this is fine. :)
unsafe impl Send for SafeDemo {}
unsafe impl Sync for SafeDemo {}

fn main() {
    env_logger::try_init().unwrap();

    let mut server = Server::new("0.0.0.0:5000");


    loop {
        let tick_start = Instant::now();

        server.show_on_clients(|ctx| {
            let demo = ctx.memory_mut(|mem| {
                mem.data
                    .get_temp_mut_or_insert_with(Id::new("Demo"), || {
                        Arc::new(Mutex::new(SafeDemo(DemoWindows::default())))
                    })
                    .clone()
            });

            demo.lock().unwrap().0.ui(ctx);
        });

        let desired_tickrate = 90.0;
        let tick_time = tick_start.elapsed();
        let remaining_time = (1. / desired_tickrate - tick_time.as_secs_f32()).max(0.0);
        std::thread::sleep(std::time::Duration::from_secs_f32(remaining_time));
    }
}

use std::time::Instant;

use egui_demo_lib::DemoWindows;
use metacontrols_server::{egui, Server};

#[derive(Default)]
struct SafeDemo(DemoWindows);

// I'm sure this is fine. :)
unsafe impl Send for SafeDemo {}
unsafe impl Sync for SafeDemo {}

fn main() {
    env_logger::try_init().unwrap();

    let mut server = Server::new("0.0.0.0:5000");

    let mut counter = 0;

    // We want 20 ticks per second
    let desired_tickrate = 90.0;

    loop {
        let tick_start = Instant::now();

        server.show_on_clients(|ctx, user| {
            let demo = user.entry("Demo")
                .or_insert_with(|| Box::new(SafeDemo(DemoWindows::default())))
                .downcast_mut::<SafeDemo>()
                .unwrap();

            demo.0.ui(ctx);
        });

        let tick_time = tick_start.elapsed();
        let remaining_time = (1. / desired_tickrate - tick_time.as_secs_f32()).max(0.0);
        std::thread::sleep(std::time::Duration::from_secs_f32(remaining_time));
    }
}

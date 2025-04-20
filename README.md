# Meterm 
Meterm is a web-compatible visual terminal for egui, heavily inspired by [eterm](https://github.com/emilk/eterm).

All rendering is done on the server, and only descriptions of shapes are passed over the wire.

For example:
```rust
let mut server = Server::new("0.0.0.0:5000");
let mut counter = 0;

loop {
    server.for_each_client(|ctx| {
        egui::CentralPanel::default().show(ctx, |ui| {
            let text = format!("Click to increase! ({})", counter);
            if ui.button(text).clicked() {
                counter += 1;
            }
        });
    });
}
```
We could then access this service by visiting `https://masterchef365.github.io/meterm-viewer/?srv=ws://localhost:5000`. Note that the viewer we are using (meterm-viewer) has no prior knowledge of this service, besides its URL.

The current implementation is NOT production-ready. It requires a (lightly) patched fork of egui. The internals are cursed and use unwrap(). It's buggy. There are features missing. It has scalability issues. It's unencrypted. But this took me most of my week off so I thought I'd share. Cheers!

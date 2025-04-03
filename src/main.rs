use gtk::prelude::*;
use gtk::{Application, ApplicationWindow, Button, Label, Box};
use std::sync::{Arc, Mutex};

struct AppState {
    status: String,
}

fn main() {
    let application = Application::new(Some("com.example.roblox_counter_blox_multi"), Default::default());
    application.connect_activate(|app| {
        let window = ApplicationWindow::new(app);
        window.set_title("Roblox Counter Blox Multi");
        window.set_default_size(400, 200);

        let state = Arc::new(Mutex::new(AppState { status: String::from("Ready") }));
        let vbox = Box::new(gtk::Orientation::Vertical, 5);
        
        let label = Label::new(Some("Status: Ready"));
        vbox.pack_start(&label, true, true, 0);

        let button = Button::with_label("Start");
        vbox.pack_start(&button, true, true, 0);

        button.connect_clicked({
            let state = Arc::clone(&state);
            move |_| {
                let mut state = state.lock().unwrap();
                state.status = String::from("Running...");
                label.set_text(&format!("Status: {}", state.status));
            }
        });

        window.add(&vbox);
        window.show_all();
    });

    application.run();
}
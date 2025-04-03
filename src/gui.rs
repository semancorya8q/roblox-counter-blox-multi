use gtk::prelude::*;
use gtk::{Window, Button, Label, Box};

pub struct GUI {
    window: Window,
    label: Label,
}

impl GUI {
    pub fn new() -> Self {
        let window = Window::new(gtk::WindowType::Toplevel);
        window.set_title("Roblox Counter Blox Multi");
        window.set_default_size(400, 200);

        let vbox = Box::new(gtk::Orientation::Vertical, 5);
        let label = Label::new(Some("Status: Ready"));
        vbox.pack_start(&label, true, true, 0);

        let button = Button::with_label("Start");
        vbox.pack_start(&button, true, true, 0);

        window.add(&vbox);
        window.show_all();

        GUI { window, label }
    }

    pub fn update_status(&self, status: &str) {
        self.label.set_text(status);
    }
}
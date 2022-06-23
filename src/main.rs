use fltk::{app, button, prelude::*, window};

fn main () {
    // Plan : Window, Binary Search - Reggex (/usr/bin), Icons, Animation / Switcher, Indexing
    // Link /usr/bin -> /usr/share/icons
    let application = app::App::default();
    let mut win = window::Window::default().with_size(400, 300).with_label("Rusti");
    let mut btn = button::Button::default().with_size(80, 30).center_of_parent().with_label("Click");
    win.end();
    win.show();
    btn.set_callback(move |b| {
        b.set_label("Clicked");
        win.set_label("You can't see me");
    });
    application.run().unwrap();
}

use fltk::{app, button::Button, prelude::*, window::Window};

fn main() {
    let app = app::App::default();
    let mut w = Window::new(100, 100, 250, 250, "");

    let bw = 60;
    let bh = 30;
    let mut b = Button::default()
        .with_pos((w.w() - bw) / 2, (w.h() - bh) / 2)
        .with_size(bw, bh)
        .with_label("Resize");
    b.set_callback(resize);

    w.show();
    app.run().unwrap();
}

fn resize(b: &mut Button) {
    if let Some(ref mut w) = b.parent() {
        if w.h() == 250 {
            w.resize(w.x(), w.y(), w.w(), 150);
        } else {
            w.resize(w.x(), w.y(), w.w(), 250);
        }
        b.set_pos((w.w() - b.w()) / 2, (w.h() - b.h()) / 2);
    }
}

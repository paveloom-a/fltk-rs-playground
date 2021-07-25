use fltk::{
    app, button::Button, enums::FrameType, frame::Frame, group::Group, prelude::*, window::Window,
};

fn main() {
    let app = app::App::default();
    let mut w = Window::new(100, 100, 250, 250, "");

    let gh = 40;
    let mut g = Group::default().with_pos(10, 10).with_size(w.w() - 20, gh);
    g.set_frame(FrameType::BorderBox);

    let bw = 60;
    let bh = 30;
    let mut b = Button::default()
        .with_pos(g.x() + (g.w() - bw) / 2, g.y() + (g.h() - bh) / 2)
        .with_size(bw, bh)
        .with_label("Hide");
    b.set_callback(resize);

    g.end();

    let mut f = Frame::default()
        .with_pos(10, 10 + gh + 10)
        .with_size(w.w() - 20, w.h() - 10 - gh - 10 - 10);
    f.set_frame(FrameType::BorderBox);

    w.show();
    app.run().unwrap();
}

fn resize(b: &mut Button) {
    if let Some(ref g) = b.parent() {
        if let Some(ref mut w) = g.parent() {
            if let Some(ref mut f) = w.child(1) {
                if w.h() == 250 {
                    w.resize(w.x(), w.y(), w.w(), 10 + g.h() + 10);
                    b.set_label("Show");
                    f.hide();
                } else {
                    w.resize(w.x(), w.y(), w.w(), 250);
                    b.set_label("Hide");
                    f.show();
                }
            }
        }
    }
}

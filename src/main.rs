use fltk::{
    app, button::Button, enums::FrameType, frame::Frame, group::Group, prelude::*, window::Window,
};

fn main() {
    let app = app::App::default();

    let ww = 250;
    let wh = 250;
    let mut w = Window::new(100, 100, ww, wh, "");
    w.size_range(ww, wh, ww, wh + 100);

    let gh = 40;
    let mut bg = Group::default().with_pos(10, 10).with_size(w.w() - 20, gh);
    bg.set_frame(FrameType::BorderBox);

    let bw = 60;
    let bh = 30;
    let mut b = Button::default()
        .with_pos(bg.x() + (bg.w() - bw) / 2, bg.y() + (bg.h() - bh) / 2)
        .with_size(bw, bh)
        .with_label("Hide");
    b.set_callback(resize);

    bg.end();

    let mut f = Frame::default()
        .with_pos(10, 10 + gh + 10)
        .with_size(w.w() - 20, w.h() - 10 - gh - 10 - 10);
    f.set_frame(FrameType::BorderBox);

    w.resizable(&f);
    w.show();
    app.run().unwrap();
}

fn resize(b: &mut Button) {
    if let Some(ref g) = b.parent() {
        if let Some(ref mut w) = g.parent() {
            if let Some(ref mut s) = w.child(1) {
                let gh = 10 + g.h() + 10;
                if w.h() == gh {
                    w.resize(w.x(), w.y(), w.w(), 250);
                    b.set_label("Hide");
                    s.show();
                } else {
                    w.resize(w.x(), w.y(), w.w(), gh);
                    b.set_label("Show");
                    s.hide();
                }
            }
        }
    }
}

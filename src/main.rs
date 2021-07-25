use fltk::{
    app,
    button::Button,
    draw,
    enums::{Color, FrameType},
    frame::Frame,
    group::{Group, Pack},
    prelude::*,
    window::Window,
};

mod widgets;
use widgets::list;

fn main() {
    let app = app::App::default();
    let mut w = Window::new(100, 100, 250, 250, "");

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

    let mut lg = Pack::default()
        .with_pos(10, 10 + gh + 10)
        .with_size(w.w() - 20, w.h() - 10 - gh - 10 - 10);
    lg.set_frame(FrameType::BorderFrame);

    let mut l = list::List::default();
    l.set_size(0, lg.h());
    l.add("Hello there!");

    lg.end();

    w.show();
    app.run().unwrap();
}

fn resize(b: &mut Button) {
    if let Some(ref g) = b.parent() {
        if let Some(ref mut w) = g.parent() {
            if let Some(ref mut s) = w.child(1) {
                if w.h() == 250 {
                    w.resize(w.x(), w.y(), w.w(), 10 + g.h() + 10);
                    b.set_label("Show");
                    s.hide();
                } else {
                    w.resize(w.x(), w.y(), w.w(), 250);
                    b.set_label("Hide");
                    s.show();
                }
            }
        }
    }
}

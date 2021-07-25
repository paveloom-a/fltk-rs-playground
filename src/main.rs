use fltk::{
    app, button::Button, enums::FrameType, frame::Frame, group::Group, prelude::*, window::Window,
};

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

    let mut fg = Group::default()
        .with_pos(10, 10 + gh + 10)
        .with_size(w.w() - 20, w.h() - 10 - gh - 10 - 10);
    fg.set_frame(FrameType::BorderBox);

    let mut f = MyFrame::default()
        .with_pos(10, 10)
        .with_size(fg.w() - 20, fg.h() - 20);
    f.set_frame(FrameType::BorderBox);

    fg.end();

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

struct MyFrame {
    frame: Frame,
}

impl MyFrame {
    fn default() -> Self {
        MyFrame {
            frame: Frame::default(),
        }
    }

    fn with_pos(mut self, x: i32, y: i32) -> Self {
        if let Some(ref p) = self.frame.parent() {
            self.frame.set_pos(p.x() + x, p.y() + y);
        }
        self
    }

    fn with_size(mut self, width: i32, height: i32) -> Self {
        self.frame.set_size(width, height);
        self
    }

    fn set_frame(&mut self, typ: FrameType) {
        self.frame.set_frame(typ);
    }
}

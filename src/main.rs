use fltk::{app, group::Group, enums::FrameType, button::Button, prelude::*, window::Window};

fn main() {
    let app = app::App::default();

    app::background(255, 255, 255);
    app::set_frame_type(FrameType::FlatBox);

    let mut w = Window::new(100, 100, 400, 300, "Focus Box Test");

    let g = Group::default().with_size(180, 190).center_of(&w);

    let h = 40;

    let _b1 = Button::new(g.x(), g.y(), 60, h, "60");
    let _b2 = Button::new(g.x(), g.y() + h + 10, 100, h, "100");
    let _b3 = Button::new(g.x(), g.y() + 2 * h + 10, 140, h, "140");
    let _b4 = Button::new(g.x(), g.y() + 3 * h + 10, 180, h, "180");

    g.end();

    w.end();
    w.show();
    app.run().unwrap();
}
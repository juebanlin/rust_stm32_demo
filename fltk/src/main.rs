use fltk::window::{Window, DoubleWindow};
use fltk::prelude::{WidgetBase, WidgetExt, WindowExt};
use fltk::app;

fn main() {
    let app = app::App::default();
    app::set_visible_focus(false);
    app::background(0x42, 0x42, 0x42);

    let win_w = 400;
    let win_h = 500;
    let but_row = 160;

    let mut wind = Window::default()
        .with_label("FLTK")
        .with_size(win_w, win_h)
        .center_screen();
    let mut wind2 = Window::default() as DoubleWindow;
    wind2.with_label("FLTK2")
        .center_screen();
    let mut wind3=wind2 as &dyn WidgetBase;
    wind3.with_label("FLTK3");
}

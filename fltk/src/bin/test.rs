use fltk::{
    app,
    button::Button,
    enums::{Align, Color, Event, FrameType, Key, Shortcut},
    frame::Frame,
    group::{Pack, PackType},
    prelude::*,
    window::Window,
};
use std::ops::{Deref, DerefMut};
use fltk::window::DoubleWindow;

fn main() {
    let mut wind = Window::default();
    wind.with_label("FLTK")
        .center_screen();
}


#[test]
fn test(){
    let a=100f32.ln();
    let b=100f32.log10();
    let c=100f32.log(10f32);
    let x=a+b+c;
    println!("{},{},{}",a,b,c);
}


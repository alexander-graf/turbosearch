use fltk::{
    app::App,
    button::Button,
    input::{Input, InputType},
    prelude::{WidgetExt, GroupExt},
    window::Window,
};

fn main() {
    // Startvariablen für counter
    let mut x = 0;
    let mut y = 0;

    let app = App::default();
    let mut win = Window::default()
        .with_size(600, 400)
        .with_label("FLTK GUI");

    let mut input1 = Input::default()
        .with_size(100, 40)
        .with_pos(10, 10)
        .with_label("Input 1");

    let mut input2 = Input::default()
        .with_size(100, 40)
        .with_pos(10, 60)
        .with_label("Input 2");

    let mut button1 = Button::default()
        .with_size(100, 30)
        .with_pos(10, 120)
        .with_label("Button 1");
    let original_label1 = button1.label();
    button1.set_callback(move |b| {
        x += 1;
        b.set_label(&x.to_string());
    });

    let mut button2 = Button::default()
        .with_size(100, 30)
        .with_pos(10, 160)
        .with_label("Button 2");
    let original_label2 = button2.label();
    button2.set_callback(move |b| {
        y += 1;
        b.set_label(&y.to_string());
    });
use fltk::{
    app::App,
    button::Button,
    input::{Input, InputType},
    prelude::{WidgetExt, GroupExt},
    window::Window,
};

fn main() {
    let app = App::default();
    let mut win = Window::default()
        .with_size(600, 400)
        .with_label("FLTK GUI");

    let mut input1 = Input::default()
        .with_size(100, 40)
        .with_pos(10, 10)
        .with_label("Input 1");

    let mut input2 = Input::default()
        .with_size(100, 40)
        .with_pos(10, 60)
        .with_label("Input 2");

    let mut button1 = Button::default()
        .with_size(100, 30)
        .with_pos(10, 120)
        .with_label("Button 1");
    let original_label1 = button1.label().to_string();
    button1.set_callback(move |b| {
        if b.label() == "Pressed" {
            b.set_label(&original_label1);
        } else {
            b.set_label("Pressed");
        }
    });

    let mut button2 = Button::default()
        .with_size(100, 30)
        .with_pos(10, 160)
        .with_label("Button 2");
    let original_label2 = button2.label().to_string();
    button2.set_callback(move |b| {
        if b.label() == "Pressed" {
            b.set_label(&original_label2);
        } else {
            b.set_label("Pressed");
        }
    });

    win.end();
    win.show();

    app.run().unwrap();
}
    win.end();
    win.show();

    app.run().unwrap();
}
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

    let mut button2 = Button::default()
        .with_size(100, 30)
        .with_pos(10, 160)
        .with_label("Button 2");

    let mut input3 = Input::default()
        .with_size(100, 40)
        .with_pos(200, 10)
        .with_label("Input 3");

    let mut input4 = Input::default()
        .with_size(100, 40)
        .with_pos(200, 60)
        .with_label("Input 4");

    let mut button3 = Button::default()
        .with_size(100, 30)
        .with_pos(200, 120)
        .with_label("Button 3");

    let mut button4 = Button::default()
        .with_size(100, 30)
        .with_pos(200, 160)
        .with_label("Button 4");

    win.end();
    win.show();

    app.run().unwrap();
}
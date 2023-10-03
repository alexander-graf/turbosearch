use fltk::{app, button::Button, dialog::FileDialog, frame::Frame, input::Input, prelude::*, window::Window};
use std::fs;
use fltk::enums::FrameType;

fn main() {
    let app = app::App::default();
    let mut window = Window::new(100, 100, 600, 400, "File Search GUI");
    window.make_resizable(true);

    let mut input = Input::new(10, 10, 300, 30, "");
    let mut button = Button::new(320, 10, 100, 30, "Search");
    let mut frame = Frame::new(10, 50, 580, 340, "");
    frame.set_frame(FrameType::BorderFrame);

    button.set_callback(move |_| {
        let search_string = input.value();
        let found_files = search_files(search_string);
        frame.set_label(&found_files.join("\n"));
    });

    window.end();
    window.show();

    app.run().unwrap();
}

fn search_files(search_string: &str) -> Vec<String> {
    let current_dir = std::env::current_dir().unwrap();
    let mut found_files = vec![];

    for entry in fs::read_dir(current_dir).unwrap() {
        if let Ok(entry) = entry {
            let file_name = entry.file_name().into_string().unwrap();
            if file_name.contains(search_string) {
                found_files.push(file_name);
            }
        }
    }

    found_files
}
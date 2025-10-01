mod util;
use std::io;
use std::env::args;
use std::thread::sleep;
use std::time::Duration;
use crossterm::style::Color;
use crate::util::buffer::Buffer;
use crate::util::init::init;
use crate::util::gui::{draw_gui, GuiSettings};

fn main() {
    let mut args = args();
    let filename: String;

    if args.len() > 2 {
        panic!("Too many arguments");
    } else if args.len() == 1 {
        filename = String::from("");
    } else {
        filename = args.nth(1).unwrap();
    }

    let mut buffer: Buffer = Buffer {
        buffer: Vec::new()
    };

    let mut gui_settings: GuiSettings = GuiSettings {
        title_color:                Color::Rgb { r: 0, g: 0, b: 0},
        title_background_color:     Color::Rgb { r: 255, g: 255, b: 255},
        text_color:                 Color::Rgb { r: 255, g: 255, b: 255},
        highlight_text_color:       Color::Rgb { r: 0, g: 0, b: 0},
        highlight_background_color: Color::Rgb { r: 255, g: 255, b: 255}
    };

    init(&mut buffer, &mut gui_settings, filename.as_str());

    loop {
        draw_gui(&gui_settings);
        sleep(Duration::from_millis(10));
    }
}

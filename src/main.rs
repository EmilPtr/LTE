//! LTE - Lightweight Text Editor
//! Terminal-based text editor built with Rust.

mod util;

use std::io;
use std::env::args;
use std::thread::sleep;
use std::time::Duration;
use crossterm::event::DisableMouseCapture;
use crossterm::execute;
use crossterm::style::Color;
use crossterm::terminal::{disable_raw_mode, LeaveAlternateScreen};
use crate::util::buffer::Buffer;
use crate::util::init::init;
use crate::util::gui::{draw_gui, GuiSettings};

/// Main function: parses args, sets up buffer and GUI, runs editor loop
fn main() {
    // Parse command line arguments
    let mut args = args();
    let filename: String;

    if args.len() > 2 {
        panic!("Too many arguments");
    } else if args.len() == 1 {
        filename = String::from("");
    } else {
        filename = args.nth(1).unwrap();
    }

    // Initialize buffer and GUI settings
    let mut buffer = Buffer {
        buffer: Vec::new(),
        file: String::new()
    };

    let mut gui_settings = GuiSettings {
        title_color:                Color::Rgb { r: 0,   g: 0,   b: 0},
        title_background_color:     Color::Rgb { r: 255, g: 255, b: 255},
        text_color:                 Color::Rgb { r: 255, g: 255, b: 255},
        highlight_text_color:       Color::Rgb { r: 0,   g: 0,   b: 0},
        highlight_background_color: Color::Rgb { r: 255, g: 255, b: 255}
    };

    // Initialize terminal and load file
    init(&mut buffer, &mut gui_settings, filename.as_str());

    // Main rendering loop
    for _ in 0..500 {
        draw_gui(&gui_settings, &buffer);
        sleep(Duration::from_millis(10));
    }

    // Cleanup terminal
    disable_raw_mode();
    execute!(io::stdout(), LeaveAlternateScreen);
    execute!(io::stdout(), DisableMouseCapture);
}

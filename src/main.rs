//! LTE - Lightweight Text Editor
//! Terminal-based text editor built with Rust.

mod util;

use std::io;
use std::env::args;
use std::io::Write;
use std::thread::sleep;
use std::time::Duration;
use crossterm::event::{read, DisableMouseCapture};
use crossterm::{execute, queue};
use crossterm::cursor::MoveTo;
use crossterm::style::Color;
use crossterm::terminal::{disable_raw_mode, LeaveAlternateScreen};
use crate::util::buffer::{handle_event, move_cursor, Buffer};
use crate::util::cursor::Cursor;
use crate::util::init::init;
use crate::util::gui::{clear, draw_buffer, draw_gui, GuiSettings, LINE_NUMBER_WIDTH, TITLE_BAR_HEIGHT};

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

    let mut cursor = Cursor {
        x: 0,
        y: 0,
        real_x: 0+LINE_NUMBER_WIDTH,
        real_y: 0+TITLE_BAR_HEIGHT
    };

    let mut gui_settings = GuiSettings {
        title_color:                Color::Rgb { r: 0,   g: 0,   b: 0   },
        title_background_color:     Color::Rgb { r: 255, g: 255, b: 255 },
        text_color:                 Color::Rgb { r: 255, g: 255, b: 255 },
        highlight_text_color:       Color::Rgb { r: 0,   g: 0,   b: 0   },
        highlight_background_color: Color::Rgb { r: 255, g: 255, b: 255 },
        line_number_color:          Color::Rgb { r: 192, g: 192, b: 192 }
    };

    // Initialize terminal and load file
    init(&mut buffer, &mut gui_settings, filename.as_str());
    draw_gui(&gui_settings, &buffer);
    draw_buffer(&gui_settings, &buffer);
    queue!(io::stdout(), MoveTo(cursor.real_x, cursor.real_y)).unwrap();
    io::stdout().flush().unwrap();

    // Main event loop
    for _ in 0..50 {
        let event = read().unwrap();
        handle_event(event, &mut cursor, &mut buffer);
        clear();
        draw_gui(&gui_settings, &buffer);
        draw_buffer(&gui_settings, &buffer);
        move_cursor(&cursor);
        io::stdout().flush().unwrap();
    }

    // Cleanup terminal
    disable_raw_mode();
    execute!(io::stdout(), LeaveAlternateScreen);
    execute!(io::stdout(), DisableMouseCapture);
}

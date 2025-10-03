//! GUI rendering for the terminal interface

use std::io;
use crossterm::cursor::MoveTo;
use crossterm::{ExecutableCommand, QueueableCommand};
use crossterm::style::{Attribute, Color, Print, SetBackgroundColor, SetForegroundColor};
use crossterm::terminal::{size, Clear, ClearType};
use crate::util::buffer::Buffer;

/// Color settings for the GUI
pub struct GuiSettings {
    pub title_color:                Color,
    pub title_background_color:     Color,
    pub text_color:                 Color,
    pub highlight_text_color:       Color,
    pub highlight_background_color: Color,
    pub line_number_color:          Color,
}

pub const TITLE_BAR_HEIGHT: u16 = 1;
pub const LINE_NUMBER_WIDTH: u16 = 6;

/// Draw the GUI (currently just title bar)
pub fn draw_gui(settings: &GuiSettings, buffer: &Buffer) {
    let mut stdout = io::stdout();
    stdout.queue(MoveTo(0, 0)).unwrap();
    stdout.queue(SetBackgroundColor(settings.title_background_color)).unwrap();
    stdout.queue(SetForegroundColor(settings.title_color)).unwrap();
    stdout.queue(Print(title_string(buffer.file.clone()))).unwrap();
    stdout.queue(MoveTo(0, size().unwrap().1-1)).unwrap();
    stdout.queue(Print(footer())).unwrap();
}

pub fn draw_buffer(settings: &GuiSettings, buffer: &Buffer) {
    let mut stdout = io::stdout();
    stdout.queue(SetBackgroundColor(Color::Reset)).unwrap();
    stdout.queue(SetForegroundColor(settings.text_color)).unwrap();
    stdout.queue(MoveTo(0, 1)).unwrap();
    let mut line_number = 1;
    for line in buffer.buffer.iter() {
        stdout.queue(SetForegroundColor(settings.line_number_color)).unwrap();
        stdout.queue(MoveTo(0, line_number)).unwrap();
        stdout.queue(Print(format!("{:3} ┃ ", line_number))).unwrap();
        stdout.queue(SetForegroundColor(settings.text_color)).unwrap();
        stdout.queue(Print(line)).unwrap();
        line_number += 1;
    }
}

/// Clear the terminal screen
pub fn clear() {
    let mut stdout = io::stdout();
    stdout.execute(Clear(ClearType::All)).unwrap();
}

/// Create title bar string with editor name and filename
fn title_string(file: String) -> String {
    let mut title = String::from("LTE V0.1");
    let title_len   = title.len() as u16;
    let file_len    = file.len() as u16;
    for _i in 0..(size().unwrap().0-title_len-file_len) {
        title.push(' ');
    }
    title.push_str(file.as_str());
    title
}

fn footer() -> String {
    String::from("CTRL+C: Copy    CTRL+V: Paste    CTRL+S: Save    CTRL+X: Exit")
}
use std::io;
use std::io::Write;
use crossterm::cursor::MoveTo;
use crossterm::{ExecutableCommand, QueueableCommand};
use crossterm::style::{Attribute, Color, Print, SetBackgroundColor, SetForegroundColor};
use crossterm::terminal::{size, Clear, ClearType};

pub struct GuiSettings {
    pub title_color:                Color,
    pub title_background_color:     Color,
    pub text_color:                 Color,
    pub highlight_text_color:       Color,
    pub highlight_background_color: Color,
}

pub fn draw_gui(settings: &GuiSettings) {
    let mut stdout = io::stdout();
    stdout.queue(MoveTo(0, 0)).unwrap();
    stdout.queue(SetBackgroundColor(settings.title_background_color)).unwrap();
    stdout.queue(SetForegroundColor(settings.title_color)).unwrap();
    stdout.queue(Print(title_string())).unwrap();
    stdout.flush().unwrap();
}

pub fn clear() {
    let mut stdout = io::stdout();
    stdout.execute(Clear(ClearType::All)).unwrap();
}

fn title_string() -> String {
    let mut title = String::from("LTE");
    for i in 0..(size().unwrap().0-3) {
        title.push(' ');
    }
    title
}
use std::io::Stdout;
use crossterm::terminal::{
    enable_raw_mode,
    EnterAlternateScreen
};
use crossterm::execute;
use crossterm::event::EnableMouseCapture;
use crate::util::buffer::Buffer;
use crate::util::gui_settings::GuiSettings;

/// Initializes crossterm, enabling raw mode, entering alternate screen, and enabling the mouse.
fn init_crossterm(mut stdout: Stdout) {
    match enable_raw_mode() {
        Ok(_) => (),
        Err(e) => panic!("Failed to enable raw mode: {}", e)
    }
    match execute!(stdout, EnterAlternateScreen) {
        Ok(_) => (),
        Err(e) => panic!("Failed to enter alternate screen: {}", e)
    }
    match execute!(stdout, EnableMouseCapture) {
        Ok(_) => (),
        Err(e) => println!("Failed to enable mouse capture: {}", e)
    }
}

/// Initializes the buffer, reads from the file or leaves it empty
fn init_buffer(buffer: &mut Buffer, filename : &str) {
    buffer.buffer.push(String::from("Hello, world!"));
    // TODO: read from filename in pwd
}

fn init_gui(gui_settings: &mut GuiSettings) {
    // TODO: make the GUI customizable
}

/// Initializes everything, including crossterm, the buffer, and the GUI
pub fn init(stdout: Stdout, buffer: &mut Buffer, gui_settings: &mut GuiSettings, filename : &str) {
    init_crossterm(stdout);
    init_buffer(buffer, filename);
    init_gui(gui_settings);
}
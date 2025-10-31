//! Initialization functions for setting up the editor

use std::io;
use std::io::Stdout;
use crossterm::terminal::{
    enable_raw_mode,
    EnterAlternateScreen
};
use crossterm::execute;
use crossterm::event::EnableMouseCapture;
use crate::util::buffer::Buffer;
use crate::util::file_operations::load_file;
use crate::util::gui::GuiSettings;

/// Sets up terminal: raw mode, alternate screen, mouse capture
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

/// Load file into buffer (currently just adds placeholder text)
fn init_buffer(buffer: &mut Buffer, path : &str) {
    buffer.file = String::from(path);
    load_file(path, buffer);
}

/// Set up GUI customizations (currently does nothing)
fn init_gui(_gui_settings: &mut GuiSettings) {
    // TODO: make the GUI customizable
}

/// Main init function: sets up terminal, buffer, and GUI
pub fn init(buffer: &mut Buffer, gui_settings: &mut GuiSettings, filename : &str) {
    init_crossterm(io::stdout());
    init_buffer(buffer, filename);
    init_gui(gui_settings);
}

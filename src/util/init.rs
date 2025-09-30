use std::io::Stdout;
use crossterm::terminal::{
    enable_raw_mode,
    EnterAlternateScreen
};
use crossterm::{execute};

pub fn init_crossterm(mut stdout: Stdout) {
    match enable_raw_mode() {
        Ok(_) => (),
        Err(e) => panic!("Failed to enable raw mode: {}", e),
    };
    match execute!(stdout, EnterAlternateScreen) {
        Ok(_) => (),
        Err(e) => panic!("Failed to enter alternate screen: {}", e),
    }
}

pub fn init(stdout: Stdout) {
    init_crossterm(stdout);
}
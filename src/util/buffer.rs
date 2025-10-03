//! Text buffer for storing file content

use std::io::Write;
use crossterm::cursor::MoveTo;
use crossterm::event::{Event, KeyCode};
use crossterm::QueueableCommand;
use crate::util::cursor::Cursor;

/// Stores text content as lines and tracks filename
pub struct Buffer {
    pub buffer: Vec<String>,  // Each string is one line
    pub file: String          // Filename
}

pub fn handle_event(event: Event, cursor: &mut Cursor , buffer: &mut Buffer) {
    match event {
        Event::Key(key_event) => {
            handle_keypress(key_event.code, cursor, buffer);
        },
        Event::Mouse(_) => {
            handle_mouse(event, cursor, buffer);
        },
        Event::Paste(str) => {
            handle_paste(str, cursor, buffer);
        },
        _ => {}
    }
}

fn handle_keypress(key: KeyCode, c: &mut Cursor, b: &mut Buffer) {
    let mut stdout = std::io::stdout();
    match key {
        KeyCode::Up => {
            c.move_u();
            stdout.queue(MoveTo(c.real_x, c.real_y)).unwrap();
        },
        KeyCode::Down => {
            c.move_d();
            stdout.queue(MoveTo(c.real_x, c.real_y)).unwrap();
        },
        KeyCode::Left => {
            c.move_l();
            stdout.queue(MoveTo(c.real_x, c.real_y)).unwrap();
        },
        KeyCode::Right => {
            c.move_r();
            stdout.queue(MoveTo(c.real_x, c.real_y)).unwrap();
        },
        _ => {
            stdout.queue(MoveTo(c.real_x, c.real_y)).unwrap();
        }
    }
}

fn handle_mouse(event: Event, cursor: &mut Cursor, buffer: &mut Buffer) {

}

fn handle_paste(contents: String, cursor: &mut Cursor, buffer: &mut Buffer) {

}


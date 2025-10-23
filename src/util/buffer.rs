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

pub fn move_cursor(c: &Cursor) {
    std::io::stdout().queue(MoveTo(c.real_x as u16, c.real_y as u16)).unwrap();
}

fn handle_keypress(key: KeyCode, c: &mut Cursor, b: &mut Buffer) {
    let mut stdout = std::io::stdout();
    match key {
        KeyCode::Up => {
            c.move_u();
            c.clamp_x(b.buffer[c.y].len());
            stdout.queue(MoveTo(c.real_x as u16, c.real_y as u16)).unwrap();
        },
        KeyCode::Down => {
            c.move_d();
            c.clamp_y((b.buffer.len()-1));
            c.clamp_x(b.buffer[c.y].len());
            stdout.queue(MoveTo(c.real_x as u16, c.real_y as u16)).unwrap();
        },
        KeyCode::Left => {
            c.move_l();
            stdout.queue(MoveTo(c.real_x as u16, c.real_y as u16)).unwrap();
        },
        KeyCode::Right => {
            c.move_r();
            c.clamp_x(b.buffer[c.y].len());
            stdout.queue(MoveTo(c.real_x as u16, c.real_y as u16)).unwrap();
        },
        KeyCode::Backspace => {
            if c.x != 0 {
                b.buffer[c.y].remove(c.x - 1);
            } else if c.y != 0 {
                let temp_str = b.buffer[c.y].clone(); // Clone the current line
                b.buffer.remove(c.y); // remove the current line
                c.move_u(); // Move the cursor up one line
                c.set_position(b.buffer[c.y].len()+1, c.y); // Set the cursor position to the end of the new line
                b.buffer[c.y].push_str(&temp_str); // Add the old line back to the current line
            }
            c.move_l();
            stdout.queue(MoveTo(c.real_x as u16, c.real_y as u16)).unwrap();
        },
        KeyCode::Enter => {
            if c.x == b.buffer[c.y].len() {
                b.buffer.insert(c.y +1, String::new());
                c.move_d();
                c.clamp_x(b.buffer[c.y].len());
            } else {
                let temp_str = b.buffer[c.y][c.x..b.buffer[c.y].len()].to_string();
                let original_str_splice = b.buffer[c.y][0 ..c.x].to_string();
                b.buffer[c.y] = original_str_splice;
                b.buffer.insert(c.y +1, String::from(temp_str));
                c.move_d();
                c.clamp_x(0);
            }
            stdout.queue(MoveTo(c.real_x as u16, c.real_y as u16)).unwrap();
        },
        KeyCode::Char(char) => {
            b.buffer[c.y].insert(c.x, char);
            c.move_r();
            stdout.queue(MoveTo(c.real_x as u16, c.real_y as u16)).unwrap();
        }
        _ => {
        }
    }
}

fn handle_mouse(event: Event, cursor: &mut Cursor, buffer: &mut Buffer) {

}

fn handle_paste(contents: String, cursor: &mut Cursor, buffer: &mut Buffer) {

}


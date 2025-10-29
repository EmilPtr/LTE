//! Text buffer for storing file content

use std::io;
use std::process::exit;
use crossterm::cursor::MoveTo;
use crossterm::event::{read, DisableMouseCapture, Event, KeyCode, KeyEvent, KeyModifiers};
use crossterm::{execute, QueueableCommand};
use crossterm::terminal::{disable_raw_mode, LeaveAlternateScreen};
use crate::util::buffer::Buffer;
use crate::util::cursor::Cursor;

pub fn handle_event(event: Event, cursor: &mut Cursor , buffer: &mut Buffer) {
    match event {
        Event::Key(key_event) => {
            handle_keypress(key_event, cursor, buffer);
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

fn handle_keypress(key_event: KeyEvent, c: &mut Cursor, b: &mut Buffer) {
    let mut stdout = std::io::stdout();
    match (key_event.code, key_event.modifiers) {
        (KeyCode::Up, KeyModifiers::NONE) => {
            c.move_u();
            c.clamp_x(b.buffer[c.y].len());
        },
        (KeyCode::Down, KeyModifiers::NONE) => {
            c.move_d();
            c.clamp_y((b.buffer.len()-1));
            c.clamp_x(b.buffer[c.y].len());
        },
        (KeyCode::Left, KeyModifiers::NONE) => {
            c.move_l();
        },
        (KeyCode::Right, KeyModifiers::NONE) => {
            c.move_r();
            c.clamp_x(b.buffer[c.y].len());
        },
        (KeyCode::Backspace, KeyModifiers::NONE) => {
            b.handle_backspace(c);
        },
        (KeyCode::Enter, KeyModifiers::NONE) => {
            b.handle_enter(c);
        },
        (KeyCode::Char(char), KeyModifiers::NONE) => {
            b.handle_char_insert(c, char);
        },
        (KeyCode::Char(char), KeyModifiers::CONTROL) => {
            if char.to_ascii_lowercase() == 'x' {
                disable_raw_mode().unwrap();
                execute!(io::stdout(), LeaveAlternateScreen).unwrap();
                execute!(io::stdout(), DisableMouseCapture).unwrap();
                exit(0);
            }
        },
        _ => {
        }
    }
}

fn handle_mouse(event: Event, cursor: &mut Cursor, buffer: &mut Buffer) {

}

fn handle_paste(contents: String, cursor: &mut Cursor, buffer: &mut Buffer) {

}


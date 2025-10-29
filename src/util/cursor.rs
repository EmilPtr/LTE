use crossterm::cursor::MoveTo;
use crossterm::QueueableCommand;
use crate::util::gui::{LINE_NUMBER_WIDTH, TITLE_BAR_HEIGHT};

pub struct Cursor {
    pub x: usize,
    pub y: usize,
    pub real_x: usize,
    pub real_y: usize,
}

impl Cursor {
    pub fn set_position(&mut self, x: usize, y: usize) {
        self.x = x;
        self.y = y;
        self.real_x = x+LINE_NUMBER_WIDTH;
        self.real_y = y+TITLE_BAR_HEIGHT;
    }

    pub fn move_u(&mut self) {
        self.y = self.y.saturating_sub(1);
        self.set_position(self.x, self.y);
    }

    pub fn move_d(&mut self) {
        self.y += 1;
        self.set_position(self.x, self.y);
    }

    pub fn move_l(&mut self) {
        self.x = self.x.saturating_sub(1);
        self.set_position(self.x, self.y);
    }

    pub fn move_r(&mut self) {
        self.x += 1;
        self.set_position(self.x, self.y);
    }

    pub fn clamp_x(&mut self, max: usize) {
        if self.x > max {
            self.x = max;
            self.set_position(self.x, self.y);
        }
    }

    pub fn clamp_y(&mut self, max: usize) {
        if self.y > max {
            self.y = max;
            self.set_position(self.x, self.y);
        }
    }
    
    pub fn move_cursor(&self) {
        std::io::stdout().queue(MoveTo(self.real_x as u16, self.real_y as u16)).unwrap();
    }
}
use crate::util::gui::{LINE_NUMBER_WIDTH, TITLE_BAR_HEIGHT};

pub struct Cursor {
    pub x: u16,
    pub y: u16,
    pub real_x: u16,
    pub real_y: u16,
}

impl Cursor {
    pub fn set_position(&mut self, x: u16, y: u16) {
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

    pub fn clamp_x(&mut self, max: u16) {
        if self.x > max {
            self.x = max;
            self.set_position(self.x, self.y);
        }
    }

    pub fn clamp_y(&mut self, max: u16) {
        if self.y > max {
            self.y = max;
            self.set_position(self.x, self.y);
        }
    }
}
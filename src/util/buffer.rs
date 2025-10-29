use crate::util::cursor::Cursor;

/// Stores text content as lines and tracks filename
pub struct Buffer {
    pub buffer: Vec<String>,  // Each string is one line
    pub file: String          // Filename
}

impl Buffer {
    pub fn handle_backspace(&mut self, c: &mut Cursor) {
        if c.x != 0 {
            self.buffer[c.y].remove(c.x - 1);
        } else if c.y != 0 {
            let temp_str = self.buffer[c.y].clone(); // Clone the current line
            self.buffer.remove(c.y); // remove the current line
            c.move_u(); // Move the cursor up one line
            c.set_position(self.buffer[c.y].len()+1, c.y); // Set the cursor position to the end of the new line
            self.buffer[c.y].push_str(&temp_str); // Add the old line back to the current line
        }
        c.move_l();
    }

    pub fn handle_enter(&mut self, c: &mut Cursor) {
        if c.x == self.buffer[c.y].len() {
            self.buffer.insert(c.y +1, String::new());
            c.move_d();
            c.clamp_x(self.buffer[c.y].len());
        } else {
            let temp_str = self.buffer[c.y][c.x..self.buffer[c.y].len()].to_string();
            let original_str_splice = self.buffer[c.y][0 ..c.x].to_string();
            self.buffer[c.y] = original_str_splice;
            self.buffer.insert(c.y +1, String::from(temp_str));
            c.move_d();
            c.clamp_x(0);
        }
    }

    pub fn handle_char_insert(&mut self, c: &mut Cursor, char: char) {
        self.buffer[c.y].insert(c.x, char);
        c.move_r();
    }
}
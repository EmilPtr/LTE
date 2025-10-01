//! Text buffer for storing file content

/// Stores text content as lines and tracks filename
pub struct Buffer {
    pub buffer: Vec<String>,  // Each string is one line
    pub file: String          // Filename
}

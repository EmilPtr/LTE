mod util;
use std::io;
use crate::util::init::init;

fn main() {
    let stdout = io::stdout();
    init(stdout);
}

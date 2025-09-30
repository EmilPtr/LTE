mod util;
use std::io;
use std::env::args;
use crate::util::buffer::Buffer;
use crate::util::init::init;
use crate::util::gui_settings::GuiSettings;

fn main() {
    let mut args = args();
    let filename: String;

    if args.len() > 2 {
        panic!("Too many arguments");
    } else if args.len() == 1 {
        filename = String::from("");
    } else {
        filename = args.nth(1).unwrap();
    }

    let stdout = io::stdout();

    let mut buffer: Buffer = Buffer {
        buffer: Vec::new()
    };

    let mut gui_settings: GuiSettings = GuiSettings {
        title_color: 0x000000,
        title_background_color: 0xffffff,
        text_color: 0xffffff,
        highlight_text_color: 0x000000,
        highlight_background_color: 0xffffff
    };

    init(stdout, &mut buffer, &mut gui_settings, filename.as_str());
}

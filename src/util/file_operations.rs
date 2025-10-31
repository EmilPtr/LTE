use std::fs::File;
use std::io::Read;
use std::path::Path;
use crate::util::buffer::Buffer;

pub fn load_file(path: &str, b: &mut Buffer) {
    let path = Path::new(path);
    let mut file = match File::open(path) {
        Err(why) => {

            let new_file;
            if why.kind() == std::io::ErrorKind::NotFound {
                new_file = match File::create(path) {
                    Err(why) => panic!("couldn't create {}: {}", path.display(), why),
                    Ok(_) => match File::open(path) {
                        Err(why) => panic!("couldn't open {}: {}", path.display(), why),
                        Ok(file) => file
                    },
                };
            } else {
                panic!("couldn't open {}: {}", path.display(), why);
            }

            new_file
        },
        Ok(file) => file
    };

    if file.metadata().unwrap().len() == 0 {
        b.buffer.push(String::from(""));
    }

    let mut contents = String::new();
    match file.read_to_string(&mut contents) {
        Err(why) => panic!("couldn't read {}: {}", path.display(), why),
        Ok(_) => (),
    }

    for line in contents.lines() {
        b.buffer.push(String::from(line));
    }
}
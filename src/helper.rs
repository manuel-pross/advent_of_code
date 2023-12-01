use std::fs::File;
use std::io::{self, Read};

pub fn read_file(path: String) -> io::Result<String> {
    println!("{}", path);
    let mut file = File::open(path)?;

    let mut contents = String::new();
    file.read_to_string(&mut contents)?;

    Ok(contents)
}

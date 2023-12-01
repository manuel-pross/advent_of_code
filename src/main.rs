use std::io;

mod helper;
use helper::read_file;

fn main() -> io::Result<()> {
    let current_dir = std::env::current_dir().expect("Failed");

    let file_path = String::from(current_dir.to_string_lossy() + "/src/data/day1.txt");

    match read_file(file_path) {
        Ok(contents) => {
            // Print the contents of the file
            println!("File Contents:\n{}", contents);
            Ok(())
        }
        Err(e) => {
            eprintln!("Error reading file: {}", e);
            Err(e)
        }
    }
}


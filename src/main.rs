use std::fs;
use std::io;
use std::path::Path;
use std::process::exit;

// Restrict access to root directory, handle unreadable directories
fn main() {
    let mut path = String::new();
    io::stdin().read_line(&mut path).expect("could not read");
    let path = path.trim();
    // check if the directory is readable
    let check: bool = Path::new(&path).is_dir();
    if check {
        println!("failure");
        exit(-1);
    }
    match fs::File::open(path) {
        Ok(_s) => println!("success"),
        Err(_e) => println!("failure"),
    };
}

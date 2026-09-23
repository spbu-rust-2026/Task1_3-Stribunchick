use std::fs;
use std::io;
use std::path::Path;

// Restrict access to root directory, handle unreadable directories
fn main() {
    let mut path = String::new();
    match io::stdin().read_line(&mut path) {
        Ok(_) => {}
        Err(_e) => {
            println!("failure");
            return;
        }
    }
    let path = path.trim();
    // check if the directory is readable
    let check: bool = Path::new(&path).is_dir();
    if check {
        println!("failure");
        return;
    }
    match fs::File::open(path) {
        Ok(_s) => println!("success"),
        Err(_e) => println!("failure"),
    };
}

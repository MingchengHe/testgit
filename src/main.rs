use std::fs::File;
use std::io;
use std::io::BufRead;
use walkdir::WalkDir;

fn main() {
    for entry in WalkDir::new(".").into_iter().filter_map(Result::ok) {
        println!("{}", entry.path().display());
        if let Ok(file) = File::open(&entry.path()) {
            println!("true")
        }
    }
}

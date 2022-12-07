use std::io::{BufReader, BufRead};
use std::fs::File;
use std::path::Path;

/// Read lines into a file. Adapted from the Rust by Example book
/// https://doc.rust-lang.org/rust-by-example/std_misc/file/read_lines.html
/// 
/// # Panics
/// If any IO error occurs.
pub fn read_lines<P>(filename: P) -> impl Iterator<Item = String> where P: AsRef<Path> {
    let file = File::open(filename).expect("File doesn't exist");
    BufReader::new(file).lines().map(|x| x.expect("Couldn't unwrap line"))
}

#[macro_export]
macro_rules! input {
    () => {
        $crate::read_lines(concat!("inputs/", env!("CARGO_BIN_NAME"), ".input"))
    };
}

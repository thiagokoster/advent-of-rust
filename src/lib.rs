use std::fs::File;
use std::io::{self, BufRead};

pub fn get_lines(path: &str) -> io::Lines<io::BufReader<File>> {
    let file = File::open(path).expect("Should be able to open file");
    io::BufReader::new(file).lines()
}

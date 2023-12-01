use std::fs::File;
use std::io::{self, BufRead};

pub fn get_lines(path: &str) -> io::Lines<io::BufReader<File>> {
    let file = open_file(path);
    io::BufReader::new(file).lines()
}

pub fn read_file(path: &str) -> Vec<String> {
    let mut result: Vec<String> = Vec::new();
    let file = open_file(path);
    for line in io::BufReader::new(file).lines() {
        result.push(line.expect("should be able to read line"));
    }

    result
}

fn open_file(path: &str) -> File {
    File::open(path).expect("Should be able to open file")
}

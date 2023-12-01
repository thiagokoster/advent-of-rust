use std::str::Chars;

use crate::day::Solution;
use advent_of_rust::read_file;

pub struct Day01;
pub const BASEPATH: &str = "src/year2023/day01/";

impl Solution for Day01 {
    fn part_1(&self, input: &str) -> String {
        let path = BASEPATH.to_owned() + input;
        let input = read_file(&path);
        let mut result = 0;
        for line in input {
            let left_digit = line.chars().find(|e| e.is_numeric());
            let right_digit = line.chars().rev().find(|e| e.is_numeric());

            let mut number = String::new();
            number.push(left_digit.unwrap());
            number.push(right_digit.unwrap());

            let number = number.parse::<u32>().unwrap();
            result += number;
        }
        result.to_string()
    }

    fn part_2(&self, input: &str) -> String {
        "".to_string()
    }
}

fn first_int(chars: Chars) {
    for c in chars {
        if c.is_numeric() {}
    }
}

#[cfg(test)]
mod tests {
    use super::{Day01, Solution};

    #[test]
    fn test_part_1() {
        let day = Day01 {};
        assert_eq!(day.part_1("example.txt"), "142");
    }
}

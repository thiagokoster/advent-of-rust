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
        let valid_digits = [
            "one", "two", "three", "four", "five", "six", "seven", "eight", "nine", "1", "2", "3",
            "4", "5", "6", "7", "8", "9",
        ];
        let path = BASEPATH.to_owned() + input;
        let lines = read_file(&path);
        let mut result = 0;

        for line in lines {
            let mut number = String::new();
            number.push(first_digit(&line, &valid_digits));
            number.push(last_digit(&line, &valid_digits));
            result += number.parse::<u32>().unwrap();
        }
        result.to_string()
    }
}

fn first_digit(input: &str, valid_digits: &[&str]) -> char {
    let mut first = (usize::MAX, "");
    for digit in valid_digits {
        if !input.contains(digit) {
            continue;
        }

        let m = input.match_indices(digit).next().unwrap();
        if m.0 < first.0 {
            first = m;
        }
    }
    return convert(first.1);
}

fn last_digit(input: &str, valid_digits: &[&str]) -> char {
    let mut last = (0, "");
    for digit in valid_digits {
        if !input.contains(digit) {
            continue;
        }

        let m = input.match_indices(digit).last().unwrap();
        if m.0 >= last.0 {
            last = m;
        }
    }
    return convert(last.1);
}

fn convert(digit: &str) -> char {
    match digit {
        "one" => '1',
        "two" => '2',
        "three" => '3',
        "four" => '4',
        "five" => '5',
        "six" => '6',
        "seven" => '7',
        "eight" => '8',
        "nine" => '9',
        _ => digit.chars().next().unwrap(),
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

    #[test]
    fn test_part_2() {
        let day = Day01 {};
        assert_eq!(day.part_2("example2.txt"), "281");
    }
}

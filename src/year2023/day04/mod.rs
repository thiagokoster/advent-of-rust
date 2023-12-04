use std::collections::HashSet;

use crate::day::Solution;
use advent_of_rust::read_file;

pub struct Day04;
pub const BASEPATH: &str = "src/year2023/day04/";

#[derive(Debug)]
struct Card {
    winning_numbers: HashSet<u16>,
    numbers: HashSet<u16>,
    points: u32,
}

impl Card {
    pub fn from_str(line: &str) -> Self {
        let colon_index = line.find(":").expect("line should contain a ':'");
        let line = &line[colon_index + 1..].trim();

        let line = line.replace("  ", " ");
        println!("Line: {}", line);

        let (left, right) = line.split_once(" | ").expect("line should contain a '|'");

        let winning_numbers: HashSet<u16> =
            left.split(" ").map(|x| x.parse::<u16>().unwrap()).collect();
        let numbers: HashSet<u16> = right
            .split(" ")
            .map(|x| x.parse::<u16>().unwrap())
            .collect();

        let hits: u32 = winning_numbers
            .intersection(&numbers)
            .count()
            .try_into()
            .unwrap();
        let points = if hits > 0 { 2u32.pow(hits - 1) } else { 0 };

        Self {
            winning_numbers,
            numbers,
            points,
        }
    }
}

impl Solution for Day04 {
    fn part_1(&self, input: &str) -> String {
        let path = BASEPATH.to_owned() + input;
        let lines = read_file(&path);

        let mut result = 0;

        for line in lines {
            let card = Card::from_str(&line);
            result += card.points;
        }
        result.to_string()
    }

    fn part_2(&self, input: &str) -> String {
        "".to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_1() {
        let day = Day04 {};
        let result = day.part_1("example.txt");
        assert_eq!(result, "13");
    }
}

use std::collections::HashSet;

use crate::day::Solution;
use advent_of_rust::read_file;

pub struct Day05;
pub const BASEPATH: &str = "src/year2023/day05/";

impl Solution for Day05 {
    fn part_1(&self, input: &str) -> String {
        let path = BASEPATH.to_owned() + input;
        let lines = read_file(&path);
        for line in lines {
            println!("{}", line);
        }
        "".to_string()
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
        let day = Day05 {};
        let result = day.part_1("example.txt");
        assert_eq!(result, "35");
    }
}

use crate::day::Solution;
use advent_of_rust::read_file;
use regex::Regex;

pub struct Day02;
pub const BASEPATH: &str = "src/year2023/day02/";

struct Bag {
    red: u32,
    blue: u32,
    green: u32,
}

impl Bag {
    pub fn impossible_game(&self, game: &str) -> bool {
        let red = Bag::check_stone(game, "red", self.red);
        let blue = Bag::check_stone(game, "blue", self.blue);
        let green = Bag::check_stone(game, "green", self.green);

        return red || blue || green;
    }

    fn check_stone(game: &str, color: &str, max: u32) -> bool {
        let pattern = format!(r"(\d*) {}", color);
        let re = Regex::new(&pattern).unwrap();

        for c in re.captures_iter(game) {
            let amount = c.get(1).unwrap().as_str().parse::<u32>().unwrap();

            if amount > max {
                return true;
            }
        }

        false
    }
}

impl Solution for Day02 {
    fn part_1(&self, input: &str) -> String {
        let bag = Bag {
            red: 12,
            green: 13,
            blue: 14,
        };
        let path = BASEPATH.to_owned() + input;
        let lines = read_file(&path);

        let mut result = 0;
        for (id, game) in lines.iter().enumerate() {
            if !bag.impossible_game(&game) {
                result += id + 1;
            }
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
        let day02 = Day02 {};
        assert_eq!(day02.part_1("example.txt"), "8");
    }
}

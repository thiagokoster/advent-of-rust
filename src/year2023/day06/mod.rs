use crate::day::Solution;
use advent_of_rust::read_file;
use regex::Regex;
pub struct Day06;
pub const BASEPATH: &str = "src/year2023/day06/";

impl Solution for Day06 {
    fn part_1(&self, input: &str) -> String {
        let path = BASEPATH.to_owned() + input;
        let lines = read_file(&path);
        let races = parse_input(lines);
        let mut result = 1;
        for race in races {
            let n = race.part_1();
            println!("{}", n);
            result *= n;
        }

        result.to_string()
    }

    fn part_2(&self, input: &str) -> String {
        "".to_string()
    }
}

struct Race {
    time: u32,
    distance: u32,
}

impl Race {
    fn part_1(&self) -> usize {
        // Math time...
        // velocity(v) increases linearly with holding time (h) => v = h
        // Distance (d) = velocity (v) * time at velocity (t) => d = v * t
        // time at velocity (t) = Total time (T) - holding time (h) t = T - h
        // Replacing the values we get the followind equation: h^2 - T.h + d = 0
        // If we replace T with the total time, and d with the opponent distance, the answer will
        // be the integer numbers between the roots of the equation.
        let b: f32 = self.time as f32;
        let c: f32 = self.distance as f32;

        let root1 = (b - (b.powf(2f32) - 4f32 * c).powf(1.0 / 2.0)) / 2.0;
        let root2 = (b + (b.powf(2f32) - 4f32 * c).powf(1.0 / 2.0)) / 2.0;

        let start = root1.floor() as u32 + 1;
        let end = root2.ceil() as u32;
        (start..end).count()
    }
}

fn parse_input(lines: Vec<String>) -> Vec<Race> {
    let mut races: Vec<Race> = Vec::new();
    let re = Regex::new(r"(\d+)").unwrap();
    let times: Vec<u32> = re
        .captures_iter(&lines[0])
        .filter_map(|c| c[1].parse().ok())
        .collect();

    let distances: Vec<u32> = re
        .captures_iter(&lines[1])
        .filter_map(|c| c[1].parse().ok())
        .collect();

    for i in 0..times.len() {
        races.push(Race {
            time: times[i],
            distance: distances[i],
        });
    }

    races
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_1() {
        let day = Day06 {};
        let result = day.part_1("example.txt");
        assert_eq!(result, "288");
    }
}

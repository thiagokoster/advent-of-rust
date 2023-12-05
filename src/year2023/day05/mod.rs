use std::collections::HashSet;

use crate::day::Solution;
use advent_of_rust::read_file;

pub struct Day05;
pub const BASEPATH: &str = "src/year2023/day05/";

impl Solution for Day05 {
    fn part_1(&self, input: &str) -> String {
        let path = BASEPATH.to_owned() + input;
        let lines = read_file(&path);
        parse_input(&lines);
        "".to_string()
    }
    fn part_2(&self, input: &str) -> String {
        "".to_string()
    }
}

struct Seed {
    id: String,
    soil: String,
}

#[derive(Debug)]
struct Map {
    destination_start: u32,
    source_start: u32,
    length: u32,
}

impl Map {
    pub fn from_str(line: &str) -> Self {
        let ids: Vec<u32> = line
            .splitn(3, " ")
            .map(|i| i.parse::<u32>().unwrap())
            .collect();
        Map {
            destination_start: ids[0],
            source_start: ids[1],
            length: ids[2],
        }
    }

    pub fn get_destination(&self, source_id: u32) -> Option<u32> {
        if source_id >= self.source_start && source_id <= self.source_start + self.length {
            let diff = source_id - self.source_start;
            return Some(self.destination_start + diff);
        }
        None
    }
}

fn parse_input(lines: &Vec<String>) {
    let colon_index = lines[0].find(":").expect("line should contain a ':'");
    let seeds = lines[0][colon_index + 1..].trim();
    let seeds: Vec<u32> = seeds
        .split(" ")
        .map(|n| n.parse::<u32>().unwrap())
        .collect();
    for seed in seeds {
        for line in lines[2..].iter() {
            if line.contains(":") || line.is_empty() {
                println!("Empty or description line");
                continue;
            }
            let map = Map::from_str(&line);
            if let Some(destination) = map.get_destination(seed) {
                println!(
                    "Map {:?} contains seed {}. Destination: {}",
                    map, seed, destination
                );
            }
            println!("{}", line);
        }

        break;
        !todo!("debugging just the first seed for now");
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

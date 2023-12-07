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

struct Maps {
    maps: Vec<Map>,
}

impl Maps {
    pub fn find_destination(&self, source: u32) -> u32 {
        for map in &self.maps {
            if let Some(destination) = map.get_destination(source) {
                return destination;
            }
        }
        // if does not find in all maps, the destination has the same id
        source
    }
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

    let seed_to_soil = get_tag("seed-to-soil", lines);
    let soil_to_fertilizer = get_tag("soil-to-fertilizer", lines);
    let fertilizer_to_water = get_tag("fertilizer-to-water", lines);
    let water_to_light = get_tag("water-to-light", lines);
    let light_to_temperature = get_tag("light-to-temperature", lines);
    let temperature_to_humidity = get_tag("temperature-to-humidity", lines);
    let humidity_to_location = get_tag("humidity-to-location", lines);

    let mut min_location = u32::MAX;
    for seed in seeds {
        let soil = seed_to_soil.find_destination(seed);
        let fertilizer = soil_to_fertilizer.find_destination(soil);
        let water = fertilizer_to_water.find_destination(fertilizer);
        let light = water_to_light.find_destination(water);
        let temperature = light_to_temperature.find_destination(light);
        let humidity = temperature_to_humidity.find_destination(temperature);
        let location = humidity_to_location.find_destination(humidity);

        min_location = min_location.min(location);
    }

    println!("{}", min_location.to_string());
}

fn get_tag(map: &str, input: &Vec<String>) -> Maps {
    let map = map.to_owned() + " map:";
    let mut maps: Vec<Map> = Vec::new();
    let mut tag_idx = 0;
    if let Some((index, _)) = input
        .into_iter()
        .enumerate()
        .find(|&(_, s)| s == map.as_str())
    {
        tag_idx = index + 1;
    }

    for i in tag_idx..input.len() {
        if input[i].is_empty() {
            break;
        }

        maps.push(Map::from_str(&input[i]));
    }

    Maps { maps }
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

use crate::day::Solution;
use advent_of_rust::read_file;

pub struct Day05;
pub const BASEPATH: &str = "src/year2023/day05/";

impl Solution for Day05 {
    fn part_1(&self, input: &str) -> String {
        let path = BASEPATH.to_owned() + input;
        let lines = read_file(&path);
        let (seeds, map) = parse_input(&lines);

        let mut min_location = u64::MAX;
        for seed in seeds {
            let location = map.find_seed_location(seed);
            min_location = min_location.min(location);
        }

        min_location.to_string()
    }
    fn part_2(&self, input: &str) -> String {
        "".to_string()
    }
}

struct Map {
    seed_to_soil: Vec<MapEntry>,
    soil_to_fertilizer: Vec<MapEntry>,
    fertilizer_to_water: Vec<MapEntry>,
    water_to_light: Vec<MapEntry>,
    light_to_temperature: Vec<MapEntry>,
    temperature_to_humidity: Vec<MapEntry>,
    humidity_to_location: Vec<MapEntry>,
}

impl Map {
    fn find_seed_location(&self, seed: u64) -> u64 {
        let soil = Map::get_destination_internal(seed, &self.seed_to_soil);
        let fertilizer = Map::get_destination_internal(soil, &self.soil_to_fertilizer);
        let water = Map::get_destination_internal(fertilizer, &self.fertilizer_to_water);
        let light = Map::get_destination_internal(water, &self.water_to_light);
        let temperature = Map::get_destination_internal(light, &self.light_to_temperature);
        let humidity = Map::get_destination_internal(temperature, &self.temperature_to_humidity);
        let location = Map::get_destination_internal(humidity, &self.humidity_to_location);

        location
    }

    fn get_destination_internal(id: u64, maps: &Vec<MapEntry>) -> u64 {
        for map in maps {
            if let Some(destination) = map.get_destination(id) {
                return destination;
            }
        }
        // if does not find in all maps, the destination has the same id
        id
    }
}

#[derive(Debug)]
struct MapEntry {
    destination_start: u64,
    source_start: u64,
    length: u64,
}

impl MapEntry {
    pub fn from_str(line: &str) -> Self {
        let ids: Vec<u64> = line
            .splitn(3, " ")
            .map(|i| i.parse::<u64>().unwrap())
            .collect();
        MapEntry {
            destination_start: ids[0],
            source_start: ids[1],
            length: ids[2],
        }
    }

    pub fn get_destination(&self, source_id: u64) -> Option<u64> {
        if source_id >= self.source_start && source_id <= self.source_start + self.length {
            let diff = source_id - self.source_start;
            return Some(self.destination_start + diff);
        }
        None
    }
}

fn parse_input(lines: &Vec<String>) -> (Vec<u64>, Map) {
    let colon_index = lines[0].find(":").expect("line should contain a ':'");
    let seeds = lines[0][colon_index + 1..].trim();
    let seeds: Vec<u64> = seeds
        .split(" ")
        .map(|n| n.parse::<u64>().unwrap())
        .collect();

    let seed_to_soil = get_tag("seed-to-soil", lines);
    let soil_to_fertilizer = get_tag("soil-to-fertilizer", lines);
    let fertilizer_to_water = get_tag("fertilizer-to-water", lines);
    let water_to_light = get_tag("water-to-light", lines);
    let light_to_temperature = get_tag("light-to-temperature", lines);
    let temperature_to_humidity = get_tag("temperature-to-humidity", lines);
    let humidity_to_location = get_tag("humidity-to-location", lines);

    let map = Map {
        seed_to_soil,
        soil_to_fertilizer,
        fertilizer_to_water,
        water_to_light,
        light_to_temperature,
        temperature_to_humidity,
        humidity_to_location,
    };

    (seeds, map)
}

fn get_tag(map: &str, input: &Vec<String>) -> Vec<MapEntry> {
    let tag = map.to_owned() + " map:";
    let mut entries: Vec<MapEntry> = Vec::new();
    let mut tag_idx = 0;
    if let Some((index, _)) = input
        .into_iter()
        .enumerate()
        .find(|&(_, s)| s == tag.as_str())
    {
        tag_idx = index + 1;
    }

    for i in tag_idx..input.len() {
        if input[i].is_empty() {
            break;
        }

        entries.push(MapEntry::from_str(&input[i]));
    }

    entries
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

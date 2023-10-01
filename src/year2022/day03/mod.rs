pub mod day03 {
    use std::collections::HashMap;

    use crate::day::Solution;
    use advent_of_rust::get_lines;
    pub struct Day03;

    impl Day03 {
        pub fn find_duplicated(rucksack: &str) -> char {
            let compartments = rucksack.split_at(rucksack.len()/2);
            for item in compartments.0.chars() {
                if compartments.1.contains(item) {
                    return item;
                }
            }
                unreachable!();
        }

        pub fn priority(item : char) -> Option<u32> {
            match item {
                'a'..='z' => Some(item as u32 - 'a' as u32 + 1),
                'A'..='Z' => Some(item as u32 - 'A' as u32 + 27),
                _ => None 
                
            }
        }
    }

    impl Solution for Day03 {

        fn part_1(&self, input :&str) -> u32 {
            let mut total_priority = 0;
            let path = "src/year2022/day03/".to_owned() + input;
            if let Ok(lines) = get_lines(&path) {
                for line in lines {
                    if let Ok(content) = line {
                        let duplicated_item = Day03::find_duplicated(&content);
                        total_priority += Day03::priority(duplicated_item).unwrap_or(0);
                    }
                }
            }
            return total_priority;
        }

        fn part_2(&self, input :&str) -> u32 {
            return 0;
        }

        

    }
}

#[cfg(test)]
mod tests {
    use crate::{year2022::day03::day03::Day03, day::Solution};

    #[test]
    fn test_part_1(){
        assert_eq!(Day03.part_1("example.txt"), 157);
    }
    #[test]
    fn test_find_duplicated(){
        let rucksack = "vJrwpWtwJgWrhcsFMMfFFhFp";
        let duplicated_item = Day03::find_duplicated(rucksack);
        assert_eq!('p', duplicated_item);
    }

    #[test]
    fn test_priority() {
        assert_eq!(Day03::priority('a'), Some(1));
        assert_eq!(Day03::priority('z'), Some(26));
        assert_eq!(Day03::priority('A'), Some(27));
        assert_eq!(Day03::priority('Z'), Some(52));
    }

}

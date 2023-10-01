pub mod day03 {

    use std::{io, collections::HashSet};
    use crate::day::Solution;
    use advent_of_rust::get_lines;
    pub struct Day03;

    impl Day03 {
        pub fn find_duplicated(comp_1: &str, comp_2: &str) -> String {
            let first = HashSet::<char>::from_iter(comp_1.chars());
            let second = HashSet::<char>::from_iter(comp_2.chars());
            
            return first.intersection(&second).into_iter().collect();
        }

        pub fn find_badge(rucksack0: &str, rucksack1: &str, rucksack2: &str) -> char {
            let item = Day03::find_duplicated(&rucksack0, &rucksack1);

            return Day03::find_duplicated(&rucksack2, &item)
                .chars()
                .next()
                .expect("Should have only one badge")
        }

        pub fn priority(item: char) -> Option<u32> {
            match item {
                'a'..='z' => Some(item as u32 - 'a' as u32 + 1),
                'A'..='Z' => Some(item as u32 - 'A' as u32 + 27),
                _ => None,
            }
        }

        fn get_elf(line: Option<Result<String, io::Error>>) -> String {
            if let Some(content) = line {
                if let Ok(elf) = content {
                    return elf;
                } else {
                    unreachable!()
                }
            }
            unreachable!()
        }
    }

    impl Solution for Day03 {
        fn part_1(&self, input: &str) -> u32 {
            let mut total_priority = 0;
            let path = "src/year2022/day03/".to_owned() + input;
            if let Ok(lines) = get_lines(&path) {
                for line in lines {
                    if let Ok(content) = line {
                        let compartments = content.split_at(content.len() / 2);
                        let duplicated_item = Day03::find_duplicated(&compartments.0, &compartments.1);
                        total_priority += Day03::priority(duplicated_item.chars().next().unwrap()).unwrap_or(0);
                    }
                }
            }
            return total_priority;
        }

        fn part_2(&self, input: &str) -> u32 {
            let mut total_priority = 0;
            let path = "src/year2022/day03/".to_owned() + input;
            if let Ok(mut lines) = get_lines(&path) {
                while let Some(line) = lines.next() {
                    let elf0 = Day03::get_elf(Some(line));
                    let elf1 = Day03::get_elf(lines.next());
                    let elf2 = Day03::get_elf(lines.next());

                    let badge = Day03::find_badge(&elf0, &elf1, &elf2);
                    total_priority += Day03::priority(badge).unwrap_or(0);
                }
            }
            return total_priority;
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{day::Solution, year2022::day03::day03::Day03};

    #[test]
    fn test_part_1() {
        assert_eq!(Day03.part_1("example.txt"), 157);
    }

    #[test]
    fn test_part_2() {
        assert_eq!(Day03.part_2("example.txt"), 70);
    }

    #[test]
    fn test_find_duplicated() {
        let rucksack = "vJrwpWtwJgWrhcsFMMfFFhFp";
        let comps = rucksack.split_at(rucksack.len()/2);
        let duplicated_item = Day03::find_duplicated(comps.0, comps.1);
        assert_eq!('p', duplicated_item.chars().next().unwrap());
    }

    #[test]
    fn test_priority() {
        assert_eq!(Day03::priority('a'), Some(1));
        assert_eq!(Day03::priority('z'), Some(26));
        assert_eq!(Day03::priority('A'), Some(27));
        assert_eq!(Day03::priority('Z'), Some(52));
    }

    #[test]
    fn test_find_badge() {
        assert_eq!(
            Day03::find_badge(
                "vJrwpWtwJgWrhcsFMMfFFhFp",
                "jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL",
                "PmmdzqPrVvPwwTWBwg"
            ),
            'r'
        );
    }
}

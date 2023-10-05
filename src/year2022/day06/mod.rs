pub mod day06 {
    use std::collections::HashSet;

    use crate::day::Solution;
    use advent_of_rust::get_lines;
    pub struct Day06;
    impl Solution for Day06 {
        fn part_1(&self, input: &str) -> String {
            let path = "src/year2022/day06/".to_owned() + input;
            let mut lines = get_lines(&path);
            let first_line = lines.next().expect("should be able to read line").unwrap();

            return Device::find_marker(&first_line, 4).to_string();
        }

        fn part_2(&self, input: &str) -> String {
            let path = "src/year2022/day06/".to_owned() + input;
            let mut lines = get_lines(&path);
            let first_line = lines.next().expect("should be able to read line").unwrap();

            return Device::find_marker(&first_line, 14).to_string();
        }
    }

    struct Device;
    impl Device {
        fn find_marker(signal: &str, size: usize) -> u32 {
            for i in 0..signal.len() - size - 1 {
                let buffer: &str = &signal[i..i + size];
                if has_unique(buffer) {
                    return (i + size) as u32;
                }
            }
            unreachable!()
        }
    }

    fn has_unique(input: &str) -> bool {
        let mut char_set = HashSet::new();

        for c in input.chars() {
            if !char_set.insert(c) {
                return false;
            }
        }
        true
    }
}

#[cfg(test)]
mod tests {
    use crate::day::Solution;

    use super::day06::Day06;

    #[test]
    fn test_part_1() {
        assert_eq!(Day06.part_1("example.txt"), "7");
    }

    #[test]
    fn test_part_2() {
        //assert_eq!(Day06.part_2("example.txt"), "19");
    }
}

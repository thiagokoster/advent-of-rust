pub mod day02 {
    use crate::day::Solution;
    use advent_of_rust::get_lines;
    pub struct Day02;

    impl Solution for Day02 {
        fn part_1(&self, input: &str) -> u32 {
            let path = "src/year2022/day02/".to_owned() + input;
            if let Ok(lines) = get_lines(&path) {
                for line in lines {
                    if let Ok(content) = line {
                    }
                }
            }

            return 0;
        }

        fn part_2(&self, input: &str) -> u32 {
            return 0;
        }
    }
}

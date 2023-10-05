pub mod day04 {
    use advent_of_rust::get_lines;

    use crate::day::Solution;
    pub struct Day04;
    impl Solution for Day04 {
        fn part_1(&self, input: &str) -> String {
            let path = "src/year2022/day04/".to_owned() + input;
            let mut lines = get_lines(&path);
            let mut count = 0;
            while let Some(line) = lines.next() {
                if let Ok(content) = line {
                    let (assignment1, assignment2) = content
                        .split_once(',')
                        .expect("should have two assignments");
                    let assignment1 = Assignment::from_str(assignment1);
                    let assignment2 = Assignment::from_str(assignment2);

                    if assignment1.fully_contains(&assignment2)
                        || assignment2.fully_contains(&assignment1)
                    {
                        count += 1;
                    }
                }
            }

            return count.to_string();
        }

        fn part_2(&self, input: &str) -> String {
            let path = "src/year2022/day04/".to_owned() + input;
            let mut lines = get_lines(&path);
            let mut count = 0;
            while let Some(line) = lines.next() {
                if let Ok(content) = line {
                    let (assignment1, assignment2) = content
                        .split_once(',')
                        .expect("should have two assignments");
                    let assignment1 = Assignment::from_str(assignment1);
                    let assignment2 = Assignment::from_str(assignment2);

                    if assignment1.overlaps(&assignment2) || assignment2.overlaps(&assignment1) {
                        count += 1;
                    }
                }
            }

            return count.to_string();
        }
    }

    struct Assignment {
        start: u32,
        end: u32,
    }

    impl Assignment {
        pub fn from_str(input: &str) -> Self {
            let (start, end) = input.split_once('-').unwrap();
            let start = start.parse::<u32>().unwrap();
            let end = end.parse::<u32>().unwrap();
            return Self { start, end };
        }

        fn fully_contains(&self, other: &Assignment) -> bool {
            return self.start <= other.start && self.end >= other.end;
        }

        fn overlaps(&self, other: &Assignment) -> bool {
            return self.end >= other.start && self.start <= other.end;
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{day::Solution, year2022::day04::day04::Day04};

    #[test]
    fn test_part_1() {
        assert_eq!(Day04.part_1("example.txt"), "2");
    }

    #[test]
    fn test_part_2() {
        assert_eq!(Day04.part_2("example.txt"), "4");
    }
}

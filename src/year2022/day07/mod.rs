pub mod day07 {

    use advent_of_rust::get_lines;

    use crate::day::Solution;
    pub struct Day07;

    impl Solution for Day07 {
        fn part_1(&self, input: &str) -> String {
            let path = "src/year2022/day07/".to_owned() + input;
            let directories = parse(&path);

            let result = directories
                .iter()
                .filter(|&&size| size < 100_000)
                .sum::<u32>();

            return result.to_string();
        }

        fn part_2(&self, input: &str) -> String {
            let path = "src/year2022/day07/".to_owned() + input;
            let directories = parse(&path);
            let root = directories.iter().max().unwrap();
            let required = root + 30_000_000 - 70_000_000;

            let result = directories
                .iter()
                .filter(|&&size| size >= required)
                .min()
                .unwrap();

            return result.to_string();
        }
    }

    fn parse(path: &str) -> Vec<u32> {
        let mut tmp_dir: Vec<u32> = Vec::new();
        let mut directories: Vec<u32> = Vec::new();
        let mut iter = get_lines(&path);
        while let Some(line) = iter.next() {
            let content = line.unwrap();

            match content.split(' ').collect::<Vec<&str>>().as_slice() {
                ["$", "cd", ".."] => directories.push(tmp_dir.pop().unwrap()),
                ["$", "cd", _] => tmp_dir.push(0),
                [size, _] => {
                    if let Ok(num) = size.parse::<u32>() {
                        tmp_dir.iter_mut().for_each(|n| *n += num)
                    }
                }
                [..] => unreachable!(),
            }
        }
        directories.extend(tmp_dir);
        directories
    }
}
#[cfg(test)]
mod tests {
    use crate::{day::Solution, year2022::day07::day07::Day07};

    #[test]
    fn test_part_1() {
        assert_eq!(Day07.part_1("example.txt"), "95437")
    }

    #[test]
    fn test_part_2() {
        assert_eq!(Day07.part_2("example.txt"), "24933642")
    }
}

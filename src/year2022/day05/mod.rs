pub mod day05 {
    use std::collections::{HashMap, VecDeque};

    use advent_of_rust::get_lines;

    use crate::day::Solution;
    use regex::Regex;

    pub struct Day05;
    impl Solution for Day05 {
        fn part_1(&self, input: &str) -> String {
            let mut output = String::new();
            let path = "src/year2022/day05/".to_owned() + input;
            let (mut stacks, commands) = parse_input(&path);

            for command in commands {
                command.move_crate(&mut stacks)
            }

            for i in 1..=stacks.keys().len() {
                output.push_str(stacks.get(&i).unwrap().last().unwrap());
            }

            return output.to_string();
        }

        fn part_2(&self, input: &str) -> String {
            let mut output = String::new();
            let path = "src/year2022/day05/".to_owned() + input;
            let (mut stacks, commands) = parse_input(&path);

            for command in commands {
                command.move_crate_9001(&mut stacks)
            }

            for i in 1..=stacks.keys().len() {
                output.push_str(stacks.get(&i).unwrap().last().unwrap());
            }

            return output.to_string();
        }
    }

    fn parse_input(input: &str) -> (HashMap<usize, Vec<String>>, Vec<Command>) {
        let mut lines = get_lines(&input);
        let mut stacks: HashMap<usize, Vec<String>> = HashMap::new();
        let mut commands: Vec<Command> = Vec::new();

        while let Some(line_result) = lines.next() {
            let line = line_result.unwrap();
            if !line.starts_with("move") {
                // parse stacks
                let re = Regex::new(r"\[(\w)\]").unwrap();
                for captures in re.captures_iter(&line) {
                    if let Some(_crate) = captures.get(1) {
                        let index = _crate.start() / 4 + 1;
                        stacks
                            .entry(index)
                            .and_modify(|v| v.insert(0, _crate.as_str().to_string()))
                            .or_insert(vec![_crate.as_str().to_string()]);
                    }
                }
            } else {
                // parse commands
                if let Some(command) = Command::from_str(line) {
                    commands.push(command);
                }
            }
        }

        return (stacks, commands);
    }

    #[derive(Debug)]
    struct Command {
        times: u32,
        from: usize,
        to: usize,
    }

    impl Command {
        fn from_str(input: String) -> Option<Self> {
            let re = Regex::new(r"move (?<times>\d*) from (?<from>\d*) to (?<to>\d*)")
                .expect("should build move regex");
            if let Some(captures) = re.captures(&input) {
                let times = captures
                    .name("times")
                    .unwrap()
                    .as_str()
                    .parse::<u32>()
                    .unwrap();
                let from = captures
                    .name("from")
                    .unwrap()
                    .as_str()
                    .parse::<usize>()
                    .unwrap();
                let to = captures
                    .name("to")
                    .unwrap()
                    .as_str()
                    .parse::<usize>()
                    .unwrap();

                return Some(Command { times, from, to });
            }

            None
        }

        fn move_crate(&self, stacks: &mut HashMap<usize, Vec<String>>) {
            for _ in 0..self.times {
                let get = stacks.get_mut(&self.from).unwrap().pop().unwrap();
                stacks.entry(self.to).and_modify(|f| f.push(get));
            }
        }

        fn move_crate_9001(&self, stacks: &mut HashMap<usize, Vec<String>>) {
            let mut pick: VecDeque<String> = VecDeque::new();

            for _ in 0..self.times {
                let get = stacks.get_mut(&self.from).unwrap().pop().unwrap();
                pick.push_back(get);
            }

            while !pick.is_empty() {
                stacks
                    .entry(self.to)
                    .and_modify(|f| f.push(pick.pop_back().unwrap()));
            }
        }
    }
}
#[cfg(test)]
mod tests {
    use super::day05::Day05;
    use crate::day::Solution;

    #[test]
    fn test_part_1() {
        let result = Day05.part_1("example.txt");
        assert_eq!(result, "CMZ");
    }

    #[test]
    fn test_part_2() {
        let result = Day05.part_2("example.txt");
        assert_eq!(result, "MCD");
    }
}

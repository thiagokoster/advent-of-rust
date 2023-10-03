pub mod day05 {
    use std::collections::HashMap;

    use advent_of_rust::get_lines;

    use crate::day::Solution;
    use regex::Regex;

    //TODO: part_1 and part_2 needs to be String
    pub struct Day05;
    impl Solution for Day05 {
        fn part_1(&self, input: &str) -> u32 {
            let path = "src/year2022/day05/".to_owned() + input;
            let (stacks, commands) = parse_input(&path);

            for stack in stacks {
                println!("{:?}", stack);
            }
            //TODO: Get dictionary keys in order OR check the size of the stacks list first and
            //create an vector for it. (maybe this is better)
            return 0;
        }

        fn part_2(&self, input: &str) -> u32 {
            return 0;
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
                    command.move_crate(&mut stacks);
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
                println!("from: {}, to: {}", self.from, self.to);
                let get = stacks.get_mut(&self.from).unwrap().pop().unwrap();
                stacks.entry(self.to).and_modify(|f| f.push(get));

                for stack in &mut *stacks {
                    println!("{:?}", stack);
                }
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
    }
}

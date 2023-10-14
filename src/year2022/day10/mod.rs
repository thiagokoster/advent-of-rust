pub mod day10 {
    use advent_of_rust::get_lines;

    use crate::day::Solution;

    pub struct Day10;

    pub const BASEPATH: &str = "src/year2022/day10/";

    impl Solution for Day10 {
        fn part_1(&self, input: &str) -> String {
            let system = execute_system(&input);
            return system.signal.to_string();
        }

        fn part_2(&self, input: &str) -> String {
            let system = execute_system(&input);
            return system.display;
        }
    }
    #[derive(Default)]
    struct System {
        reg_x: i32,
        cycle: u32,
        signal: i32,
        display: String,
    }

    fn execute_system(input: &str) -> System {
        let path = BASEPATH.to_owned() + input;
        let mut iter = get_lines(&path);
        let mut system = System {
            reg_x: 1,
            cycle: 1,
            ..Default::default()
        };
        while let Some(line) = iter.next() {
            let content = line.expect("Should be able to read line");
            system.execute(&content);
        }

        system
    }

    impl System {
        fn execute(&mut self, input: &str) {
            let operation = Operation::from_str(&input);

            for _ in 0..operation.delay {
                if self.cycle >= 20 && (self.cycle - 20) % 40 == 0 {
                    let strength = self.reg_x * self.cycle as i32;
                    self.signal += strength;
                }

                if (self.cycle - 1) % 40 == 0 {
                    self.display.push('\n');
                }
                self.display.push(self.draw());
                self.cycle += 1;
            }

            match operation.command {
                Command::Addx(x) => self.reg_x += x,
                Command::Noop => (),
            }
        }

        fn draw(&self) -> char {
            let c = (self.cycle - 1) % 40;
            if c.abs_diff(self.reg_x as u32) < 2 {
                return '#';
            }
            '.'
        }
    }

    struct Operation {
        delay: u32,
        command: Command,
    }

    impl Operation {
        pub fn from_str(input: &str) -> Self {
            let args = input.split_once(" ");
            match args {
                Some((_, arg2)) => Operation {
                    delay: 2,
                    command: Command::Addx(arg2.parse().expect("should parse to int")),
                },
                None => Operation {
                    delay: 1,
                    command: Command::Noop,
                },
            }
        }
    }
    enum Command {
        Noop,
        Addx(i32),
    }
}

#[cfg(test)]
mod test {
    use crate::day::Solution;

    use super::day10::Day10;

    #[test]
    fn test_part_1() {
        println!("");
        let result = Day10.part_1("example.txt");
        assert_eq!(result, "13140")
    }
}

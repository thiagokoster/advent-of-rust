pub mod day11 {
    use std::collections::VecDeque;

    use advent_of_rust::get_lines;
    use regex::Regex;

    use crate::day::Solution;

    pub struct Day11;

    pub const BASEPATH: &str = "src/year2022/day11/";

    impl Solution for Day11 {
        fn part_1(&self, input: &str) -> String {
            let mut monkeys = parse(input);
            let rounds = 20;

            for _ in 0..rounds {
                for idx in 0..monkeys.len() {
                    while let Some(mut item) = monkeys[idx].inspect() {
                        item = item / 3;

                        let target = monkeys[idx].test(item);

                        //throw
                        monkeys[target].items.push_back(item);
                    }
                }
            }

            monkeys.sort_unstable_by_key(|f| f.count);

            return monkeys
                .iter()
                .rev()
                .take(2)
                .map(|m| m.count)
                .product::<u64>()
                .to_string();
        }

        fn part_2(&self, input: &str) -> String {
            let mut monkeys = parse(input);
            let rounds = 10_000;

            let common_multiple: u64 = monkeys.iter().map(|m| m.test).product();

            for _ in 0..rounds {
                for idx in 0..monkeys.len() {
                    while let Some(mut item) = monkeys[idx].inspect() {
                        // By using mod of mmc, we can lower the value without affecting the tests.
                        item = item % common_multiple;

                        let target = monkeys[idx].test(item);

                        //throw
                        monkeys[target].items.push_back(item);
                    }
                }
            }

            monkeys.sort_unstable_by_key(|f| f.count);

            return monkeys
                .iter()
                .rev()
                .take(2)
                .map(|m| m.count)
                .product::<u64>()
                .to_string();
        }
    }

    pub fn parse(input: &str) -> Vec<Monkey> {
        let path = BASEPATH.to_owned() + input;
        let mut iter = get_lines(&path);

        let mut monkeys: Vec<Monkey> = Vec::new();
        while let Some(line) = iter.next() {
            let mut content = line.expect("Should be able to read line");

            if content.starts_with("Monkey") {
                content = iter.next().unwrap().unwrap();
                let items = parse_items(&content);

                content = iter.next().unwrap().unwrap();
                let operation = parse_operation(&content);

                content = iter.next().unwrap().unwrap();
                let test = parse_test(&content);

                content = iter.next().unwrap().unwrap();
                let if_true = parse_if(&content);

                content = iter.next().unwrap().unwrap();
                let if_false = parse_if(&content);

                monkeys.push(Monkey::new(items, operation, test, if_true, if_false));
            }
        }

        monkeys
    }

    pub fn parse_items(input: &str) -> VecDeque<u64> {
        let numbers = input.trim().replace("Starting items: ", "");
        numbers
            .split(", ")
            .map(|s| s.parse::<u64>().expect("Should parse to int"))
            .collect()
    }

    pub fn parse_operation(input: &str) -> Operation {
        let op = input.trim().replace("Operation: new = ", "");
        if op.contains('+') {
            let (_, op2) = op.split_once(" + ").unwrap();
            return Operation::Sum(op2.parse().unwrap());
        }

        let (_, op2) = op.split_once(" * ").unwrap();
        if op2 == "old" {
            return Operation::Square;
        }

        return Operation::Multiply(op2.parse::<u64>().unwrap());
    }

    pub fn parse_test(input: &str) -> u64 {
        let test = input.trim().replace("Test: divisible by ", "");
        return test.parse::<u64>().unwrap();
    }

    pub fn parse_if(input: &str) -> u64 {
        let reg = Regex::new(r"[a-zA-Z\s:]+").unwrap();
        let if_r = reg.replace(input, "");
        if_r.parse::<u64>().unwrap()
    }

    #[derive(Debug, PartialEq, Eq)]
    pub enum Operation {
        Multiply(u64),
        Sum(u64),
        Square,
    }

    #[derive(Debug)]
    pub struct Monkey {
        items: VecDeque<u64>,
        operation: Operation,
        test: u64,
        if_true: u64,
        if_false: u64,
        count: u64,
    }

    impl Monkey {
        pub fn new(
            items: VecDeque<u64>,
            operation: Operation,
            test: u64,
            if_true: u64,
            if_false: u64,
        ) -> Monkey {
            Monkey {
                items,
                operation,
                test,
                if_true,
                if_false,
                count: 0,
            }
        }

        pub fn inspect(&mut self) -> Option<u64> {
            if let Some(mut item) = self.items.pop_front() {
                match self.operation {
                    Operation::Sum(x) => item += x,
                    Operation::Multiply(x) => item *= x,
                    Operation::Square => item *= item,
                };

                self.count += 1;

                return Some(item);
            }
            None
        }

        pub fn test(&self, item: u64) -> usize {
            let target: usize;
            if item % self.test == 0 {
                target = self.if_true.try_into().unwrap();
            } else {
                target = self.if_false.try_into().unwrap();
            }

            target
        }
    }
}

#[cfg(test)]
mod test {
    use crate::{
        day::Solution,
        year2022::day11::day11::{parse_items, Operation},
    };

    use super::day11::{parse_if, parse_operation, Day11};

    #[test]
    fn test_part_1() {
        let result = Day11.part_1("example.txt");
        assert_eq!(result, "10605");
    }

    #[test]
    fn test_part_2() {
        let result = Day11.part_2("example.txt");
        assert_eq!(result, "2713310158");
    }

    #[test]
    fn test_parse_items() {
        let items = parse_items("  Starting items: 79, 98");
        assert_eq!(items, vec![79, 98]);
    }

    #[test]
    fn test_parse_operation() {
        let operation = parse_operation("  Operation: new = old * 19");
        assert_eq!(operation, Operation::Multiply(19));

        let operation = parse_operation("  Operation: new = old + 19");
        assert_eq!(operation, Operation::Sum(19));

        let operation = parse_operation("  Operation: new = old * old");
        assert_eq!(operation, Operation::Square);
    }

    #[test]
    fn test_if() {
        let test = parse_if("    If true: throw to monkey 2");
        assert_eq!(test, 2);

        let test = parse_if("    If false: throw to monkey 3");
        assert_eq!(test, 3);
    }
}

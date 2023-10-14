pub mod day09 {

    use std::collections::HashSet;

    use advent_of_rust::get_lines;

    use crate::day::Solution;
    pub struct Day09;

    pub const BASEPATH: &str = "src/year2022/day09/";
    impl Solution for Day09 {
        fn part_1(&self, input: &str) -> String {
            let path = BASEPATH.to_owned() + input;
            let mut iter = get_lines(&path);

            let tail = Node::default();
            let mut head = Node::default();
            head.add_tail(tail);

            while let Some(line) = iter.next() {
                let content = line.expect("Should be able to read line");
                let (direction, times) = content
                    .split_once(" ")
                    .expect("String should have one whitespace");
                let times = times.parse::<u32>().expect("should be able to parse");
                head.update(direction, times);
            }

            return head.print();
        }

        fn part_2(&self, input: &str) -> String {
            return "0".to_string();
        }
    }

    #[derive(Default)]
    pub struct Node {
        next: Option<Box<Node>>,
        pub position: (i32, i32),
        pub visited: HashSet<(i32, i32)>,
    }

    impl Node {
        pub fn add_tail(&mut self, tail: Node) {
            self.next = Some(Box::new(tail));
        }

        pub fn print(&self) -> String {
            match self.next.as_ref() {
                Some(tail) => tail.print(),
                None => {
                    return self.visited.len().to_string();
                }
            }
        }

        pub fn update(&mut self, direction: &str, times: u32) {
            for _ in 0..times {
                self.position = match direction {
                    "R" => (self.position.0 + 1, self.position.1),
                    "L" => (self.position.0 - 1, self.position.1),
                    "U" => (self.position.0, self.position.1 + 1),
                    "D" => (self.position.0, self.position.1 - 1),
                    _ => unreachable!(),
                };
                self.pull();
            }
        }

        pub fn pull(&mut self) {
            if let Some(tail) = self.next.as_mut() {
                let diff_x: i32 = self.position.0 - tail.position.0;
                let a_diff_x = diff_x.abs();

                let diff_y: i32 = self.position.1 - tail.position.1;
                let a_diff_y = diff_y.abs();

                if a_diff_x > 1 && a_diff_y == 0 {
                    tail.position.0 += diff_x / a_diff_x;
                }

                if a_diff_y > 1 && a_diff_x == 0 {
                    tail.position.1 += diff_y / a_diff_y;
                }

                if a_diff_x >= 2 && a_diff_y >= 1 || a_diff_y >= 2 && a_diff_x >= 1 {
                    tail.position.0 += diff_x / a_diff_x;
                    tail.position.1 += diff_y / a_diff_y;
                }
                tail.visited.insert(tail.position);

                tail.pull();
            }
        }
    }
}

#[cfg(test)]
mod test {
    use crate::day::Solution;

    use super::day09::{Day09, Node};
    #[test]
    fn test_part_1() {
        let result = Day09.part_1("example.txt");
        assert_eq!(result, "13");
    }

    fn test_update() {
        let mut head = Node::default();
        head.update("R", 2);
        head.update("U", 2);
        assert_eq!(head.position, (2, 2));
        head.update("L", 1);
        head.update("D", 1);
        assert_eq!(head.position, (1, 1));
    }
}

pub mod day09 {

    use std::{borrow::BorrowMut, cell::RefCell, collections::HashSet, ops::Deref, rc::Rc};

    use advent_of_rust::get_lines;

    use crate::day::Solution;
    pub struct Day09;

    pub const BASEPATH: &str = "src/year2022/day09/";
    impl Solution for Day09 {
        fn part_1(&self, input: &str) -> String {
            let tail = Node::default();
            let mut head = Node::default();
            head.add_tail(tail);

            update_head(input, &mut head);

            return head.print();
        }

        fn part_2(&self, input: &str) -> String {
            let mut head = Node::default();
            head.add_tail(Node::default());
            head.add_tail(Node::default());
            head.add_tail(Node::default());
            head.add_tail(Node::default());
            head.add_tail(Node::default());
            head.add_tail(Node::default());
            head.add_tail(Node::default());
            head.add_tail(Node::default());
            head.add_tail(Node::default());

            update_head(input, &mut head);

            return head.print();
        }
    }

    fn update_head(input: &str, head: &mut Node) {
        let path = BASEPATH.to_owned() + input;
        let mut iter = get_lines(&path);
        while let Some(line) = iter.next() {
            let content = line.expect("Should be able to read line");
            let (direction, times) = content
                .split_once(" ")
                .expect("String should have one whitespace");
            let times = times.parse::<u32>().expect("should be able to parse");
            head.update(direction, times);
        }
    }

    #[derive(Default, Clone)]
    pub struct Node {
        next: Option<Rc<RefCell<Node>>>,
        pub position: (i32, i32),
        pub visited: HashSet<(i32, i32)>,
    }

    impl Node {
        pub fn add_tail(&mut self, tail: Node) {
            match &self.next {
                None => self.next = Some(Rc::new(RefCell::new(tail))),
                Some(tail_ref) => tail_ref.deref().borrow_mut().add_tail(tail),
            }
        }

        pub fn print(&self) -> String {
            match &self.next {
                Some(tail_rc) => tail_rc.borrow().print(),
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

        pub fn pull(&self) {
            if let Some(tail_rc) = &self.next {
                let tail_refcell = tail_rc.deref();
                let mut tail = tail_refcell.borrow_mut();
                let mut position = tail.position.clone();

                let diff_x: i32 = self.position.0 - position.0;
                let a_diff_x = diff_x.abs();

                let diff_y: i32 = self.position.1 - position.1;
                let a_diff_y = diff_y.abs();

                if a_diff_x > 1 && a_diff_y == 0 {
                    position.0 += diff_x / a_diff_x;
                }

                if a_diff_y > 1 && a_diff_x == 0 {
                    position.1 += diff_y / a_diff_y;
                }

                if a_diff_x >= 2 && a_diff_y >= 1 || a_diff_y >= 2 && a_diff_x >= 1 {
                    position.0 += diff_x / a_diff_x;
                    position.1 += diff_y / a_diff_y;
                }
                tail.visited.insert(position);

                tail.position = position;

                tail.pull();
            }
        }
    }
}

#[cfg(test)]
mod test {
    use crate::day::Solution;

    use super::day09::Day09;
    #[test]
    fn test_part_1() {
        let result = Day09.part_1("example.txt");
        assert_eq!(result, "13");
    }

    #[test]
    fn test_part_2() {
        let result = Day09.part_2("example.txt");
        assert_eq!(result, "1");
    }
}

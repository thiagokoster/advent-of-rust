pub mod day08 {
    use advent_of_rust::get_lines;

    use crate::day::Solution;

    pub struct Day08;
    pub const BASEPATH: &str = "src/year2022/day08/";

    impl Solution for Day08 {
        fn part_1(&self, input: &str) -> String {
            let path = BASEPATH.to_owned() + input;
            let grid = parse(&path);

            let mut count = 0;
            for row in grid.as_slice() {
                for col in row {
                    let (visible, _) = col.is_visible(&grid);
                    if visible {
                        count = count + 1;
                    }
                }
            }

            return count.to_string();
        }

        fn part_2(&self, input: &str) -> String {
            let path = BASEPATH.to_owned() + input;
            let grid = parse(&path);

            let mut max_score = 0;
            for i in 0..grid.len() {
                for j in 0..grid[i].len() {
                    let (visible, score) = grid[i][j].is_visible(&grid);
                    max_score = std::cmp::max(score, max_score);
                }
            }

            return max_score.to_string();
        }
    }

    #[derive(Debug)]
    pub struct Tree {
        is_visible: bool,
        height: u32,
        position: (usize, usize),
    }

    impl Tree {
        pub fn new(height: u32, position: (usize, usize)) -> Tree {
            let tree = Tree {
                is_visible: true,
                height,
                position,
            };

            tree
        }

        pub fn is_visible(&self, grid: &Vec<Vec<Tree>>) -> (bool, u32) {
            match self.position {
                (0, _) => (true, 0),
                (_, 0) => (true, 0),
                (i, j) => self.visible_neighbour(grid),
            }
        }

        fn visible_neighbour(&self, grid: &Vec<Vec<Tree>>) -> (bool, u32) {
            let (west, w) = self.look_west(grid);
            let (east, e) = self.look_east(grid);
            let (north, n) = self.look_north(grid);
            let (south, s) = self.look_south(grid);

            return (west || east || north || south, w * e * n * s);
        }

        fn look_west(&self, grid: &Vec<Vec<Tree>>) -> (bool, u32) {
            let mut score = 0;
            for j in (0..self.position.1).rev() {
                score += 1;
                let neighbour = &grid[self.position.0][j];
                if neighbour.height >= self.height {
                    return (false, score);
                }
            }
            (true, score)
        }

        fn look_east(&self, grid: &Vec<Vec<Tree>>) -> (bool, u32) {
            let mut score = 0;
            for j in self.position.1 + 1..grid.len() {
                score += 1;
                let neighbour = &grid[self.position.0][j];
                if neighbour.height >= self.height {
                    return (false, score);
                }
            }
            (true, score)
        }

        fn look_north(&self, grid: &Vec<Vec<Tree>>) -> (bool, u32) {
            let mut score = 0;
            for i in (0..self.position.0).rev() {
                score += 1;
                let neighbour = &grid[i][self.position.1];
                if neighbour.height >= self.height {
                    return (false, score);
                }
            }
            (true, score)
        }
        fn look_south(&self, grid: &Vec<Vec<Tree>>) -> (bool, u32) {
            let mut score = 0;
            for i in self.position.0 + 1..grid.len() {
                score += 1;
                let neighbour = &grid[i][self.position.1];
                if neighbour.height >= self.height {
                    return (false, score);
                }
            }
            (true, score)
        }
    }

    pub fn parse(path: &str) -> Vec<Vec<Tree>> {
        let iter = get_lines(&path);
        iter.enumerate()
            .map(|(i, line)| {
                line.unwrap()
                    .chars()
                    .enumerate()
                    .map(|(j, c)| {
                        Tree::new(
                            c.to_digit(10).expect("Should parse char to integer"),
                            (i, j),
                        )
                    })
                    .collect()
            })
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use crate::day::Solution;

    use super::day08::Day08;

    #[test]
    fn test_part_1() {
        let result = Day08.part_1("example.txt");
        assert_eq!(result, "21");
    }

    #[test]
    fn test_part_2() {
        let result = Day08.part_2("example.txt");
        assert_eq!(result, "8");
    }
}

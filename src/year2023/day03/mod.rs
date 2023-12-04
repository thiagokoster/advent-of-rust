use std::cell::RefCell;

use crate::day::Solution;
use advent_of_rust::read_file;

pub struct Day03;
pub const BASEPATH: &str = "src/year2023/day03/";

#[derive(Debug, PartialEq, Clone, Copy)]
enum CellType {
    Empty,
    Number,
    Symbol,
    Gear,
}

#[derive(Debug, Clone)]
struct Cell {
    x: usize,
    y: usize,
    symbol: char,
    r#type: CellType,
}

impl Cell {
    fn clone(&self) -> Self {
        Cell {
            x: self.x,
            y: self.y,
            symbol: self.symbol,
            r#type: self.r#type,
        }
    }
    fn from_char(c: char, x: usize, y: usize) -> Self {
        match c {
            '.' => Cell {
                x,
                y,
                symbol: c,
                r#type: CellType::Empty,
            },
            '0'..='9' => Cell {
                x,
                y,
                symbol: c,
                r#type: CellType::Number,
            },
            '*' => Cell {
                x,
                y,
                symbol: c,
                r#type: CellType::Gear,
            },
            _ => Cell {
                x,
                y,
                symbol: c,
                r#type: CellType::Symbol,
            },
        }
    }

    fn is_symbol(&self) -> bool {
        match self.r#type {
            CellType::Symbol => true,
            CellType::Gear => true,
            _ => false,
        }
    }

    fn is_near_symbol(&self, grid: &Vec<Vec<RefCell<Cell>>>) -> bool {
        match self.r#type {
            CellType::Number => {
                if self.check_neighbour(&grid) {
                    return true;
                }

                let mut i = 1;
                while self.x + i < grid[0].len()
                    && grid[self.y][self.x + i].borrow().symbol.is_ascii_digit()
                {
                    let right = grid[self.y][self.x + i].borrow();
                    i += 1;
                    return right.is_near_symbol(&grid);
                }
                false
            }
            _ => false,
        }
    }

    fn get_neighbours(&self, grid: &Vec<Vec<RefCell<Cell>>>) -> Vec<Cell> {
        let x = self.x;
        let y = self.y;
        let mut neighbours: Vec<Cell> = Vec::new();

        if x > 0 {
            neighbours.push(grid[y][x - 1].borrow().clone());

            if y < grid.len() - 1 {
                neighbours.push(grid[y + 1][x - 1].borrow().clone());
            }
        }

        // 0  -1
        if y > 0 {
            neighbours.push(grid[y - 1][x].borrow().clone());

            if x < grid[0].len() - 1 {
                neighbours.push(grid[y - 1][x + 1].borrow().clone());
            }
        }

        // -1 -1
        if x > 0 && y > 0 {
            neighbours.push(grid[y - 1][x - 1].borrow().clone());
        }

        // +1 0
        if x < grid[0].len() - 1 {
            neighbours.push(grid[y][x + 1].borrow().clone());
        }

        // 0  +1
        if y < grid.len() - 1 {
            neighbours.push(grid[y + 1][x].borrow().clone());
        }

        // +1 +1
        if x < grid[0].len() - 1 && y < grid.len() - 1 {
            neighbours.push(grid[y + 1][x + 1].borrow().clone());
        }

        neighbours
    }

    fn check_neighbour(&self, grid: &Vec<Vec<RefCell<Cell>>>) -> bool {
        let neighbours = self.get_neighbours(grid);
        let symbol_neighbour = neighbours.iter().find(|n| n.is_symbol());
        symbol_neighbour.is_some()
    }

    fn value(&mut self, line: &Vec<RefCell<Cell>>) -> Option<u32> {
        match self.r#type {
            CellType::Number => {
                let mut value = self.symbol.to_string();
                let mut i = 1;

                if self.x > 0 && !line[self.x - 1].borrow().symbol.is_ascii_digit() || self.x == 0 {
                    while self.x + i < line.len()
                        && line[self.x + i].borrow().r#type == CellType::Number
                    {
                        let right = &line[self.x + i].borrow();
                        value.push(right.symbol);
                        i += 1;
                    }
                    return Some(value.parse::<u32>().unwrap());
                }
                None
            }
            _ => None,
        }
    }

    fn gear_ratio(&self, grid: &Vec<Vec<RefCell<Cell>>>) -> Option<u32> {
        if self.r#type != CellType::Gear {
            return None;
        }

        Some(1)
    }
}

impl Solution for Day03 {
    fn part_1(&self, input: &str) -> String {
        let path = BASEPATH.to_owned() + input;
        let lines = read_file(&path);

        let grid = parse_input(&lines);
        initialize(&grid);
        println!("GRID: {} x {}", grid[0].len(), grid.len());

        let mut result = 0;

        for row in grid.iter() {
            for cell in row {
                if cell.borrow().is_near_symbol(&grid) {
                    if let Some(value) = cell.borrow_mut().value(row) {
                        result += value;
                    }
                }
            }
            println!("");
        }
        result.to_string()
    }

    fn part_2(&self, input: &str) -> String {
        let path = BASEPATH.to_owned() + input;
        let lines = read_file(&path);

        let grid = parse_input(&lines);
        initialize(&grid);
        "".to_string()
    }
}

fn parse_input(lines: &Vec<String>) -> Vec<Vec<RefCell<Cell>>> {
    let mut grid: Vec<Vec<RefCell<Cell>>> = Vec::new();
    for (y, line) in lines.iter().enumerate() {
        grid.push(
            line.chars()
                .enumerate()
                .map(|(x, c)| RefCell::new(Cell::from_char(c, x, y)))
                .collect(),
        );
    }

    grid
}

fn initialize(grid: &Vec<Vec<RefCell<Cell>>>) {
    for row in grid.iter() {
        for cell in row {
            let cell = cell.borrow();
            cell.is_near_symbol(grid);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_part_1() {
        let day = Day03 {};

        let result = day.part_1("example.txt");
        assert_eq!(result, "4361");
    }

    #[test]
    fn test_part_2() {
        let day = Day03 {};

        let result = day.part_2("example.txt");
        assert_eq!(result, "467835");
    }
}

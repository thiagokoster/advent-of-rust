use std::collections::HashSet;

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

    fn is_near_symbol(&self, grid: &Vec<Vec<Cell>>) -> bool {
        match self.r#type {
            CellType::Number => {
                if self.check_neighbour(&grid) {
                    return true;
                }

                let mut i = 1;
                while self.x + i < grid[0].len() && grid[self.y][self.x + i].symbol.is_ascii_digit()
                {
                    let right = &grid[self.y][self.x + i];
                    i += 1;
                    return right.is_near_symbol(&grid);
                }
                false
            }
            _ => false,
        }
    }

    fn get_neighbours(&self, grid: &Vec<Vec<Cell>>) -> Vec<Cell> {
        let x = self.x;
        let y = self.y;
        let mut neighbours: Vec<Cell> = Vec::new();

        if x > 0 {
            neighbours.push(grid[y][x - 1].clone());

            if y < grid.len() - 1 {
                neighbours.push(grid[y + 1][x - 1].clone());
            }
        }

        // 0  -1
        if y > 0 {
            neighbours.push(grid[y - 1][x].clone());

            if x < grid[0].len() - 1 {
                neighbours.push(grid[y - 1][x + 1].clone());
            }
        }

        // -1 -1
        if x > 0 && y > 0 {
            neighbours.push(grid[y - 1][x - 1].clone());
        }

        // +1 0
        if x < grid[0].len() - 1 {
            neighbours.push(grid[y][x + 1].clone());
        }

        // 0  +1
        if y < grid.len() - 1 {
            neighbours.push(grid[y + 1][x].clone());
        }

        // +1 +1
        if x < grid[0].len() - 1 && y < grid.len() - 1 {
            neighbours.push(grid[y + 1][x + 1].clone());
        }

        neighbours
    }

    fn check_neighbour(&self, grid: &Vec<Vec<Cell>>) -> bool {
        let neighbours = self.get_neighbours(grid);
        let symbol_neighbour = neighbours.iter().find(|n| n.is_symbol());
        symbol_neighbour.is_some()
    }

    fn value(&self, line: &Vec<Cell>) -> Option<u32> {
        match self.r#type {
            CellType::Number => {
                let mut start = self.x;
                let mut end = self.x;
                let mut value = String::new();

                // going backwards to find the start of the number
                while start > 0 && line[start - 1].r#type == CellType::Number {
                    start -= 1;
                }

                // going forward to find the end of the number
                while end < line.len() - 1 && line[end + 1].r#type == CellType::Number {
                    end += 1;
                }

                for index in start..=end {
                    value.push(line[index].symbol.clone());
                }

                Some(value.parse::<u32>().unwrap())
            }
            _ => None,
        }
    }

    fn gear_ratio(&self, grid: &Vec<Vec<Cell>>) -> Option<u32> {
        if self.r#type != CellType::Gear {
            return None;
        }

        let neighbours = self.get_neighbours(grid);
        let parts: HashSet<u32> = neighbours
            .iter()
            .filter(|n| n.is_near_symbol(grid))
            .map(|n| n.value(&grid[n.y]).unwrap())
            .collect();

        if parts.len() == 2 {
            return Some(parts.iter().product());
        }
        None
    }
}

impl Solution for Day03 {
    fn part_1(&self, input: &str) -> String {
        let path = BASEPATH.to_owned() + input;
        let lines = read_file(&path);

        let grid = parse_input(&lines);
        initialize(&grid);
        println!("GRID: {} x {}", grid[0].len(), grid.len());

        let mut values: HashSet<u32> = HashSet::new();

        for row in grid.iter() {
            for cell in row {
                if cell.is_near_symbol(&grid) {
                    if let Some(value) = cell.value(row) {
                        values.insert(value);
                    }
                }
            }
        }
        values.iter().sum::<u32>().to_string()
    }

    fn part_2(&self, input: &str) -> String {
        let path = BASEPATH.to_owned() + input;
        let lines = read_file(&path);
        let mut result = 0;

        let grid = parse_input(&lines);
        initialize(&grid);
        for row in grid.iter() {
            for cell in row {
                if let Some(ratio) = cell.gear_ratio(&grid) {
                    result += ratio;
                }
            }
        }
        result.to_string()
    }
}

fn parse_input(lines: &Vec<String>) -> Vec<Vec<Cell>> {
    let mut grid: Vec<Vec<Cell>> = Vec::new();
    for (y, line) in lines.iter().enumerate() {
        grid.push(
            line.chars()
                .enumerate()
                .map(|(x, c)| Cell::from_char(c, x, y))
                .collect(),
        );
    }

    grid
}

fn initialize(grid: &Vec<Vec<Cell>>) {
    for row in grid.iter() {
        for cell in row {
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

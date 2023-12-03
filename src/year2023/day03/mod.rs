use std::borrow::BorrowMut;

use crate::day::Solution;
use advent_of_rust::read_file;

pub struct Day03;
pub const BASEPATH: &str = "src/year2023/day03/";

#[derive(Debug, PartialEq)]
enum CellType {
    Empty,
    Number,
    Symbol,
}

#[derive(Debug)]
struct Cell {
    x: usize,
    y: usize,
    symbol: char,
    r#type: CellType,
    value: Option<u32>,
}

impl Cell {
    fn from_char(c: char, x: usize, y: usize) -> Self {
        if c == '.' {
            return Cell {
                x,
                y,
                symbol: c,
                r#type: CellType::Empty,
                value: None,
            };
        }
        if c.is_ascii_digit() {
            return Cell {
                x,
                y,
                symbol: c,
                r#type: CellType::Number,
                value: None,
            };
        }

        Cell {
            x,
            y,
            symbol: c,
            r#type: CellType::Symbol,
            value: None,
        }
    }

    fn is_symbol(&self) -> bool {
        match self.r#type {
            CellType::Symbol => true,
            _ => false,
        }
    }

    fn is_near_symbol(&self, grid: &Vec<Vec<Cell>>) -> bool {
        match self.r#type {
            CellType::Number => self.check_neighbour(&grid),
            _ => false,
        }
    }

    fn check_neighbour(&self, grid: &Vec<Vec<Cell>>) -> bool {
        let x = self.x;
        let y = self.y;
        if x > 0 {
            if grid[y][x - 1].is_symbol() {
                return true;
            }
        }

        if y > 0 {
            if grid[y - 1][x].is_symbol() {
                return true;
            }
        }

        if x > 0 && y > 0 {
            if grid[y - 1][x - 1].is_symbol() {
                return true;
            }
        }

        if x < grid[0].len() - 1 {
            if grid[y][x + 1].is_symbol() {
                return true;
            }
        }

        if y < grid.len() - 1 {
            if grid[y + 1][x].is_symbol() {
                return true;
            }
        }

        if x < grid[0].len() - 1 && y < grid.len() - 1 {
            let n = &grid[y + 1][x + 1];
            if n.is_symbol() {
                return true;
            }
        }

        false
    }
}

impl Solution for Day03 {
    fn part_1(&self, input: &str) -> String {
        let path = BASEPATH.to_owned() + input;
        let lines = read_file(&path);

        let grid = parse_input(&lines);
        println!("GRID: {} x {}", grid[0].len(), grid.len());

        for row in grid.iter() {
            for cell in row {
                print!("{}", cell.symbol,);
            }
            println!("");
        }
        "".to_string()
    }

    fn part_2(&self, input: &str) -> String {
        "".to_string()
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

fn get_values(grid: &mut Vec<Vec<Cell>>, width: usize) {
    for row in grid.iter_mut() {
        for cell in row.iter_mut() {
            if cell.r#type == CellType::Number {
                let mut value = String::new();
                value.push(cell.symbol);
                if cell.x < width - 1 {
                    let offset = 1;
                    let right = grid[cell.y][cell.x + offset];
                    while cell.x + offset < width && right.r#type == CellType::Number {
                        value.push(right.symbol);
                    }

                    (*cell).value = Some(value.parse::<u32>().unwrap());
                }
            }
        }
    }
}

fn initialize(grid: &Vec<Vec<Cell>>) {
    for row in grid.iter() {
        for cell in row.iter() {
            cell.is_near_symbol(&grid);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_part_1() {
        let day = Day03 {};

        day.part_1("example.txt");
    }
}

use crate::day;

mod day01;
mod day02;
mod day03;
mod day04;
mod day05;
mod day06;
mod day07;
mod day08;
mod day09;
mod day10;
mod day11;

pub const ALL: [&dyn day::Solution; 11] = [
    &day01::day01::Day01,
    &day02::day02::Day02,
    &day03::day03::Day03,
    &day04::day04::Day04,
    &day05::day05::Day05,
    &day06::day06::Day06,
    &day07::day07::Day07,
    &day08::day08::Day08,
    &day09::day09::Day09,
    &day10::day10::Day10,
    &day11::day11::Day11,
];

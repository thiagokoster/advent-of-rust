use crate::day;

mod day01;
mod day02;
mod day03;
mod day04;

pub const ALL: [&dyn day::Solution; 4] = [
    &day01::day01::Day01,
    &day02::day02::Day02,
    &day03::day03::Day03,
    &day04::day04::Day04,
];

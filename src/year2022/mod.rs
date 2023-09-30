use crate::day;

mod day01;
mod day02;


pub const ALL: [&dyn day::Solution; 2] = [
    &day01::day01::Day01,
    &day02::day02::Day02,
];

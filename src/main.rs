mod day;
mod year2022;
mod year2023;

fn main() {
    println!("Advent of Rust!");
    let day = year2023::ALL.last().unwrap();
    let part1 = day.part_1("input.txt");
    let part2 = day.part_2("input.txt");

    println!("Part 1: {}", part1);
    println!("Part 2: {}", part2);
    println!("----------------");
}

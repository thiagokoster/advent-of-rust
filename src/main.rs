mod day;
mod year2022;
mod year2023;

fn main() {
    println!("Advent of Rust!");
    for (i, e) in year2023::ALL.iter().enumerate() {
        println!("----------------");
        println!("Day: {}", i + 1);
        let part1 = e.part_1("input.txt");
        let part2 = e.part_2("input.txt");

        println!("Part 1: {}", part1);
        println!("Part 2: {}", part2);
        println!("----------------");
    }
}

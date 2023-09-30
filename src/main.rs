mod year2022 {
    pub mod day01;
}

use year2022::day01::day01;

fn main() {
    println!("Hello, world!");
    let part1 = day01::part_1("input.txt");
    println!("Part 1: {}", part1);

    let part2 = day01::part_2("input.txt");
    println!("Part 2: {}", part2);
}

pub mod day01 {
    use advent_of_rust::get_lines;
    pub fn part1() -> u32 {
        println!("Hello, day 01!");

        if let Ok(lines) = get_lines("src/year2022/day01/example.txt") {
            for line in lines {
                if let Ok(content) = line {
                    println!("{}", content);
                }
            }
        }

        return 0;
    }
}

#[cfg(test)]
mod tests {
    use super::day01;

    #[test]
    fn test_part1() {
        let result = day01::part1();
        assert_eq!(result, 24000);
    }
}

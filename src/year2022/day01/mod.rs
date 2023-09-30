pub mod day01 {
    pub fn part1() {
        println!("Hello, day 01!");
    }
}

#[cfg(test)]
mod tests {
    use super::day01;

    #[test]
    fn test_part1() {
        day01::part1();
        assert_eq!(true, true);
    }
}

pub mod day01 {
    use advent_of_rust::get_lines;
    pub fn part_1(input : &str) -> u32 {
        let mut elves: Vec<u32> = Vec::new();
        elves.push(0);
        let mut i = 0;
        let path = "src/year2022/day01/".to_owned() + input;
        if let Ok(lines) = get_lines(&path) {
            for line in lines {
                if let Ok(content) = line {
                    if let Ok(calories) = content.parse::<u32>() {
                        elves[i] += calories;
                    } else {
                    i += 1;
                    elves.push(0);
                }
                } 
            }
        }
        elves.sort_by(|a,b| b.cmp(a));
        return elves[0];
    }
}

#[cfg(test)]
mod tests {
    use super::day01;

    #[test]
    fn test_part_1() {
        let result = day01::part_1("example.txt");
        println!("Hello, day 01!");
        assert_eq!(result, 24000);
    }
}

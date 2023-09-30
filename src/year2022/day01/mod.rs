pub mod day01 {
    use advent_of_rust::get_lines;


    pub fn part_1(input : &str) -> u32 {
        let mut elves: Vec<u32> = Vec::new();
        elves.push(0);
        map_calories(&mut elves, input);

        return elves[0];
    }

    pub fn part_2(input : &str) -> u32 {
        let mut elves: Vec<u32> = Vec::new();
        elves.push(0);
        map_calories(&mut elves, input);

        return elves[0] + elves[1] + elves[2];
    }


    fn map_calories(elves: &mut Vec<u32>, input : &str) {
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

use std::collections::HashMap;

use crate::day::Solution;
use advent_of_rust::read_file;
pub struct Day07;
pub const BASEPATH: &str = "src/year2023/day07/";

impl Solution for Day07 {
    fn part_1(&self, input: &str) -> String {
        let path = BASEPATH.to_owned() + input;
        let lines = read_file(&path);
        for line in lines {
            let hand = Hand::from_str(line.as_str());
            println!("{:?}", hand);
        }
        "".to_string()
    }

    fn part_2(&self, input: &str) -> String {
        "".to_string()
    }
}

#[derive(Debug)]
enum HandType {
    Fives,
    Fours,
    FullHouse,
    Threes,
    TwoPairs,
    Pair,
    HighCard,
}

#[derive(Debug)]
struct Hand {
    cards: String,
    hand_type: HandType,
    bid: u32,
}

impl Hand {
    fn from_str(line: &str) -> Self {
        let (cards, bid) = line.split_once(" ").unwrap();
        let bid = bid.parse::<u32>().unwrap();
        let hand_type = Hand::get_hand_type(cards);

        Hand {
            cards: cards.to_string(),
            hand_type,
            bid,
        }
    }

    fn get_hand_type(cards: &str) -> HandType {
        let card_count = cards.chars().fold(HashMap::new(), |mut char_count_map, c| {
            char_count_map
                .entry(c)
                .and_modify(|amount| *amount += 1)
                .or_insert(1u8);
            char_count_map
        });

        if card_count.len() == 1 {
            return HandType::Fives;
        }
        if card_count.len() == 2 {
            // A four or a FullHouse
            todo!("Find out if its a FullHouse or a Four");
            return HandType::Pair;
        }
        if card_count.len() == 3 {
            // A three of two pairs
            todo!("Find out if its a Threes or two pairs ");
            return HandType::Threes;
        }
        if card_count.len() == 4 {
            return HandType::Pair;
        }

        return HandType::HighCard;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_part_1() {
        let day = Day07 {};
        let result = day.part_1("example.txt");

        assert_eq!(result, "6440");
    }
}

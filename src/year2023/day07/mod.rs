use std::collections::HashMap;

use crate::day::Solution;
use advent_of_rust::read_file;
pub struct Day07;
pub const BASEPATH: &str = "src/year2023/day07/";

impl Solution for Day07 {
    fn part_1(&self, input: &str) -> String {
        let path = BASEPATH.to_owned() + input;
        let lines = read_file(&path);
        let mut hands: Vec<Hand> = lines
            .iter()
            .map(|line| Hand::from_str(line.as_str()))
            .collect();

        hands.sort();

        let mut result = 0;
        for (rank, hand) in hands.iter().enumerate() {
            println!(
                "Hand: {}, Bid: {}, Type: {:?}, Rank: {}",
                hand.cards,
                hand.bid,
                hand.hand_type,
                rank + 1
            );
            result += hand.bid * (rank + 1) as u32;
        }

        result.to_string()
    }

    fn part_2(&self, input: &str) -> String {
        "".to_string()
    }
}

#[derive(PartialEq, Eq, PartialOrd, Ord)]
enum Card {
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    T,
    J,
    Q,
    K,
    A,
}

impl Card {
    fn from_char(c: char) -> Self {
        match c {
            '2' => Card::Two,
            '3' => Card::Three,
            '4' => Card::Four,
            '5' => Card::Five,
            '6' => Card::Six,
            '7' => Card::Seven,
            '8' => Card::Eight,
            '9' => Card::Nine,
            'T' => Card::T,
            'J' => Card::J,
            'Q' => Card::Q,
            'K' => Card::K,
            'A' => Card::A,
            _ => unreachable!(),
        }
    }
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
enum HandType {
    HighCard,
    Pair,
    TwoPairs,
    Threes,
    FullHouse,
    Fours,
    Fives,
}

#[derive(Debug, PartialEq, Eq)]
struct Hand {
    pub cards: String,
    hand_type: HandType,
    bid: u32,
}

impl Hand {
    fn from_str(line: &str) -> Self {
        let (cards, bid) = line.split_once(" ").unwrap();
        let bid = bid.parse::<u32>().unwrap();
        let hand_type = Hand::get_hand_type(cards.to_string());

        Hand {
            cards: cards.to_string(),
            hand_type,
            bid,
        }
    }

    fn cmp_hand(&self, other: &Hand, index: usize) -> std::cmp::Ordering {
        if index >= self.cards.len() {
            return std::cmp::Ordering::Equal;
        }

        let cards: Vec<char> = self.cards.chars().collect();
        let other_cards: Vec<char> = other.cards.chars().collect();

        let cmp = Card::from_char(cards[index]).cmp(&Card::from_char(other_cards[index]));
        if cmp == std::cmp::Ordering::Equal {
            return self.cmp_hand(other, index + 1);
        }

        cmp
    }

    fn get_hand_type(cards: String) -> HandType {
        let mut cards: Vec<char> = cards.chars().collect();
        cards.sort();

        let cards: Vec<char> = cards.into_iter().collect();
        let card_count: HashMap<char, u8> =
            cards.iter().fold(HashMap::new(), |mut char_count_map, c| {
                char_count_map
                    .entry(*c)
                    .and_modify(|amount| *amount += 1)
                    .or_insert(1u8);
                char_count_map
            });

        if card_count.len() == 1 {
            return HandType::Fives;
        }
        if card_count.len() == 2 {
            // A four or a FullHouse
            let type_count = card_count[&cards[0]];
            if type_count == 4 || type_count == 1 {
                return HandType::Fours;
            }

            return HandType::FullHouse;
        }
        if card_count.len() == 3 {
            // A three of two pairs
            if card_count[&cards[0]] == 3
                || card_count[&cards[1]] == 3
                || card_count[&cards[4]] == 3
            {
                return HandType::Threes;
            }

            return HandType::TwoPairs;
        }
        if card_count.len() == 4 {
            return HandType::Pair;
        }

        return HandType::HighCard;
    }
}

impl Ord for Hand {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        let cmp = self.hand_type.cmp(&other.hand_type);

        if cmp == std::cmp::Ordering::Equal {
            self.cmp_hand(other, 0)
        } else {
            cmp
        }
    }
}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
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

    #[test]
    fn test_parse_threes() {
        let hand = Hand::from_str("8885Q 111");

        assert_eq!(hand.hand_type, HandType::Threes);
    }
}

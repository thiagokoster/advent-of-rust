pub mod day02 {
    use crate::day::Solution;
    use advent_of_rust::get_lines;
    pub struct Day02;

    impl Solution for Day02 {
        fn part_1(&self, input: &str) -> u32 {
            let mut total_score: u32 = 0;
            let path = "src/year2022/day02/".to_owned() + input;
            if let Ok(lines) = get_lines(&path) {
                for line in lines {
                    if let Ok(content) = line {
                        let moves_code = content.split_once(' ').unwrap();
                        let opponent = Move::from_str(moves_code.0);
                        let me = Move::from_str(moves_code.1);
                        let result = round(me, opponent);
                        let score = result.score(me);
                        total_score += score;
                    }
                }
            }

            return total_score;
        }

        fn part_2(&self, input: &str) -> u32 {
            let mut total_score: u32 = 0;
            let path = "src/year2022/day02/".to_owned() + input;
            if let Ok(lines) = get_lines(&path) {
                for line in lines {
                    if let Ok(content) = line {
                        let moves_code = content.split_once(' ').unwrap();
                        let opponent = Move::from_str(moves_code.0);
                        let outcome = Outcome::from_str(moves_code.1);
                        let me = match outcome {
                            Outcome::Lost => opponent.derive(false),
                            Outcome::Draw => opponent,
                            Outcome::Won => opponent.derive(true)
                        };
                        let score = outcome.score(me);
                        total_score += score;
                    }
                }
            }

            return total_score;
        }
    }

    pub fn round(me: Move, opponent: Move) -> Outcome {
        if me == opponent {
            return Outcome::Draw;
        }

        if (opponent as u32 + 1) % 3 == me as u32 {
            return Outcome::Won;
        }

        Outcome::Lost
    }

    #[derive(Debug, Clone, Copy, PartialEq)]
    pub enum Move {
        Rock,
        Paper,
        Scissors,
    }

    impl Move {
        fn from_str(code: &str) -> Self {
            match code {
                "A" | "X" => Move::Rock,
                "B" | "Y" => Move::Paper,
                "C" | "Z" => Move::Scissors,
                _ => unreachable!(),
            }
        }

        fn from_index(index: u32) -> Self {
            match index {
                0 => Move::Rock,
                1 => Move::Paper,
                2 => Move::Scissors,
                _ => unreachable!()
            }
        }

        fn derive(&self, win: bool) -> Self {
            let offset = if win { 1 } else { 2 };
            return Move::from_index((*self as u32 + offset) % 3);
        }
    }

    #[derive(Debug, Clone, Copy, PartialEq)]
    pub enum Outcome {
        Lost,
        Draw,
        Won,
    }

    impl Outcome {
        pub fn score(&self, me: Move) -> u32 {
            ((*self as u32) * 3) + (me as u32 + 1)
        }

        fn from_str(code: &str) -> Self {
            match code {
                "X" => Outcome::Lost,
                "Y" => Outcome::Draw,
                "Z" => Outcome::Won,
                _ => unreachable!()
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::day::Solution;

    use super::day02::{self, Move, *};

    #[test]
    fn test_part_1() {
        assert_eq!(Day02.part_1("example.txt"), 15);
    }

    #[test]
    fn test_part_2() {
        assert_eq!(Day02.part_2("example.txt"), 12);
    }

    #[test]
    fn test_round_rock() {
        assert_eq!(day02::round(Move::Rock, Move::Scissors), Outcome::Won);
        assert_eq!(day02::round(Move::Rock, Move::Paper), Outcome::Lost);
        assert_eq!(day02::round(Move::Rock, Move::Rock), Outcome::Draw);
    }

    #[test]
    fn test_round_paper() {
        assert_eq!(day02::round(Move::Paper, Move::Rock), Outcome::Won);
        assert_eq!(day02::round(Move::Paper, Move::Scissors), Outcome::Lost);
        assert_eq!(day02::round(Move::Paper, Move::Paper), Outcome::Draw);
    }

    #[test]
    fn test_round_scissor() {
        assert_eq!(day02::round(Move::Scissors, Move::Paper), Outcome::Won);
        assert_eq!(day02::round(Move::Scissors, Move::Rock), Outcome::Lost);
        assert_eq!(day02::round(Move::Scissors, Move::Scissors), Outcome::Draw);
    }
}

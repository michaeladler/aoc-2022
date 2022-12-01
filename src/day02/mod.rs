use log::debug;

use aoc_lib::parse;

#[derive(Debug, PartialEq, Eq, Copy, Clone)]
enum Item {
    Rock,
    Paper,
    Scissors,
}

impl Item {
    pub fn score(&self) -> i32 {
        use Item::*;
        match self {
            Rock => 1,
            Paper => 2,
            Scissors => 3,
        }
    }

    pub fn play(&self, opponent: Item) -> GameResult {
        use GameResult::*;
        use Item::*;
        match (self, opponent) {
            // Rock defeats Scissors
            (Rock, Scissors) => Win,
            (Scissors, Rock) => Loss,

            // Scissors defeats Paper
            (Scissors, Paper) => Win,
            (Paper, Scissors) => Loss,

            // Paper defeats Rock
            (Paper, Rock) => Win,
            (Rock, Paper) => Loss,

            _ => Draw,
        }
    }
}

#[derive(Debug, PartialEq, Eq, Copy, Clone)]
enum GameResult {
    Win,
    Loss,
    Draw,
}

impl GameResult {
    pub fn score(&self) -> i32 {
        use GameResult::*;
        match self {
            Loss => 0,
            Draw => 3,
            Win => 6,
        }
    }

    /// Find the move you have to play to get the desired result (from `self` point of view).
    pub fn find_move(&self, opponent: Item) -> Item {
        use GameResult::*;
        use Item::*;
        match (self, opponent) {
            (Win, Rock) => Paper,
            (Win, Paper) => Scissors,
            (Win, Scissors) => Rock,
            (Loss, Rock) => Scissors,
            (Loss, Paper) => Rock,
            (Loss, Scissors) => Paper,
            (Draw, x) => x,
        }
    }
}

pub fn solve(input: &[u8]) -> (String, String) {
    let mut input = input;

    let mut part1: i32 = 0;
    let mut part2: i32 = 0;
    while !input.is_empty() {
        let (rest, opponent_token) = parse::token(input).unwrap();
        let opponent = match opponent_token.as_bytes() {
            b"A" => Item::Rock,
            b"B" => Item::Paper,
            b"C" => Item::Scissors,
            _ => panic!("unexpected token"),
        };

        // skip whitespace in the middle
        let rest = &rest[1..];

        let (rest, me_token) = parse::token(rest).unwrap();
        let me = match me_token.as_bytes() {
            b"X" => Item::Rock,
            b"Y" => Item::Paper,
            b"Z" => Item::Scissors,
            _ => panic!("unexpected token"),
        };

        {
            let result = me.play(opponent);
            let score = me.score() + result.score();
            debug!(
                "parsed move: {:?} {:?}. game result: {:?}, score: {score}",
                opponent, me, result
            );
            part1 += score;
        }

        {
            let result = match me_token.as_bytes() {
                b"X" => GameResult::Loss,
                b"Y" => GameResult::Draw,
                b"Z" => GameResult::Win,
                _ => panic!("unexpected token"),
            };
            let me = result.find_move(opponent);
            debug!(
                "you must play {:?} against {:?} to have result {:?}",
                me, opponent, result
            );
            let score = me.score() + result.score();
            part2 += score;
        }

        input = parse::seek_next_line(rest);
    }

    (part1.to_string(), part2.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    const DAY: i32 = 02;

    #[test]
    fn example() {
        let input = b"A Y
B X
C Z
";
        let solution = solve(input);
        assert_eq!("15", solution.0);
        assert_eq!("12", solution.1);
    }

    #[test]
    fn part1_and_part2() {
        let answer = solve(&aoc_lib::io::read_input(DAY).unwrap());
        assert_eq!("10816", answer.0);
        assert_eq!("11657", answer.1);
    }
}

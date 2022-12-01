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
}

pub fn solve(input: &[u8]) -> (String, String) {
    let mut input = input;

    let mut total_score: i32 = 0;
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

        let result = me.play(opponent);
        let score = me.score() + result.score();
        debug!(
            "parsed move: {:?} {:?}. game result: {:?}, score: {score}",
            opponent, me, result
        );
        total_score += score;

        input = parse::seek_next_line(rest);
    }

    let part1 = total_score;
    let part2: i64 = 42;

    (part1.to_string(), part2.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    const DAY: i32 = 02;

    fn init() {
        let _ = env_logger::builder().is_test(true).try_init();
    }

    #[test]
    fn part1_example() {
        let bufs = vec![(
            b"A Y
B X
C Z
",
            15,
        )];

        for (s, answer) in bufs {
            assert_eq!(answer.to_string(), solve(s).0);
        }
    }

    #[test]
    #[ignore]
    fn part2_example() {
        let bufs = vec![(b"", 0)];

        for (s, answer) in bufs {
            assert_eq!(answer.to_string(), solve(s).1);
        }
    }

    #[test]
    fn part1_and_part2() {
        let answer = solve(&aoc_lib::io::read_input(DAY).unwrap());
        assert_eq!("10816", answer.0);
        // TODO
        //assert_eq!("42", answer.1);
    }
}

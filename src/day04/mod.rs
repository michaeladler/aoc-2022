use aoc_lib::{interval::ClosedInterval, parse};

pub fn solve(input: &[u8]) -> (String, String) {
    let mut part1: i32 = 0;
    let mut part2: i32 = 0;
    let mut input = input;
    while !input.is_empty() {
        let (rest, a1) = parse::integer(input, false).unwrap();
        let (rest, b1) = parse::integer(&rest[1..], false).unwrap();
        let int1 = ClosedInterval::new(a1, b1);

        let (rest, a2) = parse::integer(&rest[1..], false).unwrap();
        let (rest, b2) = parse::integer(&rest[1..], false).unwrap();
        let int2 = ClosedInterval::new(a2, b2);

        if int1.contains(&int2) || int2.contains(&int1) {
            part1 += 1;
        }
        if !int1.disjoint(&int2) {
            part2 += 1;
        }

        input = parse::seek_next_line(rest);
    }
    (part1.to_string(), part2.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    const DAY: i32 = 04;

    #[test]
    fn example() {
        let input = b"2-4,6-8
2-3,4-5
5-7,7-9
2-8,3-7
6-6,4-6
2-6,4-8
";
        let solution = solve(input);
        assert_eq!("2", solution.0);
        assert_eq!("4", solution.1);
    }

    #[test]
    fn part1_and_part2() {
        let answer = solve(&aoc_lib::io::read_input(DAY).unwrap());
        assert_eq!("538", answer.0);
        assert_eq!("792", answer.1);
    }
}

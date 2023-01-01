use arrayvec::ArrayVec;

use aoc_lib::parse;

extern "C" {
    pub fn mix(items: *const i32, n: usize) -> i32;
}

pub fn solve(input: &[u8]) -> (String, String) {
    let mut numbers: ArrayVec<i32, 5000> = ArrayVec::new();
    {
        let mut input = input;
        while !input.is_empty() {
            let (rest, x) = parse::integer(input, false).unwrap();
            numbers.push(x.try_into().unwrap());
            input = parse::seek_next_line(rest);
        }
    }

    let part1 = unsafe { mix(numbers.as_ptr(), numbers.len()) };
    let part2: i64 = 42;

    (part1.to_string(), part2.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    const DAY: i32 = 20;

    #[test]
    fn example() {
        let input = b"1
2
-3
3
-2
0
4
";

        let answer = solve(input);
        assert_eq!("3", answer.0);
    }

    #[test]
    fn part1_and_part2() {
        let answer = solve(&aoc_lib::io::read_input(DAY).unwrap());
        assert_eq!("7225", answer.0);
        //assert_eq!("42", answer.1);
    }
}

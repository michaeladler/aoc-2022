use arrayvec::ArrayVec;

use aoc_lib::parse;

extern "C" {
    pub fn mix(items: *const i64, n: usize, iterations: usize) -> i64;
}

pub fn solve(input: &[u8]) -> (String, String) {
    let mut numbers: ArrayVec<i64, 5000> = ArrayVec::new();
    {
        let mut input = input;
        while !input.is_empty() {
            let (rest, x) = parse::integer(input, false).unwrap();
            numbers.push(x);
            input = parse::seek_next_line(rest);
        }
    }

    let part1 = unsafe { mix(numbers.as_ptr(), numbers.len(), 1) };
    for x in numbers.iter_mut() {
        *x *= 811589153;
    }
    let part2 = unsafe { mix(numbers.as_ptr(), numbers.len(), 10) };

    (part1.to_string(), part2.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

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
        assert_eq!("1623178306", answer.1);
    }

    #[test]
    fn part1_and_part2() {
        let answer = solve(&aoc_lib::io::read_input(20).unwrap());
        assert_eq!("7225", answer.0);
        //assert_eq!("42", answer.1);
    }
}

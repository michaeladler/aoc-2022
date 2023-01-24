use log::debug;

use aoc_lib::parse;

pub fn solve(input: &[u8]) -> (String, String) {
    let mut sum = 0;
    let mut input = input;
    while !input.is_empty() {
        for (i, &b) in input.iter().enumerate() {
            if b == b'\n' {
                let src = &input[0..i];
                let x = snafu_to_dec(src);
                sum += x;
                debug!("parsed {} as {x}", String::from_utf8_lossy(src));
                input = &input[i..];
                break;
            }
        }
        input = parse::seek_next_line(input);
    }
    debug!("sum: {sum}");
    let part1 = dec_to_snafu(sum);
    (part1, String::new())
}

fn snafu_to_dec(s: &[u8]) -> i64 {
    let mut result: i64 = 0;
    let mut base = 1;
    for &x in s.iter().rev() {
        let val = snafu_to_digit(x);
        result += base * val;
        base *= 5;
    }
    result
}

fn dec_to_snafu(mut x: i64) -> String {
    let mut result = String::with_capacity(32);
    while x != 0 {
        let mut rem = x % 5;
        if rem > 2 {
            rem -= 5;
        }
        debug_assert!(rem >= -2);
        debug_assert!(rem <= 2);
        result.push(match rem {
            -2 => '=',
            -1 => '-',
            0 => '0',
            1 => '1',
            2 => '2',
            _ => unreachable!("invalid rem"),
        });
        x -= rem;
        x /= 5;
    }
    result.chars().rev().collect::<String>()
}

fn snafu_to_digit(x: u8) -> i64 {
    match x {
        b'2' => 2,
        b'1' => 1,
        b'0' => 0,
        b'-' => -1,
        b'=' => -2,
        _ => panic!("invalid input"),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const DAY: i32 = 25;

    #[test]
    fn test_parse_snafu() {
        assert_eq!(snafu_to_dec("2=-01".as_bytes()), 976);
    }

    #[test]
    fn test_dec_to_snafu() {
        assert_eq!(dec_to_snafu(4890), "2=-1=0".to_string());
    }

    #[test]
    fn example() {
        let input = b"1=-0-2
12111
2=0=
21
2=01
111
20012
112
1=-1=
1-12
12
1=
122
";

        let answer = solve(input);
        assert_eq!("2=-1=0", answer.0);
    }

    #[test]
    fn part1() {
        let answer = solve(&aoc_lib::io::read_input(DAY).unwrap());
        assert_eq!("2-1-110-=01-1-0-0==2", answer.0);
    }
}

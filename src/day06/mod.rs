use arrayvec::ArrayVec;

pub fn solve(input: &[u8]) -> (String, String) {
    (helper(input, 4).to_string(), helper(input, 14).to_string())
}

fn helper(input: &[u8], n: usize) -> usize {
    let mut prev_chars: ArrayVec<char, 16> = ArrayVec::new();
    for (i, &b) in input.iter().enumerate().skip(n - 1) {
        unsafe {
            for j in (1..n).rev() {
                prev_chars.push_unchecked(*input.get_unchecked(i - j) as char);
            }
            prev_chars.push_unchecked(b as char);
        }
        let mut bitset: u32 = 0;
        for &b in prev_chars.iter() {
            bitset |= 1 << (b as u8 - b'a');
        }
        if bitset.count_ones() == n as u32 {
            return i + 1;
        }
        prev_chars.clear();
    }
    return 0;
}

#[cfg(test)]
mod tests {
    use super::*;

    const DAY: i32 = 06;

    fn init() {
        let _ = env_logger::builder().is_test(true).try_init();
    }

    #[test]
    fn examples() {
        init();

        let bufs = vec![
            (&b"mjqjpqmgbljsphdztnvjfqwrcgsmlb"[..], 7, 19),
            (&b"bvwbjplbgvbhsrlpgdmjqwftvncz"[..], 5, 23),
            (&b"nppdvjthqldpwncqszvftbrmjlhg"[..], 6, 23),
            (&b"nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg"[..], 10, 29),
            (&b"zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw"[..], 11, 26),
        ];

        for (s, part1, part2) in bufs {
            let solution = solve(s);
            assert_eq!(part1.to_string(), solution.0);
            assert_eq!(part2.to_string(), solution.1);
        }
    }

    #[test]
    fn part1_and_part2() {
        let answer = solve(&aoc_lib::io::read_input(DAY).unwrap());
        assert_eq!("1080", answer.0);
        assert_eq!("3645", answer.1);
    }
}

use arrayvec::ArrayVec;
use log::debug;

pub fn solve(input: &[u8]) -> (String, String) {
    let mut part1: usize = 0;

    let mut prev_chars: ArrayVec<char, 4> = ArrayVec::new();
    for (i, &b) in input.iter().enumerate().skip(3) {
        unsafe {
            prev_chars.push_unchecked(*input.get_unchecked(i - 3) as char);
            prev_chars.push_unchecked(*input.get_unchecked(i - 2) as char);
            prev_chars.push_unchecked(*input.get_unchecked(i - 1) as char);
            prev_chars.push_unchecked(b as char);
        }
        let mut bitset: u32 = 0;
        for &b in prev_chars.iter() {
            let idx = b as u8 - b'a';
            bitset |= 1 << idx;
        }
        let n = bitset.count_ones();
        debug!("prev_chars: {:?}, cardinality: {n}", prev_chars);
        if n == 4 {
            part1 = i + 1;
            break;
        }

        prev_chars.clear();
    }

    let part2: i64 = 42;

    (part1.to_string(), part2.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    const DAY: i32 = 06;

    fn init() {
        let _ = env_logger::builder().is_test(true).try_init();
    }

    #[test]
    fn part1_example() {
        init();

        let bufs = vec![
            (&b"mjqjpqmgbljsphdztnvjfqwrcgsmlb"[..], 7),
            (&b"bvwbjplbgvbhsrlpgdmjqwftvncz"[..], 5),
            (&b"nppdvjthqldpwncqszvftbrmjlhg"[..], 6),
            (&b"nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg"[..], 10),
            (&b"zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw"[..], 11),
        ];

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
        assert_eq!("1080", answer.0);
        // TODO
        //assert_eq!("42", answer.1);
    }
}

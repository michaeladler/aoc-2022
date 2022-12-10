use ahash::AHashMap;
use log::{debug, trace};

use aoc_lib::parse;

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
struct Directory {
    path: String,
}

impl Directory {
    pub fn new(start: &str) -> Directory {
        let mut path = String::with_capacity(1024);
        path.push_str(start);
        Directory { path }
    }

    pub fn is_root(&self) -> bool {
        self.path.len() == 1
    }

    pub fn go_to_root(&mut self) {
        self.path.truncate(1);
    }

    pub fn parent(&mut self) {
        debug!("going to parent of {}", self.path);
        let mut pos = self.path.rfind('/').unwrap();
        if pos == 0 {
            pos = 1;
        }
        self.path.truncate(pos);
    }

    pub fn enter(&mut self, other: &str) {
        debug!("Entering {other}");
        if !self.is_root() {
            self.path.push('/');
        }
        self.path.push_str(other);
    }
}

pub fn solve(input: &[u8]) -> (String, String) {
    let mut input = input;

    let mut dirs: AHashMap<Directory, u64> = AHashMap::with_capacity(128);
    let start = "/";
    let mut cwd = Directory::new(start);

    while !input.is_empty() {
        debug!("=== parsing line ===");
        if input[0] == b'$' {
            // parse cmd
            match input[2] {
                b'c' => {
                    // cd command
                    let pos_eol = input
                        .iter()
                        .enumerate()
                        .find(|(_i, &x)| x == b'\n')
                        .unwrap()
                        .0;
                    let dest = &input[5..pos_eol];
                    trace!("$ cd {}", String::from_utf8_lossy(dest));
                    match dest {
                        b".." => {
                            cwd.parent();
                        }
                        b"/" => {
                            cwd.go_to_root();
                        }
                        _ => {
                            cwd.enter(&String::from_utf8_lossy(dest));
                        }
                    }

                    input = &input[pos_eol..];
                    input = parse::seek_next_line(input);
                }
                b'l' => {
                    // ls command
                    trace!("$ ls");
                    input = parse::seek_next_line(input);
                    // parse ls output
                    while !input.is_empty() && input[0] != b'$' {
                        match input[0] {
                            b'd' => {
                                // entry is a dir;
                                // we don't do anything with that information though
                                let pos_eol = input
                                    .iter()
                                    .enumerate()
                                    .find(|(_i, &x)| x == b'\n')
                                    .unwrap()
                                    .0;
                                let dir = &input[4..pos_eol];
                                trace!("dir {}", String::from_utf8_lossy(dir));
                            }
                            b'0'..=b'9' => {
                                // entry is a file
                                let (rest, fsize) = parse::positive(input, false).unwrap();
                                let pos_eol = rest
                                    .iter()
                                    .enumerate()
                                    .find(|(_i, &x)| x == b'\n')
                                    .unwrap()
                                    .0;
                                let fname = &rest[1..pos_eol];
                                trace!("{fsize} {}", String::from_utf8_lossy(fname));

                                {
                                    //  update dir and parent dirs
                                    let mut tmp = cwd.clone();
                                    loop {
                                        debug!("adding {fsize} to {:?}", tmp);
                                        let existing = dirs.entry(tmp.clone()).or_insert(0);
                                        *existing += fsize;
                                        if tmp.is_root() {
                                            break;
                                        }
                                        tmp.parent();
                                    }
                                }
                                input = rest;
                            }
                            _ => panic!("unexpected ls entry"),
                        }
                        input = parse::seek_next_line(input);
                    }
                }
                _ => panic!("unsupported command"),
            }
        }
        debug!("cwd: {:?}", cwd);
    }

    let part1: u64 = dirs.values().filter(|&&x| x <= 100000).sum();

    let total: u64 = 70000000;
    let free: u64 = total
        - dirs[&Directory {
            path: String::from("/"),
        }];
    let update_size: u64 = 30000000;
    let need_to_free = update_size - free;

    let part2 = dirs.values().filter(|&&x| x >= need_to_free).min().unwrap();

    (part1.to_string(), part2.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    const DAY: i32 = 07;

    #[test]
    fn example() {
        let input = b"$ cd /
$ ls
dir a
14848514 b.txt
8504156 c.dat
dir d
$ cd a
$ ls
dir e
29116 f
2557 g
62596 h.lst
$ cd e
$ ls
584 i
$ cd ..
$ cd ..
$ cd d
$ ls
4060174 j
8033020 d.log
5626152 d.ext
7214296 k
";

        let solution = solve(&input[..]);
        assert_eq!("95437", solution.0);
    }

    #[test]
    fn part1_and_part2() {
        let answer = solve(&aoc_lib::io::read_input(DAY).unwrap());
        assert_eq!("2061777", answer.0);
        assert_eq!("4473403", answer.1);
    }
}

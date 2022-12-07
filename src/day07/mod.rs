use ahash::AHashMap;
use log::{debug, trace};

use aoc_lib::parse;

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
struct Directory {
    path: String,
}

impl Directory {
    pub fn is_root(&self) -> bool {
        return self.path.len() == 1;
    }

    // TODO: make mutable
    pub fn parent(&self) -> Directory {
        debug!("going to parent of {}", self.path);
        let pos = self.path.rfind("/").unwrap();
        if pos == 0 {
            return Directory {
                path: String::from("/"),
            };
        }
        Directory {
            path: self.path[0..pos].to_string(),
        }
    }

    // TODO: make mutable
    pub fn enter(&self, other: &str) -> Directory {
        debug!("Entering {other}");
        let n = self.path.len();
        let pos = if self.path.chars().last() == Some('/') {
            n - 1
        } else {
            n
        };
        Directory {
            path: format!("{}/{}", &self.path[0..pos], other),
        }
    }
}

pub fn solve(input: &[u8]) -> (String, String) {
    let mut input = input;

    let mut dirs: AHashMap<Directory, u64> = AHashMap::with_capacity(128);
    let start = String::with_capacity(1024);
    let mut cwd = Directory { path: start };

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
                            cwd = cwd.parent();
                        }
                        b"/" => {
                            cwd = Directory {
                                path: String::from("/"),
                            }
                        }
                        _ => {
                            cwd = cwd.enter(&String::from_utf8_lossy(dest));
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
                                        tmp = tmp.parent();
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
    debug!("=== dirs ====");
    for (k, v) in &dirs {
        debug!("{}: {:?}", k.path, v);
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

    fn init() {
        let _ = env_logger::builder().is_test(true).try_init();
    }

    #[test]
    fn example() {
        init();

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

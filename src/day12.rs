use bitvector::BitVector;
use itertools::Itertools;
use rayon::{iter::ParallelBridge, iter::ParallelIterator};
use std::fs;

#[derive(Debug, Clone, Copy)]
enum Condition {
    Operational,
    Damaged,
    Unknown,
}

impl From<char> for Condition {
    fn from(c: char) -> Self {
        match c {
            '.' => Self::Operational,
            '#' => Self::Damaged,
            '?' => Self::Unknown,
            _ => panic!("╰(⇀︿⇀)つ-]═───"),
        }
    }
}

#[derive(Debug)]
struct Record {
    springs: Vec<Condition>,
    checksum: Vec<u32>,
    checksum_sum: usize,
}

fn calc_checksum_bv(v: &BitVector) -> Vec<u32> {
    if v.is_empty() {
        return Vec::new();
    }
    let mut r = Vec::new();
    let mut prev = v.iter().next().unwrap();
    let mut clen = 1;
    v.iter().skip(1).for_each(|b| {
        if b - 1 == prev {
            clen += 1;
        } else {
            if clen != 0 {
                r.push(clen);
            }
            clen = 1;
        }
        prev = b;
    });
    if clen != 0 {
        r.push(clen);
    }
    r
}

fn early_match_checksum(v: &BitVector, checksum: &Vec<u32>) -> bool {
    if v.is_empty() {
        return true;
    }
    let mut cur_i = 0;
    let mut prev = v.iter().next().unwrap();
    let mut clen = 1;
    !v.iter().skip(1).any(|b| {
        if b - 1 == prev {
            clen += 1;
        } else {
            if clen != 0 {
                if cur_i >= checksum.len() {
                    return true;
                }
                if clen != checksum[cur_i] {
                    return true;
                }
                cur_i += 1;
            }
            clen = 1;
        }
        prev = b;
        false
    })
    // we intentionally don't check the last checksum segment as it might be not finished
}

impl Record {
    fn from(s: &str, ext: u32) -> Self {
        let (springs, checksum) = s.split_once(' ').unwrap();
        let springs_base = springs.chars().map(Condition::from).collect_vec();
        let mut springs = springs_base.clone();
        for _ in 0..(ext - 1) {
            springs = [springs, springs_base.clone()].join(&Condition::Unknown);
        }
        let checksum = checksum
            .split(',')
            .map(|s| s.parse::<u32>().unwrap())
            .collect_vec()
            .repeat(ext as usize);
        let checksum_sum = checksum.iter().sum::<u32>() as usize;
        Self {
            checksum,
            springs,
            checksum_sum,
        }
    }

    fn early_exit(&self, bv: &BitVector, level: usize) -> bool {
        let vlen = bv.len();
        // too many bits set
        if vlen > self.checksum_sum {
            return true;
        }
        let bits_left = self.springs.len() - level;
        // not enough bits set
        if vlen + bits_left < self.checksum_sum {
            return true;
        }
        !early_match_checksum(bv, &self.checksum)
    }

    fn solve_rec(&self, bv: &mut BitVector, level: usize) -> usize {
        if level == self.springs.len() {
            return if calc_checksum_bv(bv) == self.checksum {
                1
            } else {
                0
            };
        }
        if level > 0 && self.early_exit(bv, level) {
            return 0;
        }
        match self.springs[level] {
            Condition::Unknown => {
                bv.insert(level);
                let r = self.solve_rec(bv, level + 1);
                bv.remove(level);
                r + self.solve_rec(bv, level + 1)
            }
            Condition::Damaged => {
                bv.insert(level);
                let r = self.solve_rec(bv, level + 1);
                bv.remove(level);
                r
            }
            Condition::Operational => {
                bv.remove(level);
                self.solve_rec(bv, level + 1)
            }
        }
    }

    fn solve(&self) -> usize {
        self.solve_rec(&mut BitVector::new(self.springs.len()), 0)
    }
}

fn solution(input: &str, ext: u32) -> usize {
    input
        .lines()
        .enumerate()
        .par_bridge()
        .map(|(_, l)| Record::from(l, ext))
        .map(|r| r.solve())
        .sum()
}

#[test]
fn test_run() {
    let input = fs::read_to_string("src/inputs/aoc12s.in").unwrap();
    assert_eq!(solution(&input, 1), 21);
    assert_eq!(solution(&input, 5), 525152);
    let input = fs::read_to_string("src/inputs/aoc12.in").unwrap();
    assert_eq!(solution(&input, 1), 7541);
    // this is obviously tooooooo slowwww
    // assert_eq!(solution(&input, 5), 0);
}

pub fn run() -> (String, String) {
    let input = fs::read_to_string("src/inputs/aoc12.in").unwrap();
    (
        solution(&input, 1).to_string(),
        //solution(&input, 5).to_string(),
        "TBD".to_string(),
    )
}

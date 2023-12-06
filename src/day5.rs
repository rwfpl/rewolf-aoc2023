use std::{collections::BTreeMap, fs, str::Lines};

use itertools::Itertools;
use rayon::prelude::*;

#[derive(Debug)]
struct RangeMap {
    m: BTreeMap<u64, (u64, u64)>,
}

impl From<&mut &mut Lines<'_>> for RangeMap {
    fn from(v: &mut &mut Lines) -> Self {
        Self {
            m: v.map_while(|l| {
                if l.is_empty() {
                    None
                } else {
                    let d = l
                        .split(' ')
                        .map(|n| n.parse::<u64>().unwrap())
                        .collect_vec();
                    Some((d[1], (d[0], d[2])))
                }
            })
            .collect(),
        }
    }
}

impl RangeMap {
    fn at(&self, v: u64) -> u64 {
        let ub = self.m.upper_bound(std::ops::Bound::Included(&v));
        if let Some((ub_k, ub_v)) = ub.key_value() {
            if (*ub_k..(*ub_k + ub_v.1)).contains(&v) {
                return ub_v.0 + v - *ub_k;
            }
        }
        v
    }
}

fn read_seeds(lit: &mut Lines) -> Vec<u64> {
    lit.next()
        .unwrap()
        .split_once(':')
        .unwrap()
        .1
        .split(' ')
        .filter(|d| !d.is_empty())
        .map(|s| s.parse::<u64>().unwrap())
        .collect_vec()
}

fn read_maps(mut lit: &mut Lines) -> Vec<RangeMap> {
    let mut r = Vec::new();
    while lit.next().is_some() {
        r.push(RangeMap::from(&mut lit));
    }
    r
}

fn solution(input: &str) -> (u64, u64) {
    let mut lit = input.lines();
    let seeds = read_seeds(&mut lit);
    lit.next(); // empty line
    let tr = read_maps(&mut lit);
    (
        seeds
            .iter()
            .map(|s| tr.iter().fold(*s, |acc, v| v.at(acc)))
            .min()
            .unwrap(),
        seeds
            .chunks(2)
            .map(|chunk| {
                (chunk[0]..(chunk[0] + chunk[1]))
                    .into_par_iter()
                    .map(|s| tr.iter().fold(s, |acc, v| v.at(acc)))
                    .min()
                    .unwrap()
            })
            .min()
            .unwrap(),
    )
}

#[test]
fn test_run() {
    let input = fs::read_to_string("src/inputs/aoc5s.in").unwrap();
    assert_eq!(solution(&input), (35, 46));
    let input = fs::read_to_string("src/inputs/aoc5.in").unwrap();
    assert_eq!(solution(&input), (535088217, 51399228));
}

pub fn run() -> (String, String) {
    let input = fs::read_to_string("src/inputs/aoc5.in").unwrap();
    let (p1, p2) = solution(&input);
    (p1.to_string(), p2.to_string())
}

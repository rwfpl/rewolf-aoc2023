use std::fs;

use itertools::Itertools;

fn get_diffs(seq: &[i32]) -> Vec<i32> {
    seq.windows(2).map(|w| w[1] - w[0]).collect_vec()
}

fn predict(seq: &[i32]) -> (i32, i32) {
    let mut d = seq.to_owned();
    let mut last = Vec::new();
    let mut first = Vec::new();
    last.push(*d.last().unwrap());
    first.push(*d.first().unwrap());
    while !d.iter().all_equal() {
        d = get_diffs(&d);
        last.push(*d.last().unwrap());
        first.push(*d.first().unwrap());
    }
    (
        first.iter().rev().fold(0, |acc, v| *v - acc),
        last.iter().sum::<i32>(),
    )
}

fn solution(input: &str) -> (i32, i32) {
    let predictions = input
        .lines()
        .map(|l| {
            predict(
                &l.split(' ')
                    .map(|d| d.parse::<i32>().unwrap())
                    .collect_vec(),
            )
        })
        .collect_vec();
    (
        predictions.iter().map(|p| p.0).sum(),
        predictions.iter().map(|p| p.1).sum(),
    )
}

#[test]
fn test_run() {
    let input = fs::read_to_string("src/inputs/aoc9s.in").unwrap();
    assert_eq!(solution(&input), (2, 114));
    let input = fs::read_to_string("src/inputs/aoc9.in").unwrap();
    assert_eq!(solution(&input), (973, 1479011877));
}

pub fn run() -> (String, String) {
    let input = fs::read_to_string("src/inputs/aoc9.in").unwrap();
    let (p1, p2) = solution(&input);
    (p1.to_string(), p2.to_string())
}

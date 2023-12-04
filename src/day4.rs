use std::{collections::HashSet, fs};

use itertools::Itertools;

#[derive(Debug)]
struct Card {
    winning: HashSet<u32>,
    you_have: HashSet<u32>,
}

fn get_numbers(s: &str) -> HashSet<u32> {
    s.split(' ')
        .filter(|n| !n.is_empty())
        .map(|n| n.parse::<u32>().unwrap())
        .collect()
}

impl From<&str> for Card {
    fn from(s: &str) -> Self {
        let (winning, you_have) = s.split_once(':').unwrap().1.split_once('|').unwrap();
        Self {
            winning: get_numbers(winning),
            you_have: get_numbers(you_have),
        }
    }
}

impl Card {
    fn get_matches(&self) -> usize {
        self.you_have
            .iter()
            .filter(|yh| self.winning.contains(*yh))
            .count()
    }

    fn get_points(&self) -> u32 {
        let p = self.get_matches();
        if p != 0 {
            2u32.pow(p as u32 - 1)
        } else {
            0
        }
    }
}

fn p1(cards: &[Card]) -> u32 {
    cards.iter().map(|c| c.get_points()).sum()
}

fn p2(cards: &[Card]) -> u32 {
    let mut v = vec![1; cards.len()];
    cards.iter().enumerate().for_each(|(i, c)| {
        (0..c.get_matches()).for_each(|j| {
            v[i + j + 1] += v[i];
        });
    });
    v.iter().sum()
}

fn solution(input: &str) -> (u32, u32) {
    let cards = input.lines().map(Card::from).collect_vec();
    (p1(&cards), p2(&cards))
}

#[test]
fn test_run() {
    let input = fs::read_to_string("src/inputs/aoc4s.in").unwrap();
    assert_eq!(solution(&input), (13, 30));
    let input = fs::read_to_string("src/inputs/aoc4.in").unwrap();
    assert_eq!(solution(&input), (33950, 14814534));
}

pub fn run() -> (String, String) {
    let input = fs::read_to_string("src/inputs/aoc4.in").unwrap();
    let (p1, p2) = solution(&input);
    (p1.to_string(), p2.to_string())
}

use std::cmp::Ordering;
use std::collections::HashMap;
use std::fs;

use itertools::Itertools;

#[derive(Debug, Eq, PartialEq, PartialOrd, Ord, Hash)]
struct Card {
    c: u8,
    joker: bool,
}

impl Card {
    fn from(v: char, joker: bool) -> Self {
        Self {
            c: match v {
                'A' => 14,
                'K' => 13,
                'Q' => 12,
                'J' => {
                    if joker {
                        0
                    } else {
                        11
                    }
                }
                'T' => 10,
                '2'..='9' => v.to_digit(10).unwrap() as u8,
                _ => panic!("(ノಠ益ಠ)ノ彡┻━┻"),
            },
            joker,
        }
    }
}

#[derive(Debug)]
struct Hand {
    cards: [Card; 5],
    joker: bool,
}

impl Hand {
    fn from(s: &str, joker: bool) -> Self {
        Self {
            cards: core::array::from_fn(|i| Card::from(s.chars().nth(i).unwrap(), joker)),
            joker,
        }
    }

    fn score(&self) -> u8 {
        if self.joker {
            self.score_joker()
        } else {
            self.score_no_joker()
        }
    }

    fn score_joker(&self) -> u8 {
        let score_no_joker = self.score_no_joker();
        let joker_cnt = self.cards.iter().filter(|c| c.c == 0).count();
        match score_no_joker {
            6 => score_no_joker,
            5 => match joker_cnt {
                1 | 4 => 6,
                _ => score_no_joker,
            },
            4 => match joker_cnt {
                2 | 3 => 6,
                _ => score_no_joker,
            },
            3 => match joker_cnt {
                1 | 3 => 5,
                _ => score_no_joker,
            },
            2 => match joker_cnt {
                1 => 4,
                2 => 5,
                _ => score_no_joker,
            },
            1 => match joker_cnt {
                1 | 2 => 3,
                _ => score_no_joker,
            },
            0 => match joker_cnt {
                1 => 1,
                _ => score_no_joker,
            },
            _ => panic!("🤡"),
        }
    }

    fn score_no_joker(&self) -> u8 {
        let mut m: HashMap<&Card, u8> = HashMap::new();
        self.cards
            .iter()
            .for_each(|card| *m.entry(card).or_default() += 1);
        let lens = m.values().sorted().collect_vec();
        match lens[..] {
            [5] => 6,
            [1, 4] => 5,
            [2, 3] => 4,
            [1, 1, 3] => 3,
            [1, 2, 2] => 2,
            [1, 1, 1, 2] => 1,
            [1, 1, 1, 1, 1] => 0,
            _ => panic!("(╯°□°)╯︵ ┻━┻: {:?}", lens),
        }
    }
}

impl Ord for Hand {
    fn cmp(&self, other: &Self) -> Ordering {
        let self_score = self.score();
        let other_score = other.score();
        match self_score.cmp(&other_score) {
            Ordering::Less => Ordering::Less,
            Ordering::Greater => Ordering::Greater,
            Ordering::Equal => (0..5)
                .find_map(|i| match self.cards[i].cmp(&other.cards[i]) {
                    Ordering::Equal => None,
                    Ordering::Less => Some(Ordering::Less),
                    Ordering::Greater => Some(Ordering::Greater),
                })
                .unwrap(),
        }
    }
}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for Hand {
    fn eq(&self, other: &Self) -> bool {
        self.score() == other.score()
    }
}

impl Eq for Hand {}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
struct Round {
    hand: Hand,
    bid: u32,
}

impl Round {
    fn from(s: &str, joker: bool) -> Self {
        let (hand, bid) = s.split_once(' ').unwrap();
        Self {
            hand: Hand::from(hand, joker),
            bid: bid.parse::<u32>().unwrap(),
        }
    }
}

#[derive(Debug)]
struct Game {
    rounds: Vec<Round>,
}

impl Game {
    fn from(s: &str, joker: bool) -> Self {
        Self {
            rounds: s
                .lines()
                .map(|l| Round::from(l, joker))
                .sorted()
                .collect_vec(),
        }
    }
}

fn solution(input: &str, joker: bool) -> usize {
    Game::from(input, joker)
        .rounds
        .iter()
        .enumerate()
        .map(|(i, r)| (i + 1) * r.bid as usize)
        .sum()
}

#[test]
fn test_run() {
    let input = fs::read_to_string("src/inputs/aoc7s.in").unwrap();
    assert_eq!(solution(&input, false), 6440);
    assert_eq!(solution(&input, true), 5905);
    let input = fs::read_to_string("src/inputs/aoc7.in").unwrap();
    assert_eq!(solution(&input, false), 247961593);
    assert_eq!(solution(&input, true), 248750699);
}

pub fn run() -> (String, String) {
    let input = fs::read_to_string("src/inputs/aoc7.in").unwrap();
    (
        solution(&input, false).to_string(),
        solution(&input, true).to_string(),
    )
}

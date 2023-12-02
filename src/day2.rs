use itertools::Itertools;
use std::{cmp, fs};

#[derive(Debug)]
enum Cube {
    Red(u32),
    Green(u32),
    Blue(u32),
}

impl From<&str> for Cube {
    fn from(value: &str) -> Self {
        let (cnt, color) = value.trim().split_once(' ').unwrap();
        match color {
            "red" => Cube::Red(cnt.parse::<u32>().unwrap()),
            "green" => Cube::Green(cnt.parse::<u32>().unwrap()),
            "blue" => Cube::Blue(cnt.parse::<u32>().unwrap()),
            _ => panic!("wrong color: {color}"),
        }
    }
}

#[derive(Debug)]
struct Round {
    cubes: Vec<Cube>,
}

impl From<&str> for Round {
    fn from(value: &str) -> Self {
        Self {
            cubes: value.split(',').map(Cube::from).collect(),
        }
    }
}

impl Round {
    fn get_max(&self) -> (u32, u32, u32) {
        self.cubes.iter().fold((0, 0, 0), |(r, g, b), c| match c {
            Cube::Red(v) => (cmp::max(*v, r), g, b),
            Cube::Green(v) => (r, cmp::max(*v, g), b),
            Cube::Blue(v) => (r, g, cmp::max(*v, b)),
        })
    }
}

#[derive(Debug)]
struct Game {
    id: u32,
    rounds: Vec<Round>,
}

impl From<&str> for Game {
    fn from(value: &str) -> Self {
        let (game, rounds) = value.split_once(':').unwrap();
        Self {
            id: game.split(' ').nth(1).unwrap().parse::<u32>().unwrap(),
            rounds: rounds.split(';').map(Round::from).collect(),
        }
    }
}

impl Game {
    fn is_possible(&self) -> bool {
        self.rounds.iter().all(|r| {
            r.cubes.iter().all(|c| match c {
                Cube::Red(v) => *v <= 12,
                Cube::Green(v) => *v <= 13,
                Cube::Blue(v) => *v <= 14,
            })
        })
    }

    fn get_power(&self) -> u32 {
        let (r, g, b) = self.rounds.iter().fold((0, 0, 0), |acc, r| {
            let (r, g, b) = r.get_max();
            (cmp::max(acc.0, r), cmp::max(acc.1, g), cmp::max(acc.2, b))
        });
        r * g * b
    }
}

fn solution(input: &str) -> (u32, u32) {
    let games = input.lines().map(Game::from).collect_vec();
    (
        games.iter().filter(|g| g.is_possible()).map(|g| g.id).sum(),
        games.iter().map(|g| g.get_power()).sum(),
    )
}

#[test]
fn test_run() {
    let input = fs::read_to_string("src/inputs/aoc2s.in").unwrap();
    assert_eq!(solution(&input), (8, 2286));
    let input = fs::read_to_string("src/inputs/aoc2.in").unwrap();
    assert_eq!(solution(&input), (2101, 0));
}

pub fn run() -> (String, String) {
    let input = fs::read_to_string("src/inputs/aoc2.in").unwrap();
    let (p1, p2) = solution(&input);
    (p1.to_string(), p2.to_string())
}

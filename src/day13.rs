use std::fs;

use itertools::Itertools;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum Tile {
    Ash,
    Rock,
}

impl From<char> for Tile {
    fn from(c: char) -> Self {
        match c {
            '.' => Self::Ash,
            '#' => Self::Rock,
            _ => panic!("(‡▼益▼)"),
        }
    }
}

#[derive(Debug)]
struct Pattern<const SMUDGE: usize> {
    p: Vec<Vec<Tile>>,
    p_rotated: Vec<Vec<Tile>>,
}

impl<const SMUDGE: usize> From<&str> for Pattern<SMUDGE> {
    fn from(s: &str) -> Self {
        let p = s
            .lines()
            .map(|l| l.chars().map(Tile::from).collect_vec())
            .collect_vec();
        Self {
            p_rotated: (0..p[0].len())
                .map(|x| (0..p.len()).map(|y| p[y][x]).collect_vec())
                .collect_vec(),
            p,
        }
    }
}

impl<const SMUDGE: usize> Pattern<SMUDGE> {
    fn vec_diff(&self, v1: &[Tile], v2: &[Tile]) -> usize {
        v1.iter()
            .zip(v2)
            .map(|(t1, t2)| t1 == t2)
            .filter(|p| !*p)
            .count()
    }

    fn get_candidates(&self, vec: &[Vec<Tile>]) -> Vec<usize> {
        vec.windows(2)
            .enumerate()
            .filter_map(|(i, r)| {
                if self.vec_diff(&r[0], &r[1]) <= SMUDGE {
                    Some(i)
                } else {
                    None
                }
            })
            .collect_vec()
    }

    fn get_horizontal_candidates(&self) -> Vec<usize> {
        self.get_candidates(&self.p)
    }

    fn get_vertical_candidates(&self) -> Vec<usize> {
        self.get_candidates(&self.p_rotated)
    }

    fn verify_mirror(&self, vec: &[Vec<Tile>], pos: usize) -> bool {
        vec[0..=pos]
            .iter()
            .rev()
            .zip(vec[pos + 1..].iter())
            .map(|(v1, v2)| self.vec_diff(v1, v2))
            .sum::<usize>()
            == SMUDGE
    }

    fn verify_horizontal_mirror(&self, pos: usize) -> bool {
        self.verify_mirror(&self.p, pos)
    }

    fn verify_vertical_mirror(&self, pos: usize) -> bool {
        self.verify_mirror(&self.p_rotated, pos)
    }

    fn find_mirror_score(&self) -> usize {
        if let Some(x) = self
            .get_horizontal_candidates()
            .iter()
            .find(|p| self.verify_horizontal_mirror(**p))
        {
            (x + 1) * 100
        } else if let Some(x) = self
            .get_vertical_candidates()
            .iter()
            .find(|p| self.verify_vertical_mirror(**p))
        {
            x + 1
        } else {
            panic!("( ಠ ಠ )");
        }
    }
}

fn solution<const SMUDGE: usize>(input: &str) -> usize {
    input
        .replace("\r\n", "\n")
        .split("\n\n")
        .map(|l| Pattern::<SMUDGE>::from(l).find_mirror_score())
        .sum()
}

#[test]
fn test_run() {
    let input = fs::read_to_string("src/inputs/aoc13s.in").unwrap();
    assert_eq!(solution::<0>(&input), 405);
    assert_eq!(solution::<1>(&input), 400);
    let input = fs::read_to_string("src/inputs/aoc13.in").unwrap();
    assert_eq!(solution::<0>(&input), 30575);
    assert_eq!(solution::<1>(&input), 37478);
}

pub fn run() -> (String, String) {
    let input = fs::read_to_string("src/inputs/aoc13.in").unwrap();
    (
        solution::<0>(&input).to_string(),
        solution::<1>(&input).to_string(),
    )
}

use std::{cmp::minmax, collections::HashSet, fs};

use itertools::Itertools;

#[derive(Debug, PartialEq, Eq, Hash)]
struct Pos(usize, usize);

#[derive(Debug)]
enum Tile {
    Galaxy,
    Empty,
}

impl From<char> for Tile {
    fn from(c: char) -> Self {
        match c {
            '.' => Self::Empty,
            '#' => Self::Galaxy,
            _ => panic!("(☞ﾟヮﾟ)☞ ┻━┻"),
        }
    }
}

#[derive(Debug)]
struct Cosmos {
    empty_rows: HashSet<usize>,
    empty_cols: HashSet<usize>,
    galaxies: HashSet<Pos>,
}

impl From<&str> for Cosmos {
    fn from(v: &str) -> Self {
        let g = v
            .lines()
            .map(|l| l.chars().map(Tile::from).collect_vec())
            .collect_vec();
        let empty_rows: HashSet<usize> = HashSet::from_iter(
            g.iter()
                .enumerate()
                .filter(|(_, row)| row.iter().all(|r| matches!(r, Tile::Empty)))
                .map(|(i, _)| i),
        );
        let empty_cols: HashSet<usize> = HashSet::from_iter(
            (0..g[0].len()).filter(|i| (0..g.len()).all(|j| matches!(g[j][*i], Tile::Empty))),
        );
        let galaxies: HashSet<Pos> =
            HashSet::from_iter(g.iter().enumerate().flat_map(|(i, row)| {
                row.iter()
                    .enumerate()
                    .filter(|(_, t)| matches!(t, Tile::Galaxy))
                    .map(|(j, _)| Pos(j, i))
                    .collect_vec()
            }));
        Self {
            empty_rows,
            empty_cols,
            galaxies,
        }
    }
}

fn calc_dist(v1: usize, v2: usize, exp: usize, m: &HashSet<usize>) -> usize {
    let [vmin, vmax] = minmax(v1, v2);
    let c = m.iter().filter(|col| (vmin..vmax).contains(col)).count();
    let exp_c = if c != 0 { c * (exp - 1) } else { 0 };
    vmax - vmin + exp_c
}

impl Cosmos {
    fn get_distance(&self, g1: &Pos, g2: &Pos, expansion_mul: usize) -> usize {
        calc_dist(g1.0, g2.0, expansion_mul, &self.empty_cols)
            + calc_dist(g1.1, g2.1, expansion_mul, &self.empty_rows)
    }

    fn solve_for(&self, mul: usize) -> usize {
        self.galaxies
            .iter()
            .tuple_combinations::<(_, _)>()
            .map(|(g1, g2)| self.get_distance(g1, g2, mul))
            .sum()
    }
}

fn solution(input: &str) -> (usize, usize) {
    let c = Cosmos::from(input);
    (c.solve_for(2), c.solve_for(1000000))
}

#[test]
fn test_run() {
    let input = fs::read_to_string("src/inputs/aoc11s.in").unwrap();
    assert_eq!(solution(&input), (374, 82000210));
    let input = fs::read_to_string("src/inputs/aoc11.in").unwrap();
    assert_eq!(solution(&input), (9974721, 702770569197));
}

pub fn run() -> (String, String) {
    let input = fs::read_to_string("src/inputs/aoc11.in").unwrap();
    let (p1, p2) = solution(&input);
    (p1.to_string(), p2.to_string())
}

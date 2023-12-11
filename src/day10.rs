use std::{collections::HashSet, fs};

use itertools::Itertools;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Point(i32, i32);

#[derive(Debug)]
enum Tile {
    Vertical([Point; 2]),
    Horizontal([Point; 2]),
    UpRight([Point; 2]),
    UpLeft([Point; 2]),
    DownLeft([Point; 2]),
    DownRight([Point; 2]),
    Start([Point; 2]),
    Ground,
}

impl Tile {
    fn from(c: char, pos: &Point) -> Self {
        match c {
            '|' => Self::Vertical([Point(pos.0, pos.1 - 1), Point(pos.0, pos.1 + 1)]),
            '-' => Self::Horizontal([Point(pos.0 - 1, pos.1), Point(pos.0 + 1, pos.1)]),
            'L' => Self::UpRight([Point(pos.0, pos.1 - 1), Point(pos.0 + 1, pos.1)]),
            'J' => Self::UpLeft([Point(pos.0, pos.1 - 1), Point(pos.0 - 1, pos.1)]),
            '7' => Self::DownLeft([Point(pos.0, pos.1 + 1), Point(pos.0 - 1, pos.1)]),
            'F' => Self::DownRight([Point(pos.0, pos.1 + 1), Point(pos.0 + 1, pos.1)]),
            'S' => Self::Start([Point(pos.0, pos.1), Point(pos.0, pos.1)]),
            '.' => Self::Ground,
            _ => panic!("┌(° ͜ʖ͡°)┘"),
        }
    }

    fn get_next(&self) -> Option<&[Point; 2]> {
        match self {
            Self::DownLeft(ct)
            | Self::DownRight(ct)
            | Self::Horizontal(ct)
            | Self::Vertical(ct)
            | Self::UpLeft(ct)
            | Self::UpRight(ct) => Some(ct),
            _ => None,
        }
    }

    fn connects_to(&self, p: &Point) -> bool {
        if let Some(ct) = self.get_next() {
            ct.contains(p)
        } else {
            false
        }
    }
}

#[derive(Debug)]
struct Grid {
    tiles: Vec<Vec<Tile>>,
    start: Point,
}

impl From<&str> for Grid {
    fn from(s: &str) -> Self {
        let mut start = Point(0, 0);
        let tiles = s
            .lines()
            .enumerate()
            .map(|(y, l)| {
                l.chars()
                    .enumerate()
                    .map(|(x, c)| Tile::from(c, &Point(x as i32, y as i32)))
                    .inspect(|t| {
                        if let Tile::Start(pos) = t {
                            start = pos[0]
                        }
                    })
                    .collect_vec()
            })
            .collect_vec();
        Self { tiles, start }
    }
}

impl Grid {
    fn in_grid(&self, p: &Point) -> bool {
        p.0 >= 0 && p.0 < self.tiles[0].len() as i32 && p.1 >= 0 && p.1 < self.tiles.len() as i32
    }

    fn at(&self, p: &Point) -> &Tile {
        &self.tiles[p.1 as usize][p.0 as usize]
    }

    fn get_starting_points(&self) -> Vec<Point> {
        [Point(-1, 0), Point(0, -1), Point(1, 0), Point(0, 1)]
            .iter()
            .map(|p| Point(self.start.0 + p.0, self.start.1 + p.1))
            .filter(|p| self.in_grid(p) && self.at(p).connects_to(&self.start))
            .collect_vec()
    }

    fn get_next(&self, p: &Point, visited: &HashSet<Point>) -> Option<Point> {
        let np = self
            .at(p)
            .get_next()
            .unwrap()
            .iter()
            .filter(|p| !visited.contains(p))
            .collect_vec();
        assert!(np.len() < 2);
        if np.is_empty() {
            None
        } else {
            Some(*np[0])
        }
    }

    fn get_farthest_distance(&self) -> usize {
        let mut cur = self.get_starting_points();
        let mut visited: HashSet<Point> = HashSet::from_iter(cur.iter().cloned());
        visited.insert(self.start);
        loop {
            cur = cur
                .iter()
                .map_while(|p| self.get_next(p, &visited))
                .collect_vec();
            if cur.len() < 2 {
                break;
            }
            visited.extend(cur.iter());
        }
        visited.len() / 2
    }
}

fn solution(input: &str) -> usize {
    Grid::from(input).get_farthest_distance()
}

#[test]
fn test_run() {
    let input = fs::read_to_string("src/inputs/aoc10s.in").unwrap();
    assert_eq!(solution(&input), 4);
    let input = fs::read_to_string("src/inputs/aoc10s2.in").unwrap();
    assert_eq!(solution(&input), 8);
    let input = fs::read_to_string("src/inputs/aoc10.in").unwrap();
    assert_eq!(solution(&input), 7093);
}

pub fn run() -> (String, String) {
    let input = fs::read_to_string("src/inputs/aoc10.in").unwrap();
    (solution(&input).to_string(), "TBD".to_string())
}

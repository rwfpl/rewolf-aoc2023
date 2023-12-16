use itertools::Itertools;
use std::{
    collections::{HashSet, VecDeque},
    fs,
};

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum MirrorType {
    Left,
    Right,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum SplitterType {
    Vertical,
    Horizontal,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum Tile {
    Empty,
    Mirror(MirrorType),
    Splitter(SplitterType),
}

impl From<char> for Tile {
    fn from(c: char) -> Self {
        match c {
            '.' => Self::Empty,
            '/' => Self::Mirror(MirrorType::Right),
            '\\' => Self::Mirror(MirrorType::Left),
            '|' => Self::Splitter(SplitterType::Vertical),
            '-' => Self::Splitter(SplitterType::Horizontal),
            _ => panic!("(‡▼益▼)"),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Direction {
    Left,
    Right,
    Up,
    Down,
}

impl Direction {
    fn apply_mirror(&self, mt: MirrorType) -> Self {
        match self {
            Self::Down => match mt {
                MirrorType::Left => Self::Right,
                MirrorType::Right => Self::Left,
            },
            Self::Up => match mt {
                MirrorType::Left => Self::Left,
                MirrorType::Right => Self::Right,
            },
            Self::Left => match mt {
                MirrorType::Left => Self::Up,
                MirrorType::Right => Self::Down,
            },
            Self::Right => match mt {
                MirrorType::Left => Self::Down,
                MirrorType::Right => Self::Up,
            },
        }
    }

    fn apply_splitter(&self, st: SplitterType) -> Option<[Self; 2]> {
        match self {
            Self::Down | Self::Up => match st {
                SplitterType::Horizontal => Some([Self::Left, Self::Right]),
                SplitterType::Vertical => None,
            },
            Self::Right | Self::Left => match st {
                SplitterType::Horizontal => None,
                SplitterType::Vertical => Some([Self::Up, Self::Down]),
            },
        }
    }
}

#[derive(Debug, Clone, Copy)]
struct Point {
    x: i32,
    y: i32,
}

impl Point {
    fn from(x: i32, y: i32) -> Self {
        Self { x, y }
    }

    fn next(&self, dir: &Direction) -> Self {
        match dir {
            Direction::Down => Self::from(self.x, self.y + 1),
            Direction::Up => Self::from(self.x, self.y - 1),
            Direction::Left => Self::from(self.x - 1, self.y),
            Direction::Right => Self::from(self.x + 1, self.y),
        }
    }
}

#[derive(Debug)]
struct Beam {
    p: Point,
    dir: Direction,
}

impl Beam {
    fn from(p: &Point, dir: Direction) -> Self {
        Self { p: *p, dir }
    }
}

struct Grid {
    g: Vec<Vec<Tile>>,
}

impl From<&str> for Grid {
    fn from(s: &str) -> Self {
        Self {
            g: s.lines()
                .map(|l| l.chars().map(Tile::from).collect_vec())
                .collect_vec(),
        }
    }
}

impl Grid {
    fn in_grid(&self, p: &Point) -> bool {
        p.x >= 0 && p.x < self.g[0].len() as i32 && p.y >= 0 && p.y < self.g.len() as i32
    }

    fn get_tile(&self, p: &Point) -> Tile {
        self.g[p.y as usize][p.x as usize]
    }

    fn trace(&self, b: Beam) -> usize {
        let mut energized: Vec<Vec<HashSet<Direction>>> =
            vec![vec![HashSet::new(); self.g[0].len()]; self.g.len()];
        let mut q: VecDeque<Beam> = VecDeque::new();
        q.push_back(b);
        while !q.is_empty() {
            let b = q.pop_front().unwrap();
            if !self.in_grid(&b.p) {
                continue;
            }
            if energized[b.p.y as usize][b.p.x as usize].contains(&b.dir) {
                continue;
            }
            energized[b.p.y as usize][b.p.x as usize].insert(b.dir);
            match self.get_tile(&b.p) {
                Tile::Empty => q.push_back(Beam::from(&b.p.next(&b.dir), b.dir)),
                Tile::Mirror(mt) => {
                    let new_dir = b.dir.apply_mirror(mt);
                    q.push_back(Beam::from(&b.p.next(&new_dir), new_dir))
                }
                Tile::Splitter(st) => {
                    if let Some(new_dir) = b.dir.apply_splitter(st) {
                        new_dir
                            .iter()
                            .for_each(|dir| q.push_back(Beam::from(&b.p.next(dir), *dir)))
                    } else {
                        q.push_back(Beam::from(&b.p.next(&b.dir), b.dir))
                    }
                }
            }
        }
        energized
            .iter()
            .map(|row| row.iter().filter(|t| !t.is_empty()).count())
            .sum()
    }
}

fn solution(input: &str) -> usize {
    Grid::from(input).trace(Beam::from(&Point::from(0, 0), Direction::Right))
}

fn solution2(input: &str) -> usize {
    let g = Grid::from(input);
    let w = g.g[0].len();
    let h = g.g.len();
    let top_max = (0..w)
        .map(|x| g.trace(Beam::from(&Point::from(x as i32, 0), Direction::Down)))
        .max()
        .unwrap();
    let bottom_max = (0..w)
        .map(|x| {
            g.trace(Beam::from(
                &Point::from(x as i32, h as i32 - 1),
                Direction::Up,
            ))
        })
        .max()
        .unwrap();
    let left_max = (0..h)
        .map(|y| g.trace(Beam::from(&Point::from(0, y as i32), Direction::Right)))
        .max()
        .unwrap();
    let right_max = (0..h)
        .map(|y| {
            g.trace(Beam::from(
                &Point::from(w as i32 - 1, y as i32),
                Direction::Left,
            ))
        })
        .max()
        .unwrap();
    *[top_max, bottom_max, left_max, right_max]
        .iter()
        .max()
        .unwrap()
}

#[test]
fn test_run() {
    let input = fs::read_to_string("src/inputs/aoc16s.in").unwrap();
    assert_eq!(solution(&input), 46);
    assert_eq!(solution2(&input), 51);
    let input = fs::read_to_string("src/inputs/aoc16.in").unwrap();
    assert_eq!(solution(&input), 7939);
    assert_eq!(solution2(&input), 8318);
}

pub fn run() -> (String, String) {
    let input = fs::read_to_string("src/inputs/aoc16.in").unwrap();
    (solution(&input).to_string(), solution2(&input).to_string())
}

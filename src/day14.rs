use core::fmt;
use itertools::Itertools;
use std::fs;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum Tile {
    Empty,
    Cube,
    Oval,
}

impl From<char> for Tile {
    fn from(c: char) -> Self {
        match c {
            '.' => Self::Empty,
            '#' => Self::Cube,
            'O' => Self::Oval,
            _ => panic!("(‡▼益▼)"),
        }
    }
}

impl fmt::Display for Tile {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::Cube => '#',
                Self::Empty => '.',
                Self::Oval => 'O',
            }
        )
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

impl fmt::Display for Grid {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            self.g.iter().map(|row| row.iter().join("")).join("\n")
        )
    }
}

fn detect_period(v: &[usize]) -> Option<usize> {
    if v.len() < 10 {
        return None;
    }
    (5..=v.len() / 2).find(|p| v[v.len() - p..v.len()] == v[v.len() - 2 * p..v.len() - p])
}

impl Grid {
    fn in_grid(&self, p: &Point) -> bool {
        p.x >= 0 && p.x < self.g[0].len() as i32 && p.y >= 0 && p.y < self.g.len() as i32
    }

    fn can_move_to(&self, p: &Point) -> bool {
        if self.in_grid(p) {
            match self.g[p.y as usize][p.x as usize] {
                Tile::Empty => true,
                Tile::Cube | Tile::Oval => false,
            }
        } else {
            false
        }
    }

    fn get_tile(&self, p: &Point) -> Option<Tile> {
        if self.in_grid(p) {
            Some(self.g[p.y as usize][p.x as usize])
        } else {
            None
        }
    }

    fn roll(&mut self, p: &Point, dir: &Point) {
        if let Some(Tile::Cube | Tile::Empty) = self.get_tile(p) {
            return;
        }
        let mut cp = Point::from(p.x + dir.x, p.y + dir.y);
        while self.can_move_to(&cp) {
            cp.y += dir.y;
            cp.x += dir.x;
        }
        cp.y -= dir.y;
        cp.x -= dir.x;
        self.g[p.y as usize][p.x as usize] = Tile::Empty;
        self.g[cp.y as usize][cp.x as usize] = Tile::Oval;
    }

    fn tilt(&mut self, n: usize) -> usize {
        let mut load_history = Vec::new();
        if let Some((i, period)) = (0..n).find_map(|i| {
            let dirs = [
                Point::from(0, -1),
                Point::from(-1, 0),
                Point::from(0, 1),
                Point::from(1, 0),
            ];
            let h = self.g.len();
            let w = self.g[0].len();
            match i % 4 {
                0 | 1 => {
                    (0..h).for_each(|y| {
                        (0..w).for_each(|x| {
                            self.roll(&Point::from(x as i32, y as i32), &dirs[i % dirs.len()])
                        })
                    });
                }
                2 | 3 => {
                    (0..h).rev().for_each(|y| {
                        (0..w).rev().for_each(|x| {
                            self.roll(&Point::from(x as i32, y as i32), &dirs[i % dirs.len()])
                        })
                    });
                }
                _ => panic!("( ͡~ ͜ʖ ͡°)"),
            };
            load_history.push(self.get_load());
            detect_period(&load_history).map(|p| (i, p))
        }) {
            let period_start = i - period - 1;
            let period_index = (n - period_start) % period;
            load_history[period_start + period_index - 1]
        } else {
            self.get_load()
        }
    }

    fn get_load(&self) -> usize {
        self.g
            .iter()
            .enumerate()
            .map(|(y, row)| {
                row.iter().filter(|t| matches!(t, Tile::Oval)).count() * (self.g.len() - y)
            })
            .sum()
    }
}

fn solution(input: &str, n: usize) -> usize {
    Grid::from(input).tilt(n)
}

#[test]
fn test_run() {
    let input = fs::read_to_string("src/inputs/aoc14s.in").unwrap();
    assert_eq!(solution(&input, 1), 136);
    assert_eq!(solution(&input, 4000000000), 64);
    let input = fs::read_to_string("src/inputs/aoc14.in").unwrap();
    assert_eq!(solution(&input, 1), 110407);
    assert_eq!(solution(&input, 4000000000), 87273);
}

pub fn run() -> (String, String) {
    let input = fs::read_to_string("src/inputs/aoc14.in").unwrap();
    (
        solution(&input, 1).to_string(),
        solution(&input, 4000000000).to_string(),
    )
}

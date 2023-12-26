use itertools::Itertools;
use std::collections::{BinaryHeap, HashMap};
use std::fs;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Direction {
    Left(usize),
    Right(usize),
    Up(usize),
    Down(usize),
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
struct Point<const MIN: usize, const MAX: usize> {
    x: i32,
    y: i32,
    value: usize,
    dir: Direction,
}

impl<const MIN: usize, const MAX: usize> PartialOrd for Point<MIN, MAX> {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(other.value.cmp(&self.value))
    }
}

impl<const MIN: usize, const MAX: usize> Ord for Point<MIN, MAX> {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        other.value.cmp(&self.value)
    }
}

impl<const MIN: usize, const MAX: usize> Point<MIN, MAX> {
    fn from(x: i32, y: i32, value: usize, dir: Direction) -> Self {
        Self { x, y, value, dir }
    }

    fn get_surroundings(&self) -> Vec<Point<MIN, MAX>> {
        let mut r = Vec::new();
        match self.dir {
            Direction::Down(v) => {
                r.push(Point::from(
                    self.x - MIN as i32,
                    self.y,
                    0,
                    Direction::Left(MIN),
                ));
                r.push(Point::from(
                    self.x + MIN as i32,
                    self.y,
                    0,
                    Direction::Right(MIN),
                ));
                if v < MAX {
                    r.push(Point::from(self.x, self.y + 1, 0, Direction::Down(v + 1)))
                }
            }
            Direction::Up(v) => {
                r.push(Point::from(
                    self.x - MIN as i32,
                    self.y,
                    0,
                    Direction::Left(MIN),
                ));
                r.push(Point::from(
                    self.x + MIN as i32,
                    self.y,
                    0,
                    Direction::Right(MIN),
                ));
                if v < MAX {
                    r.push(Point::from(self.x, self.y - 1, 0, Direction::Up(v + 1)))
                }
            }
            Direction::Left(v) => {
                r.push(Point::from(
                    self.x,
                    self.y - MIN as i32,
                    0,
                    Direction::Up(MIN),
                ));
                r.push(Point::from(
                    self.x,
                    self.y + MIN as i32,
                    0,
                    Direction::Down(MIN),
                ));
                if v < MAX {
                    r.push(Point::from(self.x - 1, self.y, 0, Direction::Left(v + 1)))
                }
            }
            Direction::Right(v) => {
                r.push(Point::from(
                    self.x,
                    self.y - MIN as i32,
                    0,
                    Direction::Up(MIN),
                ));
                r.push(Point::from(
                    self.x,
                    self.y + MIN as i32,
                    0,
                    Direction::Down(MIN),
                ));
                if v < MAX {
                    r.push(Point::from(self.x + 1, self.y, 0, Direction::Right(v + 1)))
                }
            }
        }
        r
    }
}
struct Grid<const MIN: usize, const MAX: usize> {
    g: Vec<Vec<u8>>,
}

impl<const MIN: usize, const MAX: usize> From<&str> for Grid<MIN, MAX> {
    fn from(s: &str) -> Self {
        Self {
            g: s.lines()
                .map(|l| {
                    l.chars()
                        .map(|c| c.to_digit(10).unwrap() as u8)
                        .collect_vec()
                })
                .collect_vec(),
        }
    }
}

impl<const MIN: usize, const MAX: usize> Grid<MIN, MAX> {
    fn in_grid(&self, p: &Point<MIN, MAX>) -> bool {
        p.x >= 0 && p.x < self.g[0].len() as i32 && p.y >= 0 && p.y < self.g.len() as i32
    }

    fn get_vector_value(&self, start: &Point<MIN, MAX>, end: &Point<MIN, MAX>) -> usize {
        let xdiff = (start.x - end.x).abs();
        let ydiff = (start.y - end.y).abs();
        let xsign = (end.x - start.x).signum();
        let ysign = (end.y - start.y).signum();
        if xdiff != 0 {
            (1..=xdiff)
                .map(|x| self.g[end.y as usize][(start.x + x * xsign) as usize] as usize)
                .sum()
        } else if ydiff != 0 {
            (1..=ydiff)
                .map(|y| self.g[(start.y + y * ysign) as usize][end.x as usize] as usize)
                .sum()
        } else {
            panic!(":(")
        }
    }

    fn dij(&self) -> usize {
        let hmmax = || -> HashMap<Direction, usize> {
            HashMap::from_iter(
                (1..=MAX)
                    .map(|x| (Direction::Down(x), usize::MAX))
                    .chain((1..=MAX).map(|x| (Direction::Up(x), usize::MAX)))
                    .chain((1..=MAX).map(|x| (Direction::Left(x), usize::MAX)))
                    .chain((1..=MAX).map(|x| (Direction::Right(x), usize::MAX))),
            )
        };
        let hmfalse = || -> HashMap<Direction, bool> {
            HashMap::from_iter(
                (1..=MAX)
                    .map(|x| (Direction::Down(x), false))
                    .chain((1..=MAX).map(|x| (Direction::Up(x), false)))
                    .chain((1..=MAX).map(|x| (Direction::Left(x), false)))
                    .chain((1..=MAX).map(|x| (Direction::Right(x), false))),
            )
        };
        let mut risks = vec![vec![hmmax(); self.g[0].len()]; self.g.len()];
        let mut visited = vec![vec![hmfalse(); self.g[0].len()]; self.g.len()];
        let mut to_visit: BinaryHeap<Point<MIN, MAX>> = BinaryHeap::new();
        to_visit.push(Point::from(0, MIN as i32, 0, Direction::Down(MIN)));
        to_visit.push(Point::from(MIN as i32, 0, 0, Direction::Right(MIN)));
        *risks[MIN][0].get_mut(&Direction::Down(MIN)).unwrap() = self.get_vector_value(
            &Point::from(0, 0, 0, Direction::Down(1)),
            &Point::from(0, MIN as i32, 0, Direction::Down(1)),
        );
        *risks[0][MIN].get_mut(&Direction::Right(MIN)).unwrap() = self.get_vector_value(
            &Point::from(0, 0, 0, Direction::Right(1)),
            &Point::from(MIN as i32, 0, 0, Direction::Right(1)),
        );
        let end = (self.g[0].len() - 1, self.g.len() - 1);
        while !to_visit.is_empty() {
            let current = to_visit.pop().unwrap();
            if visited[current.y as usize][current.x as usize][&current.dir] {
                continue;
            }
            *visited[current.y as usize][current.x as usize]
                .get_mut(&current.dir)
                .unwrap() = true;
            let cv = risks[current.y as usize][current.x as usize][&current.dir] as usize;
            current.get_surroundings().iter().for_each(|sp| {
                if self.in_grid(sp) && !visited[sp.y as usize][sp.x as usize][&sp.dir] {
                    let spv = self.get_vector_value(&current, sp);
                    if cv + spv < risks[sp.y as usize][sp.x as usize][&sp.dir] {
                        *risks[sp.y as usize][sp.x as usize]
                            .get_mut(&sp.dir)
                            .unwrap() = cv + spv;
                    }
                    to_visit.push(Point::from(
                        sp.x,
                        sp.y,
                        risks[sp.y as usize][sp.x as usize][&sp.dir],
                        sp.dir,
                    ));
                }
            });
        }
        *risks[end.1][end.0].values().min().unwrap()
    }
}

fn solution<const MIN: usize, const MAX: usize>(input: &str) -> usize {
    Grid::<MIN, MAX>::from(input).dij()
}

#[test]
fn test_run() {
    let input = fs::read_to_string("src/inputs/aoc17s.in").unwrap();
    assert_eq!(solution::<1, 3>(&input), 102);
    let input = fs::read_to_string("src/inputs/aoc17.in").unwrap();
    assert_eq!(solution::<1, 3>(&input), 843);
    let input = fs::read_to_string("src/inputs/aoc17s.in").unwrap();
    assert_eq!(solution::<4, 10>(&input), 94);
    let input = fs::read_to_string("src/inputs/aoc17s2.in").unwrap();
    assert_eq!(solution::<4, 10>(&input), 71);
    let input = fs::read_to_string("src/inputs/aoc17.in").unwrap();
    assert_eq!(solution::<4, 10>(&input), 1017);
}

pub fn run() -> (String, String) {
    let input = fs::read_to_string("src/inputs/aoc17.in").unwrap();
    (
        solution::<1, 3>(&input).to_string(),
        solution::<4, 10>(&input).to_string(),
    )
}

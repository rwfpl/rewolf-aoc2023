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
    fn from_with_value(x: i32, y: i32, value: usize, dir: Direction) -> Self {
        Self { x, y, value, dir }
    }

    fn from(x: i32, y: i32, dir: Direction) -> Self {
        Self::from_with_value(x, y, 0, dir)
    }

    fn tuple(self) -> (i32, i32) {
        (self.x, self.y)
    }

    fn get_surroundings(&self) -> Vec<Point<MIN, MAX>> {
        let mut r = Vec::with_capacity(3);
        match self.dir {
            Direction::Down(v) => {
                r.push(Point::from(
                    self.x - MIN as i32,
                    self.y,
                    Direction::Left(MIN),
                ));
                r.push(Point::from(
                    self.x + MIN as i32,
                    self.y,
                    Direction::Right(MIN),
                ));
                if v < MAX {
                    r.push(Point::from(self.x, self.y + 1, Direction::Down(v + 1)))
                }
            }
            Direction::Up(v) => {
                r.push(Point::from(
                    self.x - MIN as i32,
                    self.y,
                    Direction::Left(MIN),
                ));
                r.push(Point::from(
                    self.x + MIN as i32,
                    self.y,
                    Direction::Right(MIN),
                ));
                if v < MAX {
                    r.push(Point::from(self.x, self.y - 1, Direction::Up(v + 1)))
                }
            }
            Direction::Left(v) => {
                r.push(Point::from(self.x, self.y - MIN as i32, Direction::Up(MIN)));
                r.push(Point::from(
                    self.x,
                    self.y + MIN as i32,
                    Direction::Down(MIN),
                ));
                if v < MAX {
                    r.push(Point::from(self.x - 1, self.y, Direction::Left(v + 1)))
                }
            }
            Direction::Right(v) => {
                r.push(Point::from(self.x, self.y - MIN as i32, Direction::Up(MIN)));
                r.push(Point::from(
                    self.x,
                    self.y + MIN as i32,
                    Direction::Down(MIN),
                ));
                if v < MAX {
                    r.push(Point::from(self.x + 1, self.y, Direction::Right(v + 1)))
                }
            }
        }
        r
    }
}
struct Grid<const MIN: usize, const MAX: usize> {
    g: Vec<Vec<u8>>,
}

#[derive(Copy, Clone)]
struct GridCacheItem {
    value: usize,
    visited: bool,
}

impl GridCacheItem {
    fn from(value: usize, visited: bool) -> Self {
        Self { value, visited }
    }
}

struct GridCache<const MIN: usize, const MAX: usize> {
    gc: Vec<Vec<HashMap<Direction, GridCacheItem>>>,
}

impl<const MIN: usize, const MAX: usize> GridCache<MIN, MAX> {
    fn new(width: usize, height: usize) -> Self {
        let cachedef = || -> HashMap<Direction, GridCacheItem> {
            let gci = GridCacheItem::from(usize::MAX, false);
            HashMap::from_iter(
                (MIN..=MAX)
                    .map(|x| (Direction::Down(x), gci))
                    .chain((MIN..=MAX).map(|x| (Direction::Up(x), gci)))
                    .chain((MIN..=MAX).map(|x| (Direction::Left(x), gci)))
                    .chain((MIN..=MAX).map(|x| (Direction::Right(x), gci))),
            )
        };
        Self {
            gc: vec![vec![cachedef(); width]; height],
        }
    }

    fn update(&mut self, x: usize, y: usize, dir: &Direction, gci: GridCacheItem) {
        *self.gc[y][x].get_mut(dir).unwrap() = gci;
    }

    fn get_mut(&mut self, x: i32, y: i32, dir: &Direction) -> &mut GridCacheItem {
        self.gc[y as usize][x as usize].get_mut(dir).unwrap()
    }
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

    fn get_vector_value(&self, start: (i32, i32), end: (i32, i32)) -> usize {
        let xdiff = (start.0 - end.0).abs();
        let ydiff = (start.1 - end.1).abs();
        if xdiff != 0 {
            let xsign = (end.0 - start.0).signum();
            (1..=xdiff)
                .map(|x| self.g[end.1 as usize][(start.0 + x * xsign) as usize] as usize)
                .sum()
        } else if ydiff != 0 {
            let ysign = (end.1 - start.1).signum();
            (1..=ydiff)
                .map(|y| self.g[(start.1 + y * ysign) as usize][end.0 as usize] as usize)
                .sum()
        } else {
            panic!(":(")
        }
    }

    fn dij(&self) -> usize {
        let mut grid_cache = GridCache::<MIN, MAX>::new(self.g[0].len(), self.g.len());
        let mut to_visit: BinaryHeap<Point<MIN, MAX>> = BinaryHeap::new();
        to_visit.push(Point::from(0, MIN as i32, Direction::Down(MIN)));
        to_visit.push(Point::from(MIN as i32, 0, Direction::Right(MIN)));
        grid_cache.update(
            0,
            MIN,
            &Direction::Down(MIN),
            GridCacheItem::from(self.get_vector_value((0, 0), (0, MIN as i32)), false),
        );
        grid_cache.update(
            MIN,
            0,
            &Direction::Right(MIN),
            GridCacheItem::from(self.get_vector_value((0, 0), (MIN as i32, 0)), false),
        );
        let end = (self.g[0].len() - 1, self.g.len() - 1);
        while !to_visit.is_empty() {
            let current = to_visit.pop().unwrap();
            let from_cache = grid_cache.get_mut(current.x, current.y, &current.dir);
            if from_cache.visited {
                continue;
            }
            from_cache.visited = true;
            let cv = from_cache.value;
            current.get_surroundings().iter().for_each(|sp| {
                if self.in_grid(sp) {
                    let from_cache = grid_cache.get_mut(sp.x, sp.y, &sp.dir);
                    if !from_cache.visited {
                        from_cache.value = std::cmp::min(
                            cv + self.get_vector_value(current.tuple(), sp.tuple()),
                            from_cache.value,
                        );
                        to_visit.push(Point::from_with_value(sp.x, sp.y, from_cache.value, sp.dir));
                    }
                }
            });
        }
        grid_cache.gc[end.1][end.0]
            .values()
            .map(|t| t.value)
            .min()
            .unwrap()
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

use core::fmt;
use itertools::Itertools;
use std::{collections::HashSet, fs};

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

impl fmt::Display for Tile {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::Vertical(_) => '│',
                Self::Horizontal(_) => '─',
                Self::UpRight(_) => '└',
                Self::UpLeft(_) => '┘',
                Self::DownLeft(_) => '┐',
                Self::DownRight(_) => '┌',
                Self::Start(_) => 'S',
                Self::Ground => '.',
            }
        )
    }
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

impl fmt::Display for Grid {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            self.tiles.iter().map(|row| row.iter().join("")).join("\n")
        )
    }
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

        let mut r = Self { tiles, start };
        r.update_starting_shape();
        r
    }
}

impl Grid {
    fn in_grid(&self, p: &Point) -> bool {
        p.0 >= 0 && p.0 < self.tiles[0].len() as i32 && p.1 >= 0 && p.1 < self.tiles.len() as i32
    }

    fn at(&self, p: &Point) -> &Tile {
        &self.tiles[p.1 as usize][p.0 as usize]
    }

    fn update_starting_shape(&mut self) {
        let ps = [Point(-1, 0), Point(0, -1), Point(1, 0), Point(0, 1)]
            .iter()
            .map(|p| (Point(self.start.0 + p.0, self.start.1 + p.1), p))
            .filter_map(|(np, p)| {
                if self.in_grid(&np) && self.at(&np).connects_to(&self.start) {
                    Some(p)
                } else {
                    None
                }
            })
            .collect_vec();
        let conto = [
            Point(self.start.0 + ps[0].0, self.start.1 + ps[0].1),
            Point(self.start.0 + ps[1].0, self.start.1 + ps[1].1),
        ];
        self.tiles[self.start.1 as usize][self.start.0 as usize] = match ps[..] {
            [Point(-1, 0), Point(1, 0)] => Tile::Horizontal(conto),
            [Point(0, -1), Point(0, 1)] => Tile::Vertical(conto),
            [Point(-1, 0), Point(0, -1)] => Tile::UpLeft(conto),
            [Point(0, -1), Point(1, 0)] => Tile::UpRight(conto),
            [Point(-1, 0), Point(0, 1)] => Tile::DownLeft(conto),
            [Point(1, 0), Point(0, 1)] => Tile::DownRight(conto),
            _ => panic!("(❀◦‿◦)"),
        };
    }

    fn get_next(&self, p: &Point, visited: &HashSet<Point>) -> Option<Vec<Point>> {
        let np = self
            .at(p)
            .get_next()
            .unwrap()
            .iter()
            .filter(|p| !visited.contains(p))
            .cloned()
            .collect_vec();
        if np.is_empty() {
            None
        } else {
            Some(np)
        }
    }

    fn get_loop(&self) -> HashSet<Point> {
        let mut cur = vec![self.start];
        let mut visited: HashSet<Point> = HashSet::from_iter(cur.iter().cloned());
        loop {
            cur = cur
                .iter()
                .map_while(|p| self.get_next(p, &visited))
                .flatten()
                .collect_vec();
            if cur.len() < 2 {
                break;
            }
            visited.extend(cur.iter());
        }
        visited
    }

    fn clean_garbage(&mut self) {
        let lp = self.get_loop();
        self.tiles.iter_mut().enumerate().for_each(|(y, row)| {
            row.iter_mut().enumerate().for_each(|(x, tile)| {
                if !lp.contains(&Point(x as i32, y as i32)) {
                    *tile = Tile::Ground
                }
            })
        });
    }

    fn get_farthest_distance(&self) -> usize {
        self.get_loop().len() / 2
    }

    fn is_inside(&self, p: &Point) -> bool {
        if let Tile::Ground = self.at(p) {
            let mut last_corner = Tile::Ground;
            return ((p.0 + 1) as usize..self.tiles[0].len())
                .map(|x| {
                    match self.at(&Point(x as i32, p.1)) {
                        Tile::Vertical(_) => 1,
                        Tile::UpRight(c) => {
                            last_corner = Tile::UpRight(*c);
                            1
                        }
                        Tile::DownRight(c) => {
                            last_corner = Tile::DownRight(*c);
                            1
                        }
                        Tile::UpLeft(_) => match last_corner {
                            Tile::UpRight(_) => 1,
                            Tile::DownRight(_) => 0,
                            _ => panic!("ᶘ ᵒᴥᵒᶅ"),
                        },
                        Tile::DownLeft(_) => match last_corner {
                            Tile::UpRight(_) => 0,
                            Tile::DownRight(_) => 1,
                            _ => panic!("ᶘ ᵒᴥᵒᶅ"),
                        },
                        _ => 0,
                    }
                    //
                })
                .sum::<u32>()
                & 1
                != 0;
        }
        false
    }
}

fn solution(input: &str) -> usize {
    Grid::from(input).get_farthest_distance()
}

fn solution2(input: &str) -> usize {
    let mut g = Grid::from(input);
    g.clean_garbage();
    g.tiles
        .iter()
        .enumerate()
        .map(|(y, row)| {
            row.iter()
                .enumerate()
                .filter(|(x, _)| g.is_inside(&Point(*x as i32, y as i32)))
                .count()
        })
        .sum()
}

#[test]
fn test_run() {
    let input = fs::read_to_string("src/inputs/aoc10s.in").unwrap();
    assert_eq!(solution(&input), 4);
    let input = fs::read_to_string("src/inputs/aoc10s2.in").unwrap();
    assert_eq!(solution(&input), 8);
    let input = fs::read_to_string("src/inputs/aoc10.in").unwrap();
    assert_eq!(solution(&input), 7093);
    assert_eq!(solution2(&input), 0);

    let input = fs::read_to_string("src/inputs/aoc10s3.in").unwrap();
    assert_eq!(solution2(&input), 8);
    let input = fs::read_to_string("src/inputs/aoc10s4.in").unwrap();
    assert_eq!(solution2(&input), 10);
}

pub fn run() -> (String, String) {
    let input = fs::read_to_string("src/inputs/aoc10.in").unwrap();
    (solution(&input).to_string(), solution2(&input).to_string())
}

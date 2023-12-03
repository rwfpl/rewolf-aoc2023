use std::{
    collections::{HashMap, HashSet},
    fs,
};

use itertools::Itertools;

#[derive(Debug, Eq, PartialEq, Hash, Clone)]
struct Pos {
    x: usize,
    y: usize,
}

impl Pos {
    fn from(x: usize, y: usize) -> Self {
        Self { x, y }
    }
}

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
struct Num {
    value: u32,
    pos: Pos,
    len: usize,
}

#[derive(Debug)]
struct Grid {
    g: Vec<Vec<char>>,
    numbers: Vec<Num>,
    adjacent_symbols: HashMap<Num, Vec<Pos>>,
}

fn extract_symbols_and_numbers(g: &[Vec<char>]) -> (Vec<Num>, HashSet<Pos>) {
    let mut numbers = Vec::new();
    let mut symbols = HashSet::new();
    g.iter().enumerate().for_each(|(y, row)| {
        let mut num = 0;
        let mut num_len = 0;
        for (x, c) in row.iter().enumerate() {
            match c {
                '0'..='9' => {
                    num *= 10;
                    num += c.to_digit(10).unwrap();
                    num_len += 1;
                }
                c => {
                    if num_len > 0 {
                        numbers.push(Num {
                            value: num,
                            pos: Pos::from(x - num_len, y),
                            len: num_len,
                        });
                        num = 0;
                        num_len = 0;
                    }
                    match c {
                        '.' => {}
                        _ => {
                            symbols.insert(Pos::from(x, y));
                        }
                    }
                }
            }
        }
        if num_len > 0 {
            numbers.push(Num {
                value: num,
                pos: Pos::from(row.len() - num_len, y),
                len: num_len,
            });
        }
    });
    (numbers, symbols)
}

fn calculate_adjacent_symbols(n: &Num, g: &Vec<Vec<char>>, symbols: &HashSet<Pos>) -> Vec<Pos> {
    let mut r = Vec::new();
    let mut pos_x_min = n.pos.x;
    let mut len_max = n.len;
    if n.pos.x > 0 {
        let p = Pos::from(n.pos.x - 1, n.pos.y);
        if symbols.contains(&p) {
            r.push(p);
        }
        pos_x_min = n.pos.x - 1;
        len_max += 1;
    }
    if n.pos.x + n.len < g[0].len() {
        let p = Pos::from(n.pos.x + n.len, n.pos.y);
        if symbols.contains(&p) {
            r.push(p);
        }
        len_max += 1;
    }
    if n.pos.y > 0 {
        for i in 0..len_max {
            let p = Pos::from(pos_x_min + i, n.pos.y - 1);
            if symbols.contains(&p) {
                r.push(p);
            }
        }
    }
    if n.pos.y < g.len() - 1 {
        for i in 0..len_max {
            let p = Pos::from(pos_x_min + i, n.pos.y + 1);
            if symbols.contains(&p) {
                r.push(p);
            }
        }
    }
    r
}

impl From<&str> for Grid {
    fn from(s: &str) -> Self {
        let g = s.lines().map(|l| l.chars().collect_vec()).collect_vec();
        let (numbers, symbols) = extract_symbols_and_numbers(&g);
        let adjacent_symbols: HashMap<_, _> = numbers
            .iter()
            .map(|n| (n.clone(), calculate_adjacent_symbols(n, &g, &symbols)))
            .collect();
        Self {
            g,
            numbers,
            adjacent_symbols,
        }
    }
}

impl Grid {
    fn is_number_adjacent_to_the_symbol(&self, n: &Num) -> bool {
        !self.get_adjacent_symbols(n).is_empty()
    }

    fn get_adjacent_symbols(&self, n: &Num) -> &Vec<Pos> {
        &self.adjacent_symbols[n]
    }
}

fn solution(input: &str) -> (u32, u32) {
    let g = Grid::from(input);
    let mut syms: HashMap<Pos, Vec<&Num>> = HashMap::new();
    g.numbers.iter().for_each(|n| {
        g.get_adjacent_symbols(n)
            .iter()
            .filter(|s| g.g[s.y][s.x] == '*')
            .for_each(|s| {
                syms.entry(s.clone()).or_default().push(n);
            });
    });
    (
        g.numbers
            .iter()
            .filter(|n| g.is_number_adjacent_to_the_symbol(n))
            .map(|n| n.value)
            .sum(),
        syms.iter()
            .filter(|(_, a)| a.len() == 2)
            .map(|(_, a)| a.iter().map(|n| n.value).product::<u32>())
            .sum(),
    )
}

#[test]
fn test_run() {
    let input = fs::read_to_string("src/inputs/aoc3s.in").unwrap();
    assert_eq!(solution(&input), (4361, 467835));
    let input = fs::read_to_string("src/inputs/aoc3.in").unwrap();
    assert_eq!(solution(&input), (538046, 81709807));
}

pub fn run() -> (String, String) {
    let input = fs::read_to_string("src/inputs/aoc3.in").unwrap();
    let (p1, p2) = solution(&input);
    (p1.to_string(), p2.to_string())
}

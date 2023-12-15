use std::fs;

use itertools::Itertools;

fn hash(s: &str) -> u8 {
    s.bytes().fold(0, |acc, b| ((acc + b as u32) * 17) % 256) as u8
}

fn solution(input: &str) -> u32 {
    input.split(',').map(|s| hash(s) as u32).sum()
}

#[derive(Debug)]
enum Operation {
    Dash,
    Focal(u8),
}

#[derive(Debug)]
struct Step<'a> {
    bx: u8,
    label: &'a str,
    op: Operation,
}

impl<'a> From<&'a str> for Step<'a> {
    fn from(s: &'a str) -> Self {
        if s.ends_with('-') {
            let label = &s[0..s.len() - 1];
            Self {
                bx: hash(label),
                label,
                op: Operation::Dash,
            }
        } else {
            let (label, focal) = s.split_once('=').unwrap();
            Self {
                bx: hash(label),
                label,
                op: Operation::Focal(focal.parse().unwrap()),
            }
        }
    }
}

#[derive(Debug)]
struct Lens<'a> {
    label: &'a str,
    focal: u8,
}

#[derive(Debug, Default)]
struct Bx<'a> {
    lenses: Vec<Lens<'a>>,
}

impl<'a> Bx<'a> {
    fn find_lens_index(&self, label: &str) -> Option<usize> {
        self.lenses
            .iter()
            .enumerate()
            .find_map(|(i, l)| if l.label == label { Some(i) } else { None })
    }

    fn remove_lens(&mut self, label: &str) {
        if let Some(i) = self.find_lens_index(label) {
            self.lenses.remove(i);
        }
    }

    fn add_or_replace_lens(&mut self, label: &'a str, focal: u8) {
        if let Some(i) = self.find_lens_index(label) {
            self.lenses[i] = Lens { label, focal }
        } else {
            self.lenses.push(Lens { label, focal })
        }
    }
}

fn solution2(input: &str) -> usize {
    let mut boxes = (0..256).map(|_| Bx::default()).collect_vec();
    input
        .split(',')
        .map(Step::from)
        .for_each(|step| match step.op {
            Operation::Dash => boxes[step.bx as usize].remove_lens(step.label),
            Operation::Focal(focal) => {
                boxes[step.bx as usize].add_or_replace_lens(step.label, focal)
            }
        });
    boxes
        .iter()
        .enumerate()
        .map(|(i, bx)| {
            bx.lenses
                .iter()
                .enumerate()
                .map(|(li, l)| (i + 1) * (li + 1) * l.focal as usize)
                .sum::<usize>()
        })
        .sum::<usize>()
}

#[test]
fn test_run() {
    let input = fs::read_to_string("src/inputs/aoc15s.in").unwrap();
    assert_eq!(solution(&input), 1320);
    assert_eq!(solution2(&input), 145);
    let input = fs::read_to_string("src/inputs/aoc15.in").unwrap();
    assert_eq!(solution(&input), 518107);
    assert_eq!(solution2(&input), 303404);
}

pub fn run() -> (String, String) {
    let input = fs::read_to_string("src/inputs/aoc15.in").unwrap();
    (solution(&input).to_string(), solution2(&input).to_string())
}

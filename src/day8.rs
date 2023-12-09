use itertools::Itertools;
use std::{collections::HashMap, fs};

fn p1(turns: &str, map: &HashMap<&str, (&str, &str)>) -> usize {
    if !map.contains_key("AAA") {
        return 0;
    }
    let mut cur_node = "AAA";
    let mut cur_turn = 0;
    while cur_node != "ZZZ" {
        let t = turns.chars().nth(cur_turn % turns.len()).unwrap();
        cur_node = match t {
            'L' => map[cur_node].0,
            'R' => map[cur_node].1,
            _ => panic!(r"¯\(◉◡◔)/¯"),
        };
        cur_turn += 1;
    }
    cur_turn
}

fn p2(turns: &str, map: &HashMap<&str, (&str, &str)>) -> usize {
    let mut cur_nodes = map
        .keys()
        .filter(|k| k.ends_with('A'))
        .cloned()
        .collect_vec();
    let mut cur_turn = 0;
    let mut zs = Vec::new();
    while zs.len() != cur_nodes.len() {
        if cur_nodes.iter().any(|n| n.ends_with('Z')) {
            zs.push(cur_turn);
        }
        let t = turns.chars().nth(cur_turn % turns.len()).unwrap();
        cur_nodes.iter_mut().for_each(|n| {
            *n = match t {
                'L' => map[n].0,
                'R' => map[n].1,
                _ => panic!(r"¯\(◉◡◔)/¯"),
            }
        });
        cur_turn += 1;
    }
    zs.iter().fold(zs[0], |acc, v| num::integer::lcm(acc, *v))
}

fn solution(input: &str) -> (usize, usize) {
    let turns = input.lines().next().unwrap();
    let map: HashMap<&str, (&str, &str)> = input
        .lines()
        .skip(2)
        .map(|l| (&l[0..3], (&l[7..10], &l[12..15])))
        .collect();
    (p1(turns, &map), p2(turns, &map))
}

#[test]
fn test_run() {
    let input = fs::read_to_string("src/inputs/aoc8s.in").unwrap();
    assert_eq!(solution(&input), (2, 2));
    let input = fs::read_to_string("src/inputs/aoc8s2.in").unwrap();
    assert_eq!(solution(&input), (6, 6));
    let input = fs::read_to_string("src/inputs/aoc8s3.in").unwrap();
    assert_eq!(solution(&input), (0, 6));
    let input = fs::read_to_string("src/inputs/aoc8.in").unwrap();
    assert_eq!(solution(&input), (13207, 12324145107121));
}

pub fn run() -> (String, String) {
    let input = fs::read_to_string("src/inputs/aoc8.in").unwrap();
    let (p1, p2) = solution(&input);
    (p1.to_string(), p2.to_string())
}

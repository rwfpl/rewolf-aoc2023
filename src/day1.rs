use std::fs;

static DIGITS_P1: [(&str, u8); 9] = [
    ("1", 1),
    ("2", 2),
    ("3", 3),
    ("4", 4),
    ("5", 5),
    ("6", 6),
    ("7", 7),
    ("8", 8),
    ("9", 9),
];

static DIGITS_P2: [(&str, u8); 18] = [
    ("1", 1),
    ("2", 2),
    ("3", 3),
    ("4", 4),
    ("5", 5),
    ("6", 6),
    ("7", 7),
    ("8", 8),
    ("9", 9),
    ("one", 1),
    ("two", 2),
    ("three", 3),
    ("four", 4),
    ("five", 5),
    ("six", 6),
    ("seven", 7),
    ("eight", 8),
    ("nine", 9),
];

fn find_first_digit(s: &str, digits: &[(&str, u8)]) -> u8 {
    digits
        .iter()
        .map(|(ds, d)| (*d, s.find(ds)))
        .filter(|(_, pos)| pos.is_some())
        .min_by_key(|(_, x)| *x)
        .unwrap()
        .0
}

fn find_last_digit(s: &str, digits: &[(&str, u8)]) -> u8 {
    digits
        .iter()
        .map(|(ds, d)| (*d, s.rfind(ds)))
        .filter(|(_, pos)| pos.is_some())
        .max_by_key(|(_, x)| *x)
        .unwrap()
        .0
}

fn solution(input: &str, digits: &[(&str, u8)]) -> u32 {
    input
        .lines()
        .map(|l| find_first_digit(l, digits) as u32 * 10 + find_last_digit(l, digits) as u32)
        .sum()
}

#[test]
fn test_run() {
    let input = fs::read_to_string("src/inputs/aoc1s.in").unwrap();
    assert_eq!(solution(&input, &DIGITS_P1), 142);
    let input = fs::read_to_string("src/inputs/aoc1s2.in").unwrap();
    assert_eq!(solution(&input, &DIGITS_P2), 281);
    let input = fs::read_to_string("src/inputs/aoc1.in").unwrap();
    assert_eq!(solution(&input, &DIGITS_P1), 53651);
    assert_eq!(solution(&input, &DIGITS_P2), 53894);
}

pub fn run() -> (String, String) {
    let input = fs::read_to_string("src/inputs/aoc1.in").unwrap();
    (
        solution(&input, &DIGITS_P1).to_string(),
        solution(&input, &DIGITS_P2).to_string(),
    )
}

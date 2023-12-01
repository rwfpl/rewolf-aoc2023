use std::fs;

fn solution(input: &str) -> i32 {
    input.lines().count() as i32
}

#[test]
fn test_run() {
    let input = fs::read_to_string("src/inputs/aocXs.in").unwrap();
    assert_eq!(solution(&input), 0);
}

pub fn run() -> (String, String) {
    let input = fs::read_to_string("src/inputs/aocX.in").unwrap();
    (solution(&input).to_string(), solution(&input).to_string())
}

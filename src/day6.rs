use rayon::iter::{IntoParallelIterator, IntoParallelRefIterator, ParallelIterator};

struct Race {
    time: u64,
    distance: u64,
}

impl Race {
    fn from(time: u64, distance: u64) -> Self {
        Self { time, distance }
    }

    fn hold(&self, seconds: u64) -> u64 {
        (self.time - seconds) * seconds
    }

    fn get_win_number(&self) -> usize {
        (1..self.time)
            .into_par_iter()
            .filter(|t| self.hold(*t) > self.distance)
            .count()
    }
}

fn solution(input: &[Race]) -> usize {
    input
        .par_iter()
        .map(|race| race.get_win_number())
        .product::<usize>()
}

#[test]
fn test_run() {
    assert_eq!(
        solution(&[Race::from(7, 9), Race::from(15, 40), Race::from(30, 200)]),
        288
    );
    assert_eq!(solution(&[Race::from(71530, 940200)]), 71503);
    assert_eq!(
        solution(&[
            Race::from(48, 296),
            Race::from(93, 1928),
            Race::from(85, 1236),
            Race::from(95, 1391),
        ]),
        2756160
    );
    assert_eq!(solution(&[Race::from(48938595, 296192812361391)]), 34788142);
}

pub fn run() -> (String, String) {
    (
        solution(&[
            Race::from(48, 296),
            Race::from(93, 1928),
            Race::from(85, 1236),
            Race::from(95, 1391),
        ])
        .to_string(),
        solution(&[Race::from(48938595, 296192812361391)]).to_string(),
    )
}

#![feature(iter_map_windows)]
#![feature(iter_next_chunk)]
#![feature(cmp_minmax)]

use rayon::prelude::*;
use std::{env, time::Instant};

mod day1;

fn main() {
    let days = [
        (1, day1::run as fn() -> (String, String)),
    ];
    let now = Instant::now();
    let day = env::args()
        .nth(1)
        .unwrap_or("0".to_string())
        .parse::<usize>()
        .unwrap_or(0);
    match day {
        1..=25 => {
            let (p1, p2) = days[day - 1].1();
            println!("day{day} p1: {p1}\nday{day} p2: {p2}");
        }
        _ => days.par_iter().for_each(|day| {
            let now = Instant::now();
            let (p1, p2) = day.1();
            println!(
                "day{day_n} p1: {p1}\nday{day_n} p2: {p2}\nday{day_n} execution time: {:?}",
                now.elapsed(),
                day_n = day.0
            );
        }),
    }
    println!("total execution time: {:?}", now.elapsed());
}

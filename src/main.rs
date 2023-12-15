#![feature(iter_map_windows)]
#![feature(iter_next_chunk)]
#![feature(cmp_minmax)]
#![feature(iter_advance_by)]
#![feature(btree_cursors)]

use rayon::prelude::*;
use std::{env, time::Instant};

mod day1;
mod day10;
mod day11;
mod day12;
mod day13;
mod day14;
mod day15;
mod day2;
mod day3;
mod day4;
mod day5;
mod day6;
mod day7;
mod day8;
mod day9;

fn main() {
    let days = [
        (1, day1::run as fn() -> (String, String)),
        (2, day2::run),
        (3, day3::run),
        (4, day4::run),
        (5, day5::run),
        (6, day6::run),
        (7, day7::run),
        (8, day8::run),
        (9, day9::run),
        (10, day10::run),
        (11, day11::run),
        (12, day12::run),
        (13, day13::run),
        (14, day14::run),
        (15, day15::run),
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

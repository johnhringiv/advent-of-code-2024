use std::time::Instant;

mod days;

use days::*;

fn main() {
    let days = [day01::solve, day02::solve, day03::solve, day04::solve, day05::solve, day06::solve];
    println!(
        "{0: <3} | {1: <10} | {2: <14} | {3: <10}",
        "Day", "Part 1", "Part 2", "μs"
    );
    for (idx, day) in days.iter().enumerate() {
        let now = Instant::now();
        let (p1, p2) = day();
        let micros = now.elapsed().as_micros();
        println!(
            "{0:<3} | {1:<10} | {2:<14} | {3:<10}",
            format! {"{:02}", idx+1},
            p1,
            p2,
            micros
        )
    }
}

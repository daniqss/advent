use std::time::{Duration, Instant};
mod prelude;
mod year2020;

use prelude::{Day, DayResult};
use year2020::{day2020_01_p1, day2020_01_p2};

const YEAR_2020: [Option<Day>; 25] = [
    Some((day2020_01_p1, day2020_01_p2)),
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
];

fn main() {
    for (i, day) in YEAR_2020.iter().enumerate() {
        if let Some(day) = day {
            match timer(*day) {
                ((time, Some(part1)), (_, Some(part2))) => {
                    println!("Day {:02} -> ", i + 1);
                    println!(
                        "Time elapsed -> {}μs\tPart 1 -> {}\tPart 2 -> {}\n",
                        time.as_micros(),
                        part1,
                        part2
                    );
                }
                _ => {}
            }
        }
    }
}

fn timer<F: FnOnce() -> DayResult>(f: (F, F)) -> ((Duration, DayResult), (Duration, DayResult)) {
    let mut result = (None, None);

    let start_0 = Instant::now();
    result.0 = f.0();
    let elapsed_0 = start_0.elapsed();

    let start_1 = Instant::now();
    result.1 = f.1();
    let elapsed_1 = start_1.elapsed();

    return ((elapsed_0, result.0), (elapsed_1, result.1));
}

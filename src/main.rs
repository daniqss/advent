use std::time::{Duration, Instant};
mod prelude;
mod year2020;

use prelude::{Day, DayResult};
use year2020::year_2020;

fn main() {
    for (i, day) in year_2020().iter().enumerate() {
        if let Some(day) = day.as_deref() {
            match timer(day) {
                ((time_1, Some(puzzle_1)), (time_2, Some(puzzle_2))) => {
                    println!("Day {:02} -> ", i + 1);
                    println!(
                        "Part 1 -> {}, {}µs\tPart 2 -> {}, {}µs\n",
                        puzzle_1,
                        time_1.as_micros(),
                        puzzle_2,
                        time_2.as_micros()
                    );
                }
                _ => {}
            }
        }
    }
}

fn timer(f: &dyn Day) -> ((Duration, DayResult), (Duration, DayResult)) {
    let mut result = (None, None);

    let start_0 = Instant::now();
    result.0 = f.puzzle_1();
    let elapsed_0 = start_0.elapsed();

    let start_1 = Instant::now();
    result.1 = f.puzzle_2();
    let elapsed_1 = start_1.elapsed();

    return ((elapsed_0, result.0), (elapsed_1, result.1));
}

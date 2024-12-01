use std::time::{Duration, Instant};
mod prelude;
mod year2020;
mod year2024;

use prelude::DayResult;
use year2020::YEAR_2020;
use year2024::YEAR_2024;

fn main() {
    let years = vec![YEAR_2020, YEAR_2024];

    years.iter().for_each(|year| {
        year.iter().enumerate().for_each(|(i, day)| {
            if let Some(day) = day {
                match timer(*day) {
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
                    ((time_1, Some(puzzle_1)), _) => {
                        println!("Day {:02} -> ", i + 1);
                        println!(
                            "Part 1 -> {}, {}µs\tPart 2 -> Not complete yet",
                            puzzle_1,
                            time_1.as_micros(),
                        );
                    }
                    _ => {}
                }
            }
        })
    });
}

fn timer(
    day: (fn() -> DayResult, fn() -> DayResult),
) -> ((Duration, DayResult), (Duration, DayResult)) {
    let mut result = (None, None);

    let start_0 = Instant::now();
    result.0 = day.0();
    let elapsed_0 = start_0.elapsed();

    let start_1 = Instant::now();
    result.1 = day.1();
    let elapsed_1 = start_1.elapsed();

    ((elapsed_0, result.0), (elapsed_1, result.1))
}

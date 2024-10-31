mod prelude;
mod year2020;

use prelude::DayResult;
use year2020::day2020_01;

const YEAR_2020: [fn() -> DayResult; 25] = [
    day2020_01,
    || (None, None),
    || (None, None),
    || (None, None),
    || (None, None),
    || (None, None),
    || (None, None),
    || (None, None),
    || (None, None),
    || (None, None),
    || (None, None),
    || (None, None),
    || (None, None),
    || (None, None),
    || (None, None),
    || (None, None),
    || (None, None),
    || (None, None),
    || (None, None),
    || (None, None),
    || (None, None),
    || (None, None),
    || (None, None),
    || (None, None),
    || (None, None),
];

fn main() {
    for day in YEAR_2020.iter() {
        match day() {
            (Some(part1), Some(part2)) => println!("Part 1: {}\nPart 2: {}", part1, part2),
            _ => {}
        }
    }
}

use crate::prelude::Day;
mod day2020_01;
mod day2020_02;
use day2020_01::Day2020_01;
use day2020_02::Day2020_02;

pub fn year_2020() -> [Option<Box<dyn Day>>; 25] {
    [
        Some(Box::new(Day2020_01::new())),
        Some(Box::new(Day2020_02::new())),
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
    ]
}

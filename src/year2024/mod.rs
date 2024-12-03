mod day01;
mod day02;
mod day03;
use crate::prelude::Day;

pub const YEAR_2024: [Option<Day>; 25] = [
    Some((day01::puzzle_1, day01::puzzle_2)),
    Some((day02::puzzle_1, day02::puzzle_2)),
    Some((
        || day03::puzzle_1(include_str!("../../inputs/year2024/day03.txt")),
        || day03::puzzle_2(include_str!("../../inputs/year2024/day03.txt")),
    )),
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

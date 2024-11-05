use crate::prelude::Day;
mod day01;
mod day02;
mod day03;

pub const YEAR_2020: [Option<Day>; 25] = [
    Some((day01::puzzle_1, day01::puzzle_2)),
    Some((day02::puzzle_1, day02::puzzle_2)),
    Some((day03::puzzle_1, day03::puzzle_2)),
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

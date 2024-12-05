mod day01;
mod day02;
mod day03;
mod day04;
mod day05;
use crate::prelude::Day;

pub const YEAR_2024: [Option<Day>; 25] = [
    Some((day01::puzzle_1, day01::puzzle_2)),
    Some((day02::puzzle_1, day02::puzzle_2)),
    Some((
        || day03::puzzle_1(include_str!("../../inputs/year2024/day03.txt")),
        || day03::puzzle_2(include_str!("../../inputs/year2024/day03.txt")),
    )),
    Some((
        || day04::puzzle_1(include_str!("../../inputs/year2024/day04.txt")),
        || day04::puzzle_2(include_str!("../../inputs/year2024/day04.txt")),
    )),
    Some((
        || day05::puzzle_1(include_str!("../../inputs/year2024/day05.txt")),
        || day05::puzzle_2(include_str!("../../inputs/year2024/day05.txt")),
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
];

use std::{collections::HashSet, fmt::Display};

use crate::prelude::*;

#[derive(PartialEq)]
enum MapPosition {
    Empty,
    Passed,
    Guard,
    Obstacle,
}

impl Display for MapPosition {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            MapPosition::Empty => write!(f, "."),
            MapPosition::Passed => write!(f, "X"),
            MapPosition::Guard => write!(f, "^"),
            MapPosition::Obstacle => write!(f, "#"),
        }
    }
}

struct Map {
    map: Vec<Vec<MapPosition>>,
    guard: (isize, isize),
    directions: (isize, isize),
    positions: HashSet<(isize, isize)>,
}

impl Display for Map {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Guard: {:?}, distint positions len {:?}\n",
            self.guard,
            self.positions.len()
        )?;
        for row in &self.map {
            for cell in row {
                write!(f, "{}", cell)?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

impl Map {
    fn new(raw: &str) -> Self {
        let mut guard: (isize, isize) = (0, 0);
        let directions: (isize, isize) = (-1, 0);
        let mut positions: HashSet<(isize, isize)> = HashSet::new();
        let map: Vec<Vec<MapPosition>> = raw
            .lines()
            .enumerate()
            .map(|(i, l)| {
                l.chars()
                    .enumerate()
                    .map(|(j, c)| match c {
                        '.' => MapPosition::Empty,
                        '#' => MapPosition::Obstacle,
                        '^' => {
                            guard = (i as isize, j as isize);
                            positions.insert((i as isize, j as isize));
                            MapPosition::Guard
                        }
                        _ => unreachable!("map only has . # and ^ characters"),
                    })
                    .collect()
            })
            .collect();

        Self {
            map,
            guard,
            directions,
            positions,
        }
    }

    fn next_pos(&self) -> (usize, usize) {
        (
            (self.guard.0 + self.directions.0) as usize,
            (self.guard.1 + self.directions.1) as usize,
        )
    }
}

pub fn puzzle_1(raw: &str) -> DayResult {
    let mut map = Map::new(raw);

    while map.next_pos() > (0, 0) && map.next_pos() < (map.map.len() - 1, map.map.len() - 1) {
        match map.map[map.next_pos().0][map.next_pos().1] {
            MapPosition::Empty | MapPosition::Passed => {
                map.map[map.guard.0 as usize][map.guard.1 as usize] = MapPosition::Passed;
                map.guard.0 += map.directions.0;
                map.guard.1 += map.directions.1;
                map.positions.insert((map.guard.0, map.guard.1));
                map.map[map.guard.0 as usize][map.guard.1 as usize] = MapPosition::Guard
            }
            MapPosition::Obstacle => match map.directions {
                (-1, 0) => map.directions = (0, 1),
                (0, 1) => map.directions = (1, 0),
                (1, 0) => map.directions = (0, -1),
                (0, -1) => map.directions = (-1, 0),
                _ => unreachable!("There's only four posible directions"),
            },
            _ => unreachable!("guard should never be on a guard position"),
        }
        println!("{}", map);
    }

    Some(map.positions.len().to_string())
}

pub fn puzzle_2(_raw: &str) -> DayResult {
    None
}

mod tests_2024_06 {

    #[test]
    pub fn test_inputs() {
        debug_assert_eq!(
            Some("41".to_string()),
            super::puzzle_1(
                r"....#.....
.........#
..........
..#.......
.......#..
..........
.#..^.....
........#.
#.........
......#..."
            )
        );
    }

    #[test]
    fn real_inputs() {
        if cfg!(feature = "real_input_debug") {
            assert_eq!(
                Some("4826".to_string()),
                super::puzzle_1(include_str!("../../inputs/year2024/day06.txt"))
            );
        }
    }
}

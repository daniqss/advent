use crate::prelude::DayResult;
use regex::Regex;

/// --- Day 3: Mull It Over ---
/// "Our computers are having issues, so I have no idea if we have any Chief Historians in stock! You're welcome to check the warehouse, though," says the mildly flustered shopkeeper at the North Pole Toboggan Rental Shop. The Historians head out to take a look.
///
/// The shopkeeper turns to you. "Any chance you can see why our computers are having issues again?"
///
/// The computer appears to be trying to run a program, but its memory (your puzzle input) is corrupted. All of the instructions have been jumbled up!
///
/// It seems like the goal of the program is just to multiply some numbers. It does that with instructions like mul(X,Y), where X and Y are each 1-3 digit numbers. For instance, mul(44,46) multiplies 44 by 46 to get a result of 2024. Similarly, mul(123,4) would multiply 123 by 4.
///
/// However, because the program's memory has been corrupted, there are also many invalid characters that should be ignored, even if they look like part of a mul instruction. Sequences like mul(4*, mul(6,9!, ?(12,34), or mul ( 2 , 4 ) do nothing.
///
/// For example, consider the following section of corrupted memory:
///
/// xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))
/// Only the four highlighted sections are real mul instructions. Adding up the result of each instruction produces 161 (2*4 + 5*5 + 11*8 + 8*5).
///
/// Scan the corrupted memory for uncorrupted mul instructions. What do you get if you add up all of the results of the multiplications?
pub fn puzzle_1(raw: &str) -> DayResult {
    match Regex::new(r"mul\((\d+),\s*(\d+)\)") {
        Ok(regex) => {
            let result = format!(
                "{}",
                regex
                    .captures_iter(raw)
                    .into_iter()
                    .map(|cap| {
                        &cap[1].parse::<usize>().unwrap() * &cap[2].parse::<usize>().unwrap()
                    })
                    .sum::<usize>()
            );
            Some(result)
        }
        Err(_) => None,
    }
}

/// --- Part Two ---
/// As you scan through the corrupted memory, you notice that some of the conditional statements are also still intact. If you handle some of the uncorrupted conditional statements in the program, you might be able to get an even more accurate result.
///
/// There are two new instructions you'll need to handle:
///
/// The do() instruction enables future mul instructions.
/// The don't() instruction disables future mul instructions.
/// Only the most recent do() or don't() instruction applies. At the beginning of the program, mul instructions are enabled.
///
/// For example:
///
/// xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))
/// This corrupted memory is similar to the example from before, but this time the mul(5,5) and mul(11,8) instructions are disabled because there is a don't() instruction before them. The other mul instructions function normally, including the one at the end that gets re-enabled by a do() instruction.
///
/// This time, the sum of the results is 48 (2*4 + 8*5).
///
/// Handle the new instructions; what do you get if you add up all of the results of just the enabled multiplications?
pub fn puzzle_2(raw: &str) -> DayResult {
    let program = raw.to_string();
    let mut enabled = true;
    let mut result = 0;
    let regex = match Regex::new(r"^mul\((\d+),(\d+)\).*") {
        Ok(regex) => regex,
        Err(_) => return None,
    };

    let mut i = 0;
    while i < program.len() {
        if i + 7 < program.len() && program[i..i + 7] == *"don't()" {
            enabled = false;
            i += 7;
        } else if i + 4 < program.len() && program[i..i + 4] == *"do()" {
            i += 4;
            enabled = true;
        } else if i + 4 < program.len() && program[i..i + 4] == *"mul(" {
            result += match regex.captures(&program[i..]) {
                Some(cap) => {
                    i += cap.len();
                    match enabled {
                        true => {
                            &cap[1].parse::<usize>().unwrap() * &cap[2].parse::<usize>().unwrap()
                        }
                        false => 0,
                    }
                }
                None => {
                    i += 4;
                    0
                }
            }
        } else {
            i += 1
        }
    }

    Some(format!("{}", result))
}

mod tests_2024_03 {
    #[test]
    pub fn test_inputs() {
        debug_assert_eq!(
            Some("161".to_string()),
            super::puzzle_1(
                "xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))"
            )
        );
        debug_assert_eq!(
            Some("48".to_string()),
            super::puzzle_2(
                "xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+do()+put()+don't()mul(32,64](mul(11,8)undo()?mul(8,5))"
            )
        );
    }

    #[test]
    pub fn real_inputs() {
        if cfg!(feature = "real_input_debug") {
            debug_assert_eq!(
                Some("187825547".to_string()),
                super::puzzle_1(include_str!("../../inputs/year2024/day03.txt"))
            );
            debug_assert_eq!(
                Some("85508223".to_string()),
                super::puzzle_2(include_str!("../../inputs/year2024/day03.txt"))
            );
        }
    }
}

use crate::prelude::DayResult;

/// --- Day 4: Ceres Search ---
/// "Looks like the Chief's not here. Next!" One of The Historians pulls out a device and pushes the only button on it. After a brief flash, you recognize the interior of the Ceres monitoring station!
///
/// As the search for the Chief continues, a small Elf who lives on the station tugs on your shirt; she'd like to know if you could help her with her word search (your puzzle input). She only has to find one word: XMAS.
///
/// This word search allows words to be horizontal, vertical, diagonal, written backwards, or even overlapping other words. It's a little unusual, though, as you don't merely need to find one instance of XMAS - you need to find all of them. Here are a few ways XMAS might appear, where irrelevant characters have been replaced with .:
///
///
/// ..X...
/// .SAMX.
/// .A..A.
/// XMAS.S
/// .X....
/// The actual word search will be full of letters instead. For example:
///
/// MMMSXXMASM
/// MSAMXMSMSA
/// AMXSXMAAMM
/// MSAMASMSMX
/// XMASAMXAMM
/// XXAMMXXAMA
/// SMSMSASXSS
/// SAXAMASAAA
/// MAMMMXMMMM
/// MXMXAXMASX
/// In this word search, XMAS occurs a total of 18 times; here's the same word search again, but where letters not involved in any XMAS have been replaced with .:
///
/// ....XXMAS.
/// .SAMXMS...
/// ...S..A...
/// ..A.A.MS.X
/// XMASAMX.MM
/// X.....XA.A
/// S.S.S.S.SS
/// .A.A.A.A.A
/// ..M.M.M.MM
/// .X.X.XMASX
/// Take a look at the little Elf's word search. How many times does XMAS appear?
pub fn puzzle_1(raw: &str) -> DayResult {
    let word_search: Vec<Vec<u8>> = raw.lines().map(|l| l.as_bytes().to_vec()).collect();
    let check_letters: [u8; 3] = [b'M', b'A', b'S'];
    let mut count = 0;
    #[cfg(test)]
    println!("{}", raw);

    for i in 0..word_search.len() {
        for j in 0..word_search[i].len() {
            if word_search[i][j] == b'X' {
                // left
                if j as isize - 3 >= 0 {
                    let mut positions: Vec<(usize, usize)> = Vec::new();
                    let mut left_count = 0;
                    for k in 1..check_letters.len() + 1 {
                        if word_search[i][j - k] == check_letters[k - 1] {
                            positions.push((i, j - k));
                            left_count += 1;
                        }
                    }
                    if left_count == 3 {
                        count += 1;
                        #[cfg(test)]
                        println!("found XMAS in ({}, {}) => {:?}", i, j, positions);
                    }
                }
                // left up
                if j as isize - 3 >= 0 && i as isize - 3 >= 0 {
                    let mut positions: Vec<(usize, usize)> = Vec::new();
                    let mut left_up_count = 0;
                    for k in 1..check_letters.len() + 1 {
                        if word_search[i - k][j - k] == check_letters[k - 1] {
                            positions.push((i - k, j - k));
                            left_up_count += 1;
                        }
                    }
                    if left_up_count == 3 {
                        count += 1;
                        #[cfg(test)]
                        println!("found XMAS in ({}, {}) => {:?}", i, j, positions);
                    }
                }

                // up
                if i as isize - 3 >= 0 {
                    let mut positions: Vec<(usize, usize)> = Vec::new();
                    let mut up_count = 0;
                    for k in 1..check_letters.len() + 1 {
                        if word_search[i - k][j] == check_letters[k - 1] {
                            positions.push((i - k, j));
                            up_count += 1;
                        }
                    }
                    if up_count == 3 {
                        count += 1;
                        #[cfg(test)]
                        println!("found XMAS in ({}, {}) => {:?}", i, j, positions);
                    }
                }

                // right up
                if j + 3 <= word_search[i].len() - 1 && i as isize - 3 >= 0 {
                    let mut positions: Vec<(usize, usize)> = Vec::new();
                    let mut right_up_count = 0;
                    for k in 1..check_letters.len() + 1 {
                        if word_search[i - k][j + k] == check_letters[k - 1] {
                            positions.push((i - k, j + k));
                            right_up_count += 1;
                        }
                    }
                    if right_up_count == 3 {
                        count += 1;
                        #[cfg(test)]
                        println!("found XMAS in ({}, {}) => {:?}", i, j, positions);
                    }
                }

                // right
                if j + 3 <= word_search[i].len() - 1 {
                    let mut positions: Vec<(usize, usize)> = Vec::new();
                    let mut right_count = 0;
                    for k in 1..check_letters.len() + 1 {
                        if word_search[i][j + k] == check_letters[k - 1] {
                            positions.push((i, j + k));
                            right_count += 1;
                        }
                    }
                    if right_count == 3 {
                        count += 1;
                        #[cfg(test)]
                        println!("found XMAS in ({}, {}) => {:?}", i, j, positions);
                    }
                }

                // right down
                if j + 3 <= word_search[i].len() - 1 && i + 3 < word_search.len() {
                    let mut positions: Vec<(usize, usize)> = Vec::new();
                    let mut right_down_count = 0;
                    for k in 1..check_letters.len() + 1 {
                        if word_search[i + k][j + k] == check_letters[k - 1] {
                            positions.push((i + k, j + k));
                            right_down_count += 1;
                        }
                    }
                    if right_down_count == 3 {
                        count += 1;
                        #[cfg(test)]
                        println!("found XMAS in ({}, {}) => {:?}", i, j, positions);
                    }
                }

                // down
                if i + 3 <= word_search.len() - 1 {
                    let mut positions: Vec<(usize, usize)> = Vec::new();
                    let mut down_count = 0;
                    for k in 1..check_letters.len() + 1 {
                        if word_search[i + k][j] == check_letters[k - 1] {
                            positions.push((i + k, j));
                            down_count += 1;
                        }
                    }
                    if down_count == 3 {
                        count += 1;
                        #[cfg(test)]
                        println!("found XMAS in ({}, {}) => {:?}", i, j, positions);
                    }
                }

                // left down
                if j as isize - 3 >= 0 && i + 3 < word_search.len() {
                    let mut positions: Vec<(usize, usize)> = Vec::new();
                    let mut left_down_count = 0;
                    for k in 1..check_letters.len() + 1 {
                        if word_search[i + k][j - k] == check_letters[k - 1] {
                            positions.push((i + k, j - k));
                            left_down_count += 1;
                        }
                    }
                    if left_down_count == 3 {
                        count += 1;
                        #[cfg(test)]
                        println!("found XMAS in ({}, {}) => {:?}", i, j, positions);
                    }
                }
            }
        }
    }
    Some(format!("{}", count))
}

/// --- Part Two ---
/// The Elf looks quizzically at you. Did you misunderstand the assignment?
///
/// Looking for the instructions, you flip over the word search to find that this isn't actually an XMAS puzzle; it's an X-MAS puzzle in which you're supposed to find two MAS in the shape of an X. One way to achieve that is like this:
///
/// M.S
/// .A.
/// M.S
/// Irrelevant characters have again been replaced with . in the above diagram. Within the X, each MAS can be written forwards or backwards.
///
/// Here's the same example from before, but this time all of the X-MASes have been kept instead:
///
/// .M.S......
/// ..A..MSMS.
/// .M.S.MAA..
/// ..A.ASMSM.
/// .M.S.M....
/// ..........
/// S.S.S.S.S.
/// .A.A.A.A..
/// M.M.M.M.M.
/// ..........
/// In this example, an X-MAS appears 9 times.
///
/// Flip the word search from the instructions back over to the word search side and try again. How many times does an X-MAS appear?
pub fn puzzle_2(_raw: &str) -> DayResult {
    None
}

mod tests {
    #[test]
    pub fn test_inputs() {
        debug_assert_eq!(
            Some("18".to_string()),
            super::puzzle_1(
                "MMMSXXMASM
MSAMXMSMSA
AMXSXMAAMM
MSAMASMSMX
XMASAMXAMM
XXAMMXXAMA
SMSMSASXSS
SAXAMASAAA
MAMMMXMMMM
MXMXAXMASX"
            )
        );
    }

    #[test]
    pub fn real_inputs() {
        debug_assert_eq!(
            Some("2718".to_string()),
            super::puzzle_1(include_str!("../../inputs/year2024/day04.txt"))
        );
    }
}

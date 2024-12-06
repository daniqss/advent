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

    let directions: [(isize, isize); 8] = [
        (0, -1),
        (0, 1),
        (-1, 0),
        (1, 0),
        (-1, -1),
        (-1, 1),
        (1, -1),
        (1, 1),
    ];

    for i in 0..word_search.len() {
        for j in 0..word_search[i].len() {
            if word_search[i][j] == b'X' {
                for &(dx, dy) in &directions {
                    #[cfg(test)]
                    let mut positions = Vec::new();
                    let mut matched = true;

                    for k in 1..=check_letters.len() {
                        let x = i as isize + k as isize * dx;
                        let y = j as isize + k as isize * dy;

                        if x < 0
                            || y < 0
                            || x as usize >= word_search.len()
                            || y as usize >= word_search[0].len()
                            || word_search[x as usize][y as usize] != check_letters[k - 1]
                        {
                            matched = false;
                            break;
                        }
                        #[cfg(test)]
                        positions.push((x as usize, y as usize));
                    }

                    if matched {
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
pub fn puzzle_2(raw: &str) -> DayResult {
    let word_search: Vec<Vec<u8>> = raw.lines().map(|l| l.as_bytes().to_vec()).collect();
    let mut count = 0;

    for i in 1..word_search.len() - 1 {
        for j in 1..word_search[i].len() - 1 {
            if word_search[i][j] == b'A' {
                // if main diagonal is (MAS || SAM) && secondary diagonal is (MAS || SAM)
                if ((word_search[i - 1][j - 1] == b'M' && word_search[i + 1][j + 1] == b'S')
                    || (word_search[i - 1][j - 1] == b'S' && word_search[i + 1][j + 1] == b'M'))
                    && ((word_search[i - 1][j + 1] == b'M' && word_search[i + 1][j - 1] == b'S')
                        || (word_search[i - 1][j + 1] == b'S' && word_search[i + 1][j - 1] == b'M'))
                {
                    count += 1;
                }
            }
        }
    }

    Some(format!("{}", count))
}

mod tests_2024_04 {
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

        debug_assert_eq!(
            Some("9".to_string()),
            super::puzzle_2(
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
        if cfg!(feature = "real_input_debug") {
            debug_assert_eq!(
                Some("2718".to_string()),
                super::puzzle_1(include_str!("../../inputs/year2024/day04.txt"))
            );
            debug_assert_eq!(
                Some("2046".to_string()),
                super::puzzle_2(include_str!("../../inputs/year2024/day04.txt"))
            );
        }
    }
}

use crate::DayResult;

const RAW_MAP: &str = include_str!("../../inputs/year2020/day03.txt");

/// # Day 3: Toboggan Trajectory
/// With the toboggan login problems resolved, you set off toward the airport. While travel by toboggan might be easy, it's certainly not safe: there's very minimal steering and the area is covered in trees. You'll need to see which angles will take you near the fewest trees.
///
/// Due to the local geology, trees in this area only grow on exact integer coordinates in a grid. You make a map (your puzzle input) of the open squares (.) and trees (#) you can see. For example:
/// ```
/// ..##.......
/// #...#...#..
/// .#....#..#.
/// ..#.#...#.#
/// .#...##..#.
/// ..#.##.....
/// .#.#.#....#
/// .#........#
/// #.##...#...
/// #...##....#
/// .#..#...#.#
/// ```
/// These aren't the only trees, though; due to something you read about once involving arboreal genetics and biome stability, the same pattern repeats to the right many times:
/// ```
/// ..##.........##.........##.........##.........##.........##.......  --->
/// #...#...#..#...#...#..#...#...#..#...#...#..#...#...#..#...#...#..
/// .#....#..#..#....#..#..#....#..#..#....#..#..#....#..#..#....#..#.
/// ..#.#...#.#..#.#...#.#..#.#...#.#..#.#...#.#..#.#...#.#..#.#...#.#
/// .#...##..#..#...##..#..#...##..#..#...##..#..#...##..#..#...##..#.
/// ..#.##.......#.##.......#.##.......#.##.......#.##.......#.##.....  --->
/// .#.#.#....#.#.#.#....#.#.#.#....#.#.#.#....#.#.#.#....#.#.#.#....#
/// .#........#.#........#.#........#.#........#.#........#.#........#
/// #.##...#...#.##...#...#.##...#...#.##...#...#.##...#...#.##...#...
/// #...##....##...##....##...##....##...##....##...##....##...##....#
/// .#..#...#.#.#..#...#.#.#..#...#.#.#..#...#.#.#..#...#.#.#..#...#.#  --->
/// ```
/// You start on the open square (.) in the top-left corner and need to reach the bottom (below the bottom-most row on your map).
///
/// The toboggan can only follow a few specific slopes (you opted for a cheaper model that prefers rational numbers); start by counting all the trees you would encounter for the slope right 3, down 1:
///
/// From your starting position at the top-left, check the position that is right 3 and down 1. Then, check the position that is right 3 and down 1 from there, and so on until you go past the bottom of the map.
///
/// The locations you'd check in the above example are marked here with O where there was an open square and X where there was a tree:
///
/// ```
/// ..##.........##.........##.........##.........##.........##.......  --->
/// #..O#...#..#...#...#..#...#...#..#...#...#..#...#...#..#...#...#..
/// .#....X..#..#....#..#..#....#..#..#....#..#..#....#..#..#....#..#.
/// ..#.#...#O#..#.#...#.#..#.#...#.#..#.#...#.#..#.#...#.#..#.#...#.#
/// .#...##..#..X...##..#..#...##..#..#...##..#..#...##..#..#...##..#.
/// ..#.##.......#.X#.......#.##.......#.##.......#.##.......#.##.....  --->
/// .#.#.#....#.#.#.#.O..#.#.#.#....#.#.#.#....#.#.#.#....#.#.#.#....#
/// .#........#.#........X.#........#.#........#.#........#.#........#
/// #.##...#...#.##...#...#.X#...#...#.##...#...#.##...#...#.##...#...
/// #...##....##...##....##...#X....##...##....##...##....##...##....#
/// .#..#...#.#.#..#...#.#.#..#...X.#.#..#...#.#.#..#...#.#.#..#...#.#  --->
/// ```
/// In this example, traversing the map using this slope would cause you to encounter 7 trees.
///
/// Starting at the top-left corner of your map and following a slope of right 3 and down 1, how many trees would you encounter?
pub fn puzzle_1() -> DayResult {
    let trees_array = RAW_MAP.as_bytes();
    let mut trees_found = 0;
    let mut x = 0;
    let mut y = 0;

    let mut line_width = 0;
    while trees_array[line_width] as char != '\n' {
        line_width += 1;
    }
    while x + 3 <= line_width {
        x += 3;
        y += 1;
        match trees_array[x * y * std::mem::size_of::<u8>()] as char {
            '#' => trees_found += 1,
            _ => (),
        };
    }

    Some(trees_found.to_string())
}

pub fn puzzle_2() -> DayResult {
    Some("weon".to_string())
}

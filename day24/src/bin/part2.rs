//! --- Part Two ---
//!
//! After careful analysis, one thing is certain: you have no idea where all these bugs are coming
//! from.
//!
//! Then, you remember: Eris is an old Plutonian settlement! Clearly, the bugs are coming from
//! recursively-folded space.
//!
//! This 5x5 grid is only one level in an infinite number of recursion levels. The tile in the
//! middle of the grid is actually another 5x5 grid, the grid in your scan is contained as the
//! middle tile of a larger 5x5 grid, and so on. Two levels of grids look like this:
//!
//!      |     |         |     |
//!      |     |         |     |
//!      |     |         |     |
//! -----+-----+---------+-----+-----
//!      |     |         |     |
//!      |     |         |     |
//!      |     |         |     |
//! -----+-----+---------+-----+-----
//!      |     | | | | | |     |
//!      |     |-+-+-+-+-|     |
//!      |     | | | | | |     |
//!      |     |-+-+-+-+-|     |
//!      |     | | |?| | |     |
//!      |     |-+-+-+-+-|     |
//!      |     | | | | | |     |
//!      |     |-+-+-+-+-|     |
//!      |     | | | | | |     |
//! -----+-----+---------+-----+-----
//!      |     |         |     |
//!      |     |         |     |
//!      |     |         |     |
//! -----+-----+---------+-----+-----
//!      |     |         |     |
//!      |     |         |     |
//!      |     |         |     |
//!
//! (To save space, some of the tiles are not drawn to scale.) Remember, this is only a small part
//! of the infinitely recursive grid; there is a 5x5 grid that contains this diagram, and a 5x5
//! grid that contains that one, and so on. Also, the ? in the diagram contains another 5x5 grid,
//! which itself contains another 5x5 grid, and so on.
//!
//! The scan you took (your puzzle input) shows where the bugs are on a single level of this
//! structure. The middle tile of your scan is empty to accommodate the recursive grids within it.
//! Initially, no other levels contain bugs.
//!
//! Tiles still count as adjacent if they are directly up, down, left, or right of a given tile.
//! Some tiles have adjacent tiles at a recursion level above or below its own level. For example:
//!
//!      |     |         |     |
//!   1  |  2  |    3    |  4  |  5
//!      |     |         |     |
//! -----+-----+---------+-----+-----
//!      |     |         |     |
//!   6  |  7  |    8    |  9  |  10
//!      |     |         |     |
//! -----+-----+---------+-----+-----
//!      |     |A|B|C|D|E|     |
//!      |     |-+-+-+-+-|     |
//!      |     |F|G|H|I|J|     |
//!      |     |-+-+-+-+-|     |
//!  11  | 12  |K|L|?|N|O|  14 |  15
//!      |     |-+-+-+-+-|     |
//!      |     |P|Q|R|S|T|     |
//!      |     |-+-+-+-+-|     |
//!      |     |U|V|W|X|Y|     |
//! -----+-----+---------+-----+-----
//!      |     |         |     |
//!  16  | 17  |    18   |  19 |  20
//!      |     |         |     |
//! -----+-----+---------+-----+-----
//!      |     |         |     |
//!  21  | 22  |    23   |  24 |  25
//!      |     |         |     |
//!
//!     Tile 19 has four adjacent tiles: 14, 18, 20, and 24.
//!     Tile G has four adjacent tiles: B, F, H, and L.
//!     Tile D has four adjacent tiles: 8, C, E, and I.
//!     Tile E has four adjacent tiles: 8, D, 14, and J.
//!     Tile 14 has eight adjacent tiles: 9, E, J, O, T, Y, 15, and 19.
//!     Tile N has eight adjacent tiles: I, O, S, and five tiles within the sub-grid marked ?.
//!
//! The rules about bugs living and dying are the same as before.
//!
//! For example, consider the same initial state as above:
//!
//! ....#
//! #..#.
//! #.?##
//! ..#..
//! #....
//!
//! The center tile is drawn as ? to indicate the next recursive grid. Call this level 0; the grid
//! within this one is level 1, and the grid that contains this one is level -1. Then, after ten
//! minutes, the grid at each level would look like this:
//!
//! Depth -5:
//! ..#..
//! .#.#.
//! ..?.#
//! .#.#.
//! ..#..
//!
//! Depth -4:
//! ...#.
//! ...##
//! ..?..
//! ...##
//! ...#.
//!
//! Depth -3:
//! #.#..
//! .#...
//! ..?..
//! .#...
//! #.#..
//!
//! Depth -2:
//! .#.##
//! ....#
//! ..?.#
//! ...##
//! .###.
//!
//! Depth -1:
//! #..##
//! ...##
//! ..?..
//! ...#.
//! .####
//!
//! Depth 0:
//! .#...
//! .#.##
//! .#?..
//! .....
//! .....
//!
//! Depth 1:
//! .##..
//! #..##
//! ..?.#
//! ##.##
//! #####
//!
//! Depth 2:
//! ###..
//! ##.#.
//! #.?..
//! .#.##
//! #.#..
//!
//! Depth 3:
//! ..###
//! .....
//! #.?..
//! #....
//! #...#
//!
//! Depth 4:
//! .###.
//! #..#.
//! #.?..
//! ##.#.
//! .....
//!
//! Depth 5:
//! ####.
//! #..#.
//! #.?#.
//! ####.
//! .....
//!
//! In this example, after 10 minutes, a total of 99 bugs are present.
//!
//! Starting with your scan, how many bugs are present after 200 minutes?

use std::error::Error;
use std::fs::File;
use std::io::{BufRead, BufReader};

// Each recursive board starts out full of empty spots, but with each passing minute, the state of
// the starting board will spread inward and outward one board. So, after MINUTES, there will be a
// total of 2 * MINUTES + 1 boards. We'll create arrays of 2 * MINUTES + 3 so we have an extra
// empty board in each direction for running calculations on the last minute.
//
// STARTING_BOARD is the index of the starting board in an array of 2 * MINUTES + 3 boards
const MINUTES: usize = 200;
const BOARDS: usize = 2 * MINUTES + 3;
const STARTING_BOARD: usize = MINUTES + 1;

/// Count the number of bugs in a board
fn count_bugs(board: &Vec<Vec<u8>>) -> u8 {
    board.iter().fold(0, |acc, line| {
        acc + line
            .iter()
            .fold(0, |acc2, &c| acc2 + if c == b'#' { 1 } else { 0 })
    })
}

fn main() -> Result<(), Box<dyn Error>> {
    let file = File::open("input.txt")?;
    let reader = BufReader::new(file);
    let board = reader
        .lines()
        .map(|line| line.map(|l| l.bytes().collect::<Vec<u8>>()))
        .collect::<Result<Vec<Vec<u8>>, _>>()?;
    let (width, height) = (board[0].len(), board.len());
    let mut boards = vec![vec![vec![b'.'; width]; height]; BOARDS];
    boards[STARTING_BOARD] = board;

    let mut convolutions = vec![vec![0; width]; height];
    let (middlex, middley) = (width / 2, height / 2);
    println!("Minutes: {: >3}", 0);
    for minutes in 1..=MINUTES {
        let minimum_level = STARTING_BOARD - minutes;
        let maximum_level = STARTING_BOARD + minutes;

        // these variables keep track of whether or not the previous level contained a bug up,
        // down, left, or right of the middle tile.
        let mut prev_lvl_bugs_d = false;
        let mut prev_lvl_bugs_l = false;
        let mut prev_lvl_bugs_r = false;
        let mut prev_lvl_bugs_u = false;
        for level in minimum_level..=maximum_level {
            // Prepare the convolution matrix, which we're using to count the number of adjascent
            // bugs to a given tile. In this loop, we'll zero the matrix, but also add 1 to the
            // tiles along the edges if the corresponding level above us had bugs adjascent to the
            // middle tile.
            for y in 0..height {
                for x in 0..width {
                    convolutions[y][x] = 0;
                    if x == 0 && prev_lvl_bugs_l {
                        convolutions[y][x] += 1;
                    }
                    if x + 1 == width && prev_lvl_bugs_r {
                        convolutions[y][x] += 1;
                    }
                    if y == 0 && prev_lvl_bugs_u {
                        convolutions[y][x] += 1;
                    }
                    if y + 1 == height && prev_lvl_bugs_d {
                        convolutions[y][x] += 1;
                    }
                }
            }

            // We must also adjust the counts around the middle tile based on the number of bugs in
            // the next level's outer tiles.
            let folder = |acc, &c| acc + if c == b'#' { 1 } else { 0 };
            convolutions[middley - 1][middlex] += boards[level + 1][0].iter().fold(0, folder);
            convolutions[middley][middlex + 1] += boards[level + 1]
                .iter()
                .map(|r| &r[width - 1])
                .fold(0, folder);
            convolutions[middley + 1][middlex] +=
                boards[level + 1][height - 1].iter().fold(0, folder);
            convolutions[middley][middlex - 1] +=
                boards[level + 1].iter().map(|l| &l[0]).fold(0, folder);

            // Finally, update these variables to pass on the current values to the next level
            prev_lvl_bugs_d = boards[level][middley + 1][middlex] == b'#';
            prev_lvl_bugs_l = boards[level][middley][middlex - 1] == b'#';
            prev_lvl_bugs_r = boards[level][middley][middlex + 1] == b'#';
            prev_lvl_bugs_u = boards[level][middley - 1][middlex] == b'#';

            // Now loop through the tiles and compute the new bugs
            for y in 0..height {
                let not_last_row = y + 1 < height;
                for x in 0..width {
                    let not_last_col = x + 1 < height;
                    let is_bug = boards[level][y][x] == b'#';
                    if not_last_col {
                        if boards[level][y][x + 1] == b'#' {
                            convolutions[y][x] += 1;
                        }
                        if is_bug {
                            convolutions[y][x + 1] += 1;
                        }
                    }
                    if not_last_row {
                        if boards[level][y + 1][x] == b'#' {
                            convolutions[y][x] += 1;
                        }
                        if is_bug {
                            convolutions[y + 1][x] += 1;
                        }
                    }

                    if x != middlex || y != middley {
                        if is_bug && convolutions[y][x] != 1 {
                            boards[level][y][x] = b'.';
                        } else if !is_bug && (convolutions[y][x] == 1 || convolutions[y][x] == 2) {
                            boards[level][y][x] = b'#';
                        }
                    }
                }
            }
        }

        println!("\x1b[FMinutes: {: >3}", minutes);
    }

    let result = boards
        .iter()
        .fold(0, |acc, board| acc + count_bugs(board) as u16);
    println!("Total bugs: {}", result);

    Ok(())
}

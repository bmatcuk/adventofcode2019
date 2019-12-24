//! --- Day 24: Planet of Discord ---
//!
//! You land on Eris, your last stop before reaching Santa. As soon as you do, your sensors start
//! picking up strange life forms moving around: Eris is infested with bugs! With an over 24-hour
//! roundtrip for messages between you and Earth, you'll have to deal with this problem on your
//! own.
//!
//! Eris isn't a very large place; a scan of the entire area fits into a 5x5 grid (your puzzle
//! input). The scan shows bugs (#) and empty spaces (.).
//!
//! Each minute, The bugs live and die based on the number of bugs in the four adjacent tiles:
//!
//!     A bug dies (becoming an empty space) unless there is exactly one bug adjacent to it.
//!     An empty space becomes infested with a bug if exactly one or two bugs are adjacent to it.
//!
//! Otherwise, a bug or empty space remains the same. (Tiles on the edges of the grid have fewer
//! than four adjacent tiles; the missing tiles count as empty space.) This process happens in
//! every location simultaneously; that is, within the same minute, the number of adjacent bugs is
//! counted for every tile first, and then the tiles are updated.
//!
//! Here are the first few minutes of an example scenario:
//!
//! Initial state:
//! ....#
//! #..#.
//! #..##
//! ..#..
//! #....
//!
//! After 1 minute:
//! #..#.
//! ####.
//! ###.#
//! ##.##
//! .##..
//!
//! After 2 minutes:
//! #####
//! ....#
//! ....#
//! ...#.
//! #.###
//!
//! After 3 minutes:
//! #....
//! ####.
//! ...##
//! #.##.
//! .##.#
//!
//! After 4 minutes:
//! ####.
//! ....#
//! ##..#
//! .....
//! ##...
//!
//! To understand the nature of the bugs, watch for the first time a layout of bugs and empty
//! spaces matches any previous layout. In the example above, the first layout to appear twice is:
//!
//! .....
//! .....
//! .....
//! #....
//! .#...
//!
//! To calculate the biodiversity rating for this layout, consider each tile left-to-right in the
//! top row, then left-to-right in the second row, and so on. Each of these tiles is worth
//! biodiversity points equal to increasing powers of two: 1, 2, 4, 8, 16, 32, and so on. Add up
//! the biodiversity points for tiles with bugs; in this example, the 16th tile (32768 points) and
//! 22nd tile (2097152 points) have bugs, a total biodiversity rating of 2129920.
//!
//! What is the biodiversity rating for the first layout that appears twice?

use std::collections::HashSet;
use std::error::Error;
use std::fs::File;
use std::hash::{Hash, Hasher};
use std::io::{BufRead, BufReader};
use std::iter::successors;

fn hash(board: &Vec<Vec<u8>>) -> u64 {
    let mut hasher = std::collections::hash_map::DefaultHasher::new();
    board.hash(&mut hasher);
    hasher.finish()
}

fn print_board(board: &Vec<Vec<u8>>) {
    for line in board.iter() {
        println!(
            "{}",
            line.iter().map(|c| char::from(*c)).collect::<String>()
        );
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let file = File::open("input.txt")?;
    let reader = BufReader::new(file);
    let mut seen_boards: HashSet<u64> = HashSet::new();
    let mut boards = vec![reader
        .lines()
        .map(|line| line.map(|l| l.bytes().collect::<Vec<u8>>()))
        .collect::<Result<Vec<Vec<u8>>, _>>()?];
    let (width, height) = (boards[0][0].len(), boards[0].len());
    let mut board_num = 0;
    boards.push(vec![vec![0; width]; height]);

    let mut minutes = 0;
    println!("Minutes: {: >4}", minutes);
    print_board(&boards[0]);
    loop {
        if !seen_boards.insert(hash(&boards[board_num])) {
            break;
        }

        // We need to make sure the first row is initialized to zeros... all the other positions
        // will be initialized as the loop runs
        let update_board = board_num ^ 1;
        for x in 0..width {
            boards[update_board][0][x] = 0;
        }

        for y in 0..height {
            let not_last_row = y + 1 < height;
            for x in 0..width {
                let not_last_col = x + 1 < height;
                let is_bug = boards[board_num][y][x] == b'#';
                let mut adjascent_bugs = boards[update_board][y][x];
                if not_last_col {
                    if boards[board_num][y][x + 1] == b'#' {
                        adjascent_bugs += 1;
                    }
                    if is_bug {
                        boards[update_board][y][x + 1] += 1;
                    }
                }
                if not_last_row {
                    if boards[board_num][y + 1][x] == b'#' {
                        adjascent_bugs += 1;
                    }
                    boards[update_board][y + 1][x] = if is_bug { 1 } else { 0 };
                }

                if is_bug && adjascent_bugs != 1 {
                    boards[update_board][y][x] = b'.';
                } else if !is_bug && (adjascent_bugs == 1 || adjascent_bugs == 2) {
                    boards[update_board][y][x] = b'#';
                } else {
                    boards[update_board][y][x] = boards[board_num][y][x];
                }
            }
        }

        minutes += 1;
        board_num = update_board;
        println!("\x1b[{}FMinutes: {: >4}", height + 1, minutes);
        print_board(&boards[board_num]);
    }

    let result = boards[board_num]
        .iter()
        .flat_map(|l| l.iter())
        .zip(successors(Some(1), |n| Some(n << 1)))
        .fold(0, |acc, (&n, p)| acc + if n == b'#' { p } else { 0 });
    println!("Biodiversity rating: {}", result);

    Ok(())
}

//! --- Day 10: Monitoring Station ---
//!
//! You fly into the asteroid belt and reach the Ceres monitoring station. The Elves here have an
//! emergency: they're having trouble tracking all of the asteroids and can't be sure they're safe.
//!
//! The Elves would like to build a new monitoring station in a nearby area of space; they hand you
//! a map of all of the asteroids in that region (your puzzle input).
//!
//! The map indicates whether each position is empty (.) or contains an asteroid (#). The asteroids
//! are much smaller than they appear on the map, and every asteroid is exactly in the center of
//! its marked position. The asteroids can be described with X,Y coordinates where X is the
//! distance from the left edge and Y is the distance from the top edge (so the top-left corner is
//! 0,0 and the position immediately to its right is 1,0).
//!
//! Your job is to figure out which asteroid would be the best place to build a new monitoring
//! station. A monitoring station can detect any asteroid to which it has direct line of sight -
//! that is, there cannot be another asteroid exactly between them. This line of sight can be at
//! any angle, not just lines aligned to the grid or diagonally. The best location is the asteroid
//! that can detect the largest number of other asteroids.
//!
//! For example, consider the following map:
//!
//! .#..#
//! .....
//! #####
//! ....#
//! ...##
//!
//! The best location for a new monitoring station on this map is the highlighted asteroid at 3,4
//! because it can detect 8 asteroids, more than any other location. (The only asteroid it cannot
//! detect is the one at 1,0; its view of this asteroid is blocked by the asteroid at 2,2.) All
//! other asteroids are worse locations; they can detect 7 or fewer other asteroids. Here is the
//! number of other asteroids a monitoring station on each asteroid could detect:
//!
//! .7..7
//! .....
//! 67775
//! ....7
//! ...87
//!
//! Here is an asteroid (#) and some examples of the ways its line of sight might be blocked. If
//! there were another asteroid at the location of a capital letter, the locations marked with the
//! corresponding lowercase letter would be blocked and could not be detected:
//!
//! #.........
//! ...A......
//! ...B..a...
//! .EDCG....a
//! ..F.c.b...
//! .....c....
//! ..efd.c.gb
//! .......c..
//! ....f...c.
//! ...e..d..c
//!
//! Here are some larger examples:
//!
//!     Best is 5,8 with 33 other asteroids detected:
//!
//!     ......#.#.
//!     #..#.#....
//!     ..#######.
//!     .#.#.###..
//!     .#..#.....
//!     ..#....#.#
//!     #..#....#.
//!     .##.#..###
//!     ##...#..#.
//!     .#....####
//!
//!     Best is 1,2 with 35 other asteroids detected:
//!
//!     #.#...#.#.
//!     .###....#.
//!     .#....#...
//!     ##.#.#.#.#
//!     ....#.#.#.
//!     .##..###.#
//!     ..#...##..
//!     ..##....##
//!     ......#...
//!     .####.###.
//!
//!     Best is 6,3 with 41 other asteroids detected:
//!
//!     .#..#..###
//!     ####.###.#
//!     ....###.#.
//!     ..###.##.#
//!     ##.##.#.#.
//!     ....###..#
//!     ..#.#..#.#
//!     #..#.#.###
//!     .##...##.#
//!     .....#.#..
//!
//!     Best is 11,13 with 210 other asteroids detected:
//!
//!     .#..##.###...#######
//!     ##.############..##.
//!     .#.######.########.#
//!     .###.#######.####.#.
//!     #####.##.#.##.###.##
//!     ..#####..#.#########
//!     ####################
//!     #.####....###.#.#.##
//!     ##.#################
//!     #####.##.###..####..
//!     ..######..##.#######
//!     ####.##.####...##..#
//!     .#####..#.######.###
//!     ##...#.##########...
//!     #.##########.#######
//!     .####.#.###.###.#.##
//!     ....##.##.###..#####
//!     .#.#.###########.###
//!     #.#.#.#####.####.###
//!     ###.##.####.##.#..##
//!
//! Find the best location for a new monitoring station. How many other asteroids can be detected
//! from that location?

use std::fs::File;
use std::io::{self, BufRead, BufReader};

// This only works because I know a >= b when I call gcd below
fn gcd(a: isize, b: isize) -> isize {
    if a == 0 {
        b
    } else if b == 0 {
        a
    } else {
        gcd(b, a % b)
    }
}

// ugh, there's got to be a better solution than this...
fn calculate(astroids: &Vec<Vec<bool>>, x: usize, y: usize, width: usize, height: usize) -> u16 {
    let mut count = 0;

    // horizontal
    if x > 0 {
        for x1 in (0..x).rev() {
            if astroids[y][x1] {
                count += 1;
                break;
            }
        }
    }
    for x1 in (x + 1)..width {
        if astroids[y][x1] {
            count += 1;
            break;
        }
    }

    // vertical
    if y > 0 {
        for y1 in (0..y).rev() {
            if astroids[y1][x] {
                count += 1;
                break;
            }
        }
    }
    for y1 in (y + 1)..height {
        if astroids[y1][x] {
            count += 1;
            break;
        }
    }

    // diagonals
    let valid_x = 0..(width as isize);
    let valid_y = 0..(height as isize);
    let max_dx = x.saturating_sub(1).max(width - x) as isize;
    let max_dy = y.saturating_sub(1).max(height - y) as isize;
    let max_delta = max_dx.max(max_dy) + 1;
    for dy in 1..=max_delta {
        for dx in dy..=max_delta {
            // check if this dx,dy is a multiple of a dx,dy we've already checked
            if gcd(dx, dy) > 1 {
                continue;
            }

            // the eight octants, except when dx == dy, then we only need the four diagonals
            let octants = if dx == dy {
                vec![(dx, dy), (-dx, dy), (-dx, -dy), (dx, -dy)]
            } else {
                vec![
                    (dx, dy),
                    (dy, dx),
                    (-dy, dx),
                    (-dx, dy),
                    (-dx, -dy),
                    (-dy, -dx),
                    (dy, -dx),
                    (dx, -dy),
                ]
            };
            for (dx, dy) in octants {
                let mut x = (x as isize) + dx;
                let mut y = (y as isize) + dy;
                while valid_x.contains(&x) && valid_y.contains(&y) {
                    if astroids[y as usize][x as usize] {
                        count += 1;
                        break;
                    }
                    x += dx;
                    y += dy;
                }
            }
        }
    }

    count
}

fn main() -> io::Result<()> {
    let file = File::open("input.txt")?;
    let reader = BufReader::new(file);
    let astroids = reader
        .lines()
        .map(|line| line.map(|l| l.bytes().map(|b| b == b'#').collect()))
        .collect::<io::Result<Vec<Vec<bool>>>>()?;
    let width = astroids[0].len();
    let height = astroids.len();
    let mut max_count = 0;
    let mut base_x = 0;
    let mut base_y = 0;
    for y in 0..height {
        for x in 0..width {
            let count = calculate(&astroids, x, y, width, height);
            if count > max_count {
                max_count = count;
                base_x = x;
                base_y = y;
            }
        }
    }
    println!("Max count {} for {},{}", max_count, base_x, base_y);

    Ok(())
}

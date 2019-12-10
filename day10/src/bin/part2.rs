//! --- Part Two ---
//!
//! Once you give them the coordinates, the Elves quickly deploy an Instant Monitoring Station to
//! the location and discover the worst: there are simply too many asteroids.
//!
//! The only solution is complete vaporization by giant laser.
//!
//! Fortunately, in addition to an asteroid scanner, the new monitoring station also comes equipped
//! with a giant rotating laser perfect for vaporizing asteroids. The laser starts by pointing up
//! and always rotates clockwise, vaporizing any asteroid it hits.
//!
//! If multiple asteroids are exactly in line with the station, the laser only has enough power to
//! vaporize one of them before continuing its rotation. In other words, the same asteroids that
//! can be detected can be vaporized, but if vaporizing one asteroid makes another one detectable,
//! the newly-detected asteroid won't be vaporized until the laser has returned to the same
//! position by rotating a full 360 degrees.
//!
//! For example, consider the following map, where the asteroid with the new monitoring station
//! (and laser) is marked X:
//!
//! .#....#####...#..
//! ##...##.#####..##
//! ##...#...#.#####.
//! ..#.....X...###..
//! ..#.#.....#....##
//!
//! The first nine asteroids to get vaporized, in order, would be:
//!
//! .#....###24...#..
//! ##...##.13#67..9#
//! ##...#...5.8####.
//! ..#.....X...###..
//! ..#.#.....#....##
//!
//! Note that some asteroids (the ones behind the asteroids marked 1, 5, and 7) won't have a chance
//! to be vaporized until the next full rotation. The laser continues rotating; the next nine to be
//! vaporized are:
//!
//! .#....###.....#..
//! ##...##...#.....#
//! ##...#......1234.
//! ..#.....X...5##..
//! ..#.9.....8....76
//!
//! The next nine to be vaporized are then:
//!
//! .8....###.....#..
//! 56...9#...#.....#
//! 34...7...........
//! ..2.....X....##..
//! ..1..............
//!
//! Finally, the laser completes its first full rotation (1 through 3), a second rotation (4
//! through 8), and vaporizes the last asteroid (9) partway through its third rotation:
//!
//! ......234.....6..
//! ......1...5.....7
//! .................
//! ........X....89..
//! .................
//!
//! In the large example above (the one with the best monitoring station location at 11,13):
//!
//!     The 1st asteroid to be vaporized is at 11,12.
//!     The 2nd asteroid to be vaporized is at 12,1.
//!     The 3rd asteroid to be vaporized is at 12,2.
//!     The 10th asteroid to be vaporized is at 12,8.
//!     The 20th asteroid to be vaporized is at 16,0.
//!     The 50th asteroid to be vaporized is at 16,9.
//!     The 100th asteroid to be vaporized is at 10,16.
//!     The 199th asteroid to be vaporized is at 9,6.
//!     The 200th asteroid to be vaporized is at 8,2.
//!     The 201st asteroid to be vaporized is at 10,9.
//!     The 299th and final asteroid to be vaporized is at 11,1.
//!
//! The Elves are placing bets on which will be the 200th asteroid to be vaporized. Win the bet by
//! determining which asteroid that will be; what do you get if you multiply its X coordinate by
//! 100 and then add its Y coordinate? (For example, 8,2 becomes 802.)

use std::cmp::{Eq, Ord, Ordering, PartialEq, PartialOrd};
use std::fs::File;
use std::io::{self, BufRead, BufReader};

const BASE_X: usize = 22;
const BASE_Y: usize = 25;

#[derive(Debug)]
struct Asteroid {
    x: isize,
    y: isize,
}

impl Asteroid {
    fn new(x: usize, y: usize, x1: usize, y1: usize) -> Asteroid {
        let deltax = (x1 as isize) - (x as isize);
        let deltay = (y1 as isize) - (y as isize);
        Asteroid { x: deltax, y: deltay }
    }

    fn radians(&self) -> f64 {
        // atan2(1,0) == 0, increases counter-clockwise, and decreases clockwise. We want f(0,-1)
        // == 0, increasing clockwise, and no negative numbers. Therefore, we swap the x and y
        // inputs to atan2, negate, then add 2pi if negative.
        let mut angle = (self.x as f64).atan2(-self.y as f64);
        if angle < 0.0 {
            angle += 2.0 * std::f64::consts::PI;
        }
        angle
    }
}

impl std::fmt::Display for Asteroid {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?} {}", self, self.radians())
    }
}

impl PartialEq for Asteroid {
    fn eq(&self, other: &Self) -> bool {
        self.x == other.x && self.y == other.y
    }
}

impl Eq for Asteroid {}

impl Ord for Asteroid {
    fn cmp(&self, other: &Self) -> Ordering {
        if self.eq(&other) {
            return Ordering::Equal;
        }
        self.radians().partial_cmp(&other.radians()).unwrap()
    }
}

impl PartialOrd for Asteroid {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(&other))
    }
}

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
fn get_visible_asteroids(asteroids: &mut Vec<Vec<bool>>, x: usize, y: usize, width: usize, height: usize) -> Vec<Asteroid> {
    let mut visible: Vec<Asteroid> = Vec::new();

    // horizontal
    if x > 0 {
        for x1 in (0..x).rev() {
            if asteroids[y][x1] {
                visible.push(Asteroid::new(x, y, x1, y));
                asteroids[y][x1] = false;
                break;
            }
        }
    }
    for x1 in (x + 1)..width {
        if asteroids[y][x1] {
            visible.push(Asteroid::new(x, y, x1, y));
            asteroids[y][x1] = false;
            break;
        }
    }

    // vertical
    if y > 0 {
        for y1 in (0..y).rev() {
            if asteroids[y1][x] {
                visible.push(Asteroid::new(x, y, x, y1));
                asteroids[y1][x] = false;
                break;
            }
        }
    }
    for y1 in (y + 1)..height {
        if asteroids[y1][x] {
            visible.push(Asteroid::new(x, y, x, y1));
            asteroids[y1][x] = false;
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
                let mut x1 = (x as isize) + dx;
                let mut y1 = (y as isize) + dy;
                while valid_x.contains(&x1) && valid_y.contains(&y1) {
                    if asteroids[y1 as usize][x1 as usize] {
                        visible.push(Asteroid::new(x, y, x1 as usize, y1 as usize));
                        asteroids[y1 as usize][x1 as usize] = false;
                        break;
                    }
                    x1 += dx;
                    y1 += dy;
                }
            }
        }
    }

    visible
}

fn main() -> io::Result<()> {
    let file = File::open("input.txt")?;
    let reader = BufReader::new(file);
    let mut asteroids = reader
        .lines()
        .map(|line| line.map(|l| l.bytes().map(|b| b == b'#').collect()))
        .collect::<io::Result<Vec<Vec<bool>>>>()?;
    let width = asteroids[0].len();
    let height = asteroids.len();
    let mut remaining = 199;
    loop {
        let mut visible = get_visible_asteroids(&mut asteroids, BASE_X, BASE_Y, width, height);
        // visible.sort();
        // for (i, v) in visible.iter().enumerate() {
        //     println!("{}: {}", i, v);
        // }
        if visible.len() < remaining {
            remaining -= visible.len();
        } else {
            visible.sort();

            let target = &visible[remaining];
            println!("200th asteroid: {}", (BASE_X as isize + target.x) * 100 + (BASE_Y as isize + target.y));
            break;
        }
    }

    Ok(())
}

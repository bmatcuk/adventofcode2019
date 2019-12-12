//! --- Part Two ---
//!
//! All this drifting around in space makes you wonder about the nature of the universe. Does
//! history really repeat itself? You're curious whether the moons will ever return to a previous
//! state.
//!
//! Determine the number of steps that must occur before all of the moons' positions and velocities
//! exactly match a previous point in time.
//!
//! For example, the first example above takes 2772 steps before they exactly match a previous
//! point in time; it eventually returns to the initial state:
//!
//! After 0 steps:
//! pos=<x= -1, y=  0, z=  2>, vel=<x=  0, y=  0, z=  0>
//! pos=<x=  2, y=-10, z= -7>, vel=<x=  0, y=  0, z=  0>
//! pos=<x=  4, y= -8, z=  8>, vel=<x=  0, y=  0, z=  0>
//! pos=<x=  3, y=  5, z= -1>, vel=<x=  0, y=  0, z=  0>
//!
//! After 2770 steps:
//! pos=<x=  2, y= -1, z=  1>, vel=<x= -3, y=  2, z=  2>
//! pos=<x=  3, y= -7, z= -4>, vel=<x=  2, y= -5, z= -6>
//! pos=<x=  1, y= -7, z=  5>, vel=<x=  0, y= -3, z=  6>
//! pos=<x=  2, y=  2, z=  0>, vel=<x=  1, y=  6, z= -2>
//!
//! After 2771 steps:
//! pos=<x= -1, y=  0, z=  2>, vel=<x= -3, y=  1, z=  1>
//! pos=<x=  2, y=-10, z= -7>, vel=<x= -1, y= -3, z= -3>
//! pos=<x=  4, y= -8, z=  8>, vel=<x=  3, y= -1, z=  3>
//! pos=<x=  3, y=  5, z= -1>, vel=<x=  1, y=  3, z= -1>
//!
//! After 2772 steps:
//! pos=<x= -1, y=  0, z=  2>, vel=<x=  0, y=  0, z=  0>
//! pos=<x=  2, y=-10, z= -7>, vel=<x=  0, y=  0, z=  0>
//! pos=<x=  4, y= -8, z=  8>, vel=<x=  0, y=  0, z=  0>
//! pos=<x=  3, y=  5, z= -1>, vel=<x=  0, y=  0, z=  0>
//!
//! Of course, the universe might last for a very long time before repeating. Here's a copy of the
//! second example from above:
//!
//! <x=-8, y=-10, z=0>
//! <x=5, y=5, z=10>
//! <x=2, y=-7, z=3>
//! <x=9, y=-8, z=-3>
//!
//! This set of initial positions takes 4686774924 steps before it repeats a previous state!
//! Clearly, you might need to find a more efficient way to simulate the universe.
//!
//! How many steps does it take to reach the first state that exactly matches a previous state?

use std::error::Error;
use std::fs::File;
use std::io::{BufRead, BufReader};

struct Point3 {
    pub x: isize,
    pub y: isize,
    pub z: isize,
}

struct Moon {
    position: Point3,
}

impl Moon {
    fn load(serialized: String) -> Result<Moon, Box<dyn Error>> {
        // serialized format:
        // "<x=X, y=Y, z=Z>"
        let coords = serialized[1..(serialized.len() - 1)]
            .split(", ")
            .map(|coord| coord[2..].parse())
            .collect::<Result<Vec<isize>, _>>()?;
        let position = Point3 {
            x: coords[0],
            y: coords[1],
            z: coords[2],
        };
        Ok(Moon { position })
    }
}

fn gcd(a: u64, b: u64) -> u64 {
    fn _gcd(a: u64, b: u64) -> u64 {
        if a == 0 {
            b
        } else if b == 0 {
            a
        } else {
            _gcd(b, a % b)
        }
    }

    if a == b {
        a
    } else if a >= b {
        _gcd(a, b)
    } else {
        _gcd(b, a)
    }
}

fn lcm(a: u64, b: u64) -> u64 {
    a / gcd(a, b) * b
}

fn main() -> Result<(), Box<dyn Error>> {
    let file = File::open("input.txt")?;
    let reader = BufReader::new(file);
    let moons = reader
        .lines()
        .map(|line| line.map_err(|e| e.into()).and_then(|s| Moon::load(s)))
        .collect::<Result<Vec<Moon>, _>>()?;
    let num_moons = moons.len();
    let initial = [
        moons
            .iter()
            .map(|moon| moon.position.x)
            .collect::<Vec<isize>>(),
        moons
            .iter()
            .map(|moon| moon.position.y)
            .collect::<Vec<isize>>(),
        moons
            .iter()
            .map(|moon| moon.position.z)
            .collect::<Vec<isize>>(),
    ];
    let mut iterations = [0u64; 3];
    for (i, initial) in initial.iter().enumerate() {
        let initial_velocities = vec![0isize; num_moons];
        let mut velocities = initial_velocities.clone();
        let mut positions = initial.clone();
        loop {
            for i in 0..(num_moons - 1) {
                for j in (i + 1)..num_moons {
                    let change = (positions[j] - positions[i]).signum();
                    velocities[i] += change;
                    velocities[j] -= change;
                }
                positions[i] += velocities[i];
            }
            positions[num_moons - 1] += velocities[num_moons - 1];

            iterations[i] += 1;
            // println!("{:?} {:?}", positions, initial);
            // println!("{:?} {:?}", velocities, initial_velocities);
            if positions.eq(initial) && velocities.eq(&initial_velocities) {
                break;
            }
        }
    }

    let result = iterations[1..]
        .iter()
        .fold(iterations[0], |acc, &x| lcm(acc, x));
    println!("X cycle after {} iterations", iterations[0]);
    println!("Y cycle after {} iterations", iterations[1]);
    println!("Z cycle after {} iterations", iterations[2]);
    println!("Cycle after {} iterations", result);

    Ok(())
}

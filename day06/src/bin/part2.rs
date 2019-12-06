//! --- Part Two ---
//!
//! Now, you just need to figure out how many orbital transfers you (YOU) need to take to get to
//! Santa (SAN).
//!
//! You start at the object YOU are orbiting; your destination is the object SAN is orbiting. An
//! orbital transfer lets you move from any object to an object orbiting or orbited by that object.
//!
//! For example, suppose you have the following map:
//!
//! COM)B
//! B)C
//! C)D
//! D)E
//! E)F
//! B)G
//! G)H
//! D)I
//! E)J
//! J)K
//! K)L
//! K)YOU
//! I)SAN
//!
//! Visually, the above map of orbits looks like this:
//!
//!                           YOU
//!                          /
//!         G - H       J - K - L
//!        /           /
//! COM - B - C - D - E - F
//!                \
//!                 I - SAN
//!
//! In this example, YOU are in orbit around K, and SAN is in orbit around I. To move from K to I,
//! a minimum of 4 orbital transfers are required:
//!
//!     K to J
//!     J to E
//!     E to D
//!     D to I
//!
//! Afterward, the map of orbits looks like this:
//!
//!         G - H       J - K - L
//!        /           /
//! COM - B - C - D - E - F
//!                \
//!                 I - SAN
//!                  \
//!                   YOU
//!
//! What is the minimum number of orbital transfers required to move from the object YOU are
//! orbiting to the object SAN is orbiting? (Between the objects they are orbiting - not between
//! YOU and SAN.)

use std::cell::RefCell;
use std::collections::HashMap;
use std::default::Default;
use std::error::Error;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::rc::Rc;

#[derive(Default)]
struct Planet {
    orbits: Option<Rc<RefCell<Planet>>>,
    count: usize,
}

impl Planet {
    fn orbits(&mut self, planet: Rc<RefCell<Planet>>) {
        self.orbits = Some(planet);
    }

    fn count(&mut self, c: usize) -> usize {
        if self.count > 0 {
            return self.count + c;
        }

        self.count = c;
        match self.orbits.as_ref() {
            Some(planet) => planet.borrow_mut().count(c + 1),
            None => c,
        }
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let file = File::open("input.txt")?;
    let reader = BufReader::new(file);

    let mut planets: HashMap<_, Rc<RefCell<Planet>>> = HashMap::new();
    for line in reader.lines() {
        // each line of input is "ABC)XYZ"
        let line = line?;
        let planet_name1 = line[..3].to_owned();
        let planet_name2 = line[4..].to_owned();
        let planet1 = Rc::clone(planets.entry(planet_name1).or_default());
        let mut planet2 = (*planets.entry(planet_name2).or_default()).borrow_mut();
        planet2.orbits(planet1);
    }

    planets.get("YOU").unwrap().borrow_mut().count(0);
    let result = planets.get("SAN").unwrap().borrow_mut().count(0);
    println!("Jumps: {}", result - 2);

    Ok(())
}

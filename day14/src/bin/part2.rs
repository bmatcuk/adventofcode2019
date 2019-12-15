//! --- Part Two ---
//!
//! After collecting ORE for a while, you check your cargo hold: 1 trillion (1000000000000) units
//! of ORE.
//!
//! With that much ore, given the examples above:
//!
//!     The 13312 ORE-per-FUEL example could produce 82892753 FUEL.
//!     The 180697 ORE-per-FUEL example could produce 5586022 FUEL.
//!     The 2210736 ORE-per-FUEL example could produce 460664 FUEL.
//!
//! Given 1 trillion ORE, what is the maximum amount of FUEL you can produce?

use std::collections::HashMap;
use std::error::Error;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::ops::Mul;

struct ChemicalQuantity {
    name: String,
    quantity: u64,
}

impl ChemicalQuantity {
    fn parse(s: &str) -> Result<ChemicalQuantity, Box<dyn Error>> {
        let space = s.find(' ').ok_or("invalid")?;
        let quantity = s[..space].parse()?;
        Ok(ChemicalQuantity::new(s[(space + 1)..].to_owned(), quantity))
    }

    fn new(name: String, quantity: u64) -> ChemicalQuantity {
        ChemicalQuantity { name, quantity }
    }
}

impl Mul<u64> for &ChemicalQuantity {
    type Output = ChemicalQuantity;

    fn mul(self, rhs: u64) -> ChemicalQuantity {
        ChemicalQuantity::new(self.name.clone(), self.quantity * rhs)
    }
}

struct ProductionRequirements {
    quantity: u64,
    requirements: Vec<ChemicalQuantity>,
}

impl ProductionRequirements {
    fn new(quantity: u64, requirements: Vec<ChemicalQuantity>) -> ProductionRequirements {
        ProductionRequirements {
            quantity,
            requirements,
        }
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    // there's only one way to create each chemical, which simplifies the algo
    let file = File::open("input.txt")?;
    let reader = BufReader::new(file);
    let mut map = HashMap::new();
    for line in reader.lines() {
        match line {
            Ok(line) => {
                let arrow = line.rfind(" => ").unwrap();
                let produced = ChemicalQuantity::parse(&line[(arrow + 4)..])?;
                let requirements = line[..arrow]
                    .split(", ")
                    .map(|s| ChemicalQuantity::parse(&s))
                    .collect::<Result<Vec<_>, _>>()?;
                map.insert(
                    produced.name.clone(),
                    ProductionRequirements::new(produced.quantity, requirements),
                );
            }
            Err(e) => return Err(Box::new(e)),
        }
    }

    // From part1, I know it takes 248794 ORE to produce one FUEL, but there are some leftover
    // chemicals from the reactions, which means producing more FUEL is "cheaper". I'm just going
    // to brute-force trying different fuel values until I exceed a trillion ORE... maybe some sort
    // of binary search to make it quicker... I'm going to guess it won't exceed twice as much, so
    // I'll use that as an upper bound.
    let mut min_fuel = 1_000_000_000_000 / 248_794;
    let mut max_fuel = min_fuel * 2;
    'outer: while min_fuel < max_fuel {
        let mut fuel = (min_fuel + max_fuel) / 2;
        if fuel == min_fuel {
            fuel += 1;
        }

        let mut ore = 1_000_000_000_000;
        let mut extra = HashMap::new();
        let mut needed = vec![ChemicalQuantity::new("FUEL".to_owned(), fuel)];
        while let Some(ChemicalQuantity {
            name,
            quantity: mut needed_quantity,
        }) = needed.pop()
        {
            // if we need ORE, increment our count
            if name == "ORE" {
                if needed_quantity > ore {
                    // this fuel requires too much ore
                    max_fuel = fuel - 1;
                    continue 'outer;
                }
                ore -= needed_quantity;
                continue;
            }

            // check if we have leftover that might fulfill our needs
            if let Some(leftover) = extra.get_mut(&name) {
                if needed_quantity >= *leftover {
                    needed_quantity -= *leftover;
                    *leftover = 0;
                } else {
                    *leftover -= needed_quantity;
                    continue;
                }
            }

            if let Some(ProductionRequirements {
                quantity: produced_quantity,
                requirements,
            }) = map.get(&name)
            {
                let multiplier = (needed_quantity + produced_quantity - 1) / produced_quantity;
                needed.extend(requirements.iter().map(|cq| cq * multiplier));

                let leftover = produced_quantity * multiplier - needed_quantity;
                if leftover > 0 {
                    extra.insert(name.clone(), leftover);
                }
            }
        }
        if ore == 0 {
            // seems unlikely, but maybe we found an answer that consumes exactly one trillion ORE
            min_fuel = fuel;
            max_fuel = fuel;
        } else {
            // this amount of FUEL took too little ORE
            min_fuel = fuel;
        }
    }
    println!("FUEL: {}", min_fuel);

    Ok(())
}

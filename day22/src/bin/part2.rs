//! --- Part Two ---
//!
//! After a while, you realize your shuffling skill won't improve much more with merely a single
//! deck of cards. You ask every 3D printer on the ship to make you some more cards while you check
//! on the ship repairs. While reviewing the work the droids have finished so far, you think you
//! see Halley's Comet fly past!
//!
//! When you get back, you discover that the 3D printers have combined their power to create for
//! you a single, giant, brand new, factory order deck of 119315717514047 space cards.
//!
//! Finally, a deck of cards worthy of shuffling!
//!
//! You decide to apply your complete shuffle process (your puzzle input) to the deck
//! 101741582076661 times in a row.
//!
//! You'll need to be careful, though - one wrong move with this many cards and you might overflow
//! your entire ship!
//!
//! After shuffling your new, giant, factory order deck that many times, what number is on the card
//! that ends up in position 2020?

use std::error::Error;
use std::fs::File;
use std::io::{BufRead, BufReader};

const DECK_SIZE: i128 = 119_315_717_514_047;
const ITERATIONS: i128 = 101_741_582_076_661;
const CARD: i128 = 2020;

// computes pow(base, exponent) % modulus
fn modular_pow(base: i128, exponent: i128, modulus: i128) -> i128 {
    if modulus == 1 {
        return 0;
    }

    let mut result = 1;
    let mut base = base % modulus;
    let mut exponent = exponent;
    while exponent > 0 {
        if (exponent & 1) == 1 {
            result = (result * base) % modulus;
        }
        exponent = exponent >> 1;
        base = (base * base) % modulus;
    }

    result
}

// modular inverse of n - assuming DECK_SIZE is prime
fn inv(n: i128) -> i128 {
    modular_pow(n, DECK_SIZE - 2, DECK_SIZE)
}

fn main() -> Result<(), Box<dyn Error>> {
    // We'll represent the position of a given number in the list as some offset + increment * n.
    // Offset represents the first number in the list, and increment is the difference between
    // numbers in the list. Each of the three shuffling techniques changes offset and increment,
    // and each complete run of the instructions will also affect them in some predictable way that
    // we can calculate without looping.
    let file = File::open("input.txt")?;
    let reader = BufReader::new(file);
    let mut offset: i128 = 0;
    let mut increment: i128 = 1;
    for line in reader.lines() {
        match line {
            Ok(line) => {
                if line == "deal into new stack" {
                    // Reverses the list which means our increment is negated. Our list's first
                    // number also changes, which we can calculate by adding the new increment to
                    // our offset
                    increment = (increment * -1).rem_euclid(DECK_SIZE);
                    offset = (offset + increment).rem_euclid(DECK_SIZE);
                } else if line.starts_with("cut ") {
                    // rotates the list which only affects the offset
                    let n: i128 = line[4..].parse()?;
                    offset = (offset + increment * n).rem_euclid(DECK_SIZE);
                } else if line.starts_with("deal with increment ") {
                    // This one is a bit more difficult to explain... The card at index 0 goes to
                    // index 0; from 1 to n; from 2 to 2n; 3 to 3n; etc. So the ith card goes to
                    // i*n. Offset isn't changing, but how do we calculate increment? If we knew
                    // what i ended up at position 1, we could calculate the increment since we'd
                    // know position 0 and 1 at that point. For that, we need to calculate the
                    // "modular inverse" of n, mod the size of the deck. Now, I'll admit this math
                    // is a bit over my head at the moment, but, combining Fermat's little theorem,
                    // Euler's theorem, and the fact that our deck size is prime, we can calculate
                    // the modular inverse as pow(n, DECK_SIZE - 2) % DECK_SIZE. We then multiply
                    // the increment by that. One last problem: that pow() will probably overflow.
                    // There is a method to calculate modular exponentiation using exponentiation
                    // by squaring that's implemented above.
                    let n: i128 = line[20..].parse()?;
                    increment = (increment * inv(n)).rem_euclid(DECK_SIZE);
                }
            }
            Err(err) => return Err(Box::new(err)),
        }
    }

    // Ok, now we have the offset and increment from a single run. How can we "run" this ITERATIONS
    // times without actually running it (because that takes forever)? Two key observations:
    // increment is always multiplied by some constant and offset is always incremented by some
    // constant multiple of increment at that point in the process. This means that increment can
    // be calculated for each iteration by simplying multplying by this constant. This also means
    // there exists some constant that we can multiply increment by and add to offset to compute
    // offset for each iteration as well.
    //
    // Ok, so what is this constant? Let's take a look at the first couple iterations:
    // iter=0, offset=0
    // iter=1, offset=0 + 1c
    // iter=2, offset=0 + 1c + c*increment
    // iter=3, offset=0 + 1c + c*increment + c*increment**2
    // iter=n, offset=0 + 1c + c*increment + ... + c*increment**(n-1)
    //
    // That's a geometric series, so the sum is: c((1 - increment**n) / (1 - increment))
    let single_increment = increment;
    increment = modular_pow(increment, ITERATIONS, DECK_SIZE);
    offset = ((offset * (1 - increment)).rem_euclid(DECK_SIZE)
        * inv((1 - single_increment).rem_euclid(DECK_SIZE)))
    .rem_euclid(DECK_SIZE);

    println!(
        "Card at position {}: {}",
        CARD,
        (offset + increment * CARD) % DECK_SIZE
    );

    Ok(())
}

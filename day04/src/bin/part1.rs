//! --- Day 4: Secure Container ---
//!
//! You arrive at the Venus fuel depot only to discover it's protected by a password. The Elves had
//! written the password on a sticky note, but someone threw it out.
//!
//! However, they do remember a few key facts about the password:
//!
//!   - It is a six-digit number.
//!   - The value is within the range given in your puzzle input.
//!   - Two adjacent digits are the same (like 22 in 122345).
//!   - Going from left to right, the digits never decrease; they only ever increase or stay the
//!     same (like 111123 or 135679).
//!
//! Other than the range rule, the following are true:
//!
//!   - 111111 meets these criteria (double 11, never decreases).
//!   - 223450 does not meet these criteria (decreasing pair of digits 50).
//!   - 123789 does not meet these criteria (no double).
//!
//! How many different passwords within the range given in your puzzle input meet these criteria?

const PASS_MIN: u32 = 357_253;
const PASS_MAX: u32 = 892_942;

struct Digits {
    num: u32,
    position: u32,
}

impl Digits {
    fn r#for(num: u32) -> Digits {
        Digits {
            num,
            position: 100_000,
        }
    }
}

impl Iterator for Digits {
    type Item = u32;

    fn next(&mut self) -> Option<Self::Item> {
        if self.position > 0 {
            let result = self.num / self.position;
            self.num %= self.position;
            self.position /= 10;
            Some(result)
        } else {
            None
        }
    }
}

struct Generator {
    current: u32,
    max: u32,
}

impl Generator {
    fn new(current: u32, max: u32) -> Generator {
        Generator { current, max }
    }
}

impl Iterator for Generator {
    type Item = u32;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            let (has_dup, next) = Digits::r#for(self.current + 1)
                .scan(0, |prev, x| {
                    if *prev < x {
                        *prev = x;
                        Some((false, x))
                    } else {
                        Some((true, *prev))
                    }
                })
                .fold((false, 0), |(dup, acc), (has_dup, x)| {
                    (dup || has_dup, acc * 10 + x)
                });
            self.current = next;

            if self.current > self.max {
                return None;
            } else if has_dup {
                return Some(self.current);
            }
        }
    }
}

fn main() {
    let result = Generator::new(PASS_MIN, PASS_MAX).count();
    println!("Possible Passwords: {}", result);
}

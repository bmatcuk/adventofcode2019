//! --- Part Two ---
//!
//! An Elf just remembered one more important detail: the two adjacent matching digits are not part
//! of a larger group of matching digits.
//!
//! Given this additional criterion, but still ignoring the range rule, the following are now true:
//!
//!   - 112233 meets these criteria because the digits never decrease and all repeated digits are
//!     exactly two digits long.
//!   - 123444 no longer meets the criteria (the repeated 44 is part of a larger group of 444).
//!   - 111122 meets the criteria (even though 1 is repeated more than twice, it still contains a
//!     double 22).
//!
//! How many different passwords within the range given in your puzzle input meet all of the
//! criteria?

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
                .scan((false, 0, 0), |state, x| {
                    let (dups, num_dups, prev) = *state;
                    if prev < x {
                        *state = (dups || num_dups == 2, 1, x);
                    } else {
                        *state = (dups, num_dups + 1, prev);
                    }
                    Some(*state)
                })
                .fold((false, 0), |(_, acc), (dups, num_dups, x)| {
                    (dups || num_dups == 2, acc * 10 + x)
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

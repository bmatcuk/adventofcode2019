//! --- Part Two ---
//!
//! Now that your FFT is working, you can decode the real signal.
//!
//! The real signal is your puzzle input repeated 10000 times. Treat this new signal as a single
//! input list. Patterns are still calculated as before, and 100 phases of FFT are still applied.
//!
//! The first seven digits of your initial input signal also represent the message offset. The
//! message offset is the location of the eight-digit message in the final output list.
//! Specifically, the message offset indicates the number of digits to skip before reading the
//! eight-digit message. For example, if the first seven digits of your initial input signal were
//! 1234567, the eight-digit message would be the eight digits after skipping 1,234,567 digits of
//! the final output list. Or, if the message offset were 7 and your final output list were
//! 98765432109876543210, the eight-digit message would be 21098765. (Of course, your real message
//! offset will be a seven-digit number, not a one-digit number like 7.)
//!
//! Here is the eight-digit message in the final output list after 100 phases. The message offset
//! given in each input has been highlighted. (Note that the inputs given below are repeated 10000
//! times to find the actual starting input lists.)
//!
//!     03036732577212944063491565474664 becomes 84462026.
//!     02935109699940807407585447034323 becomes 78725270.
//!     03081770884921959731165446850517 becomes 53553731.
//!
//! After repeating your input signal 10000 times and running 100 phases of FFT, what is the
//! eight-digit message embedded in the final output list?

use std::fs::File;
use std::io::{self, Read};

const REPEAT_INPUT: usize = 10_000;

fn main() -> io::Result<()> {
    // We don't even need to consider the first `offset` elements because we're only interested in
    // the result after the offset. Since the base pattern starts with a 0, those numbers will
    // never be considered in the computation.
    let file = File::open("input.txt")?;
    let mut input = file
        .bytes()
        .filter_map(|n| match n {
            Ok(n @ b'0'..=b'9') => Some(Ok((n - b'0') as i32)),
            Ok(_) => None,
            Err(e) => Some(Err(e)),
        })
        .collect::<Result<Vec<_>, _>>()?;
    let offset = input[..7].iter().fold(0, |acc, n| acc * 10 + n) as usize;
    let input_length = input.len() * REPEAT_INPUT - offset;
    input = input
        .iter()
        .cycle()
        .skip(offset)
        .take(input_length)
        .copied()
        .collect();
    for _ in 0..100 {
        // The intuition here is that if we are computing the value for position i, all values
        // before i (0..i) are ignored since the base pattern ([0, 1, 0, -1]), repeated i times,
        // would multiply all those values by 0. That means the very last number never changes; all
        // preceeding values are ignored, and there are no numbers after it. As we work our way
        // backward through the list, the next number is only the one's digit of adding it to the
        // last number. The 2nd from last is the one's digit of the sum of the last three numbers,
        // but since we've already updated the next-to-last with the sum of the last two, we don't
        // need to add them again. And so on.
        //
        // Now, there are 650 digits in the input. Once we multiply by 10,000, we have 6,650,000.
        // The first seven digits are 5,977,377. Since we don't care about those first ~5.97M
        // numbers, `i` will never get below 5.97M, which means that even when `i` is at its
        // lowest, the only part of the base pattern that matters is the first two numbers: [0, 1].
        // In other words, we always throw out everything before our current position, and add
        // everything after it.
        //
        // So now we've got an efficient algorithm: just iterate through the list backward,
        // updating each number as the one's digit of the sum of itself and the number after it.
        for i in (1..input_length).rev() {
            input[i - 1] = (input[i - 1] + input[i]) % 10;
        }
    }

    println!(
        "Result: {}",
        input
            .iter()
            .take(8)
            .map(|&n| char::from((n as u8) + b'0'))
            .collect::<String>()
    );

    Ok(())
}

//! --- Day 16: Flawed Frequency Transmission ---
//!
//! You're 3/4ths of the way through the gas giants. Not only do roundtrip signals to Earth take
//! five hours, but the signal quality is quite bad as well. You can clean up the signal with the
//! Flawed Frequency Transmission algorithm, or FFT.
//!
//! As input, FFT takes a list of numbers. In the signal you received (your puzzle input), each
//! number is a single digit: data like 15243 represents the sequence 1, 5, 2, 4, 3.
//!
//! FFT operates in repeated phases. In each phase, a new list is constructed with the same length
//! as the input list. This new list is also used as the input for the next phase.
//!
//! Each element in the new list is built by multiplying every value in the input list by a value
//! in a repeating pattern and then adding up the results. So, if the input list were 9, 8, 7, 6, 5
//! and the pattern for a given element were 1, 2, 3, the result would be 9*1 + 8*2 + 7*3 + 6*1 +
//! 5*2 (with each input element on the left and each value in the repeating pattern on the right
//! of each multiplication). Then, only the ones digit is kept: 38 becomes 8, -17 becomes 7, and so
//! on.
//!
//! While each element in the output array uses all of the same input array elements, the actual
//! repeating pattern to use depends on which output element is being calculated. The base pattern
//! is 0, 1, 0, -1. Then, repeat each value in the pattern a number of times equal to the position
//! in the output list being considered. Repeat once for the first element, twice for the second
//! element, three times for the third element, and so on. So, if the third element of the output
//! list is being calculated, repeating the values would produce: 0, 0, 0, 1, 1, 1, 0, 0, 0, -1,
//! -1, -1.
//!
//! When applying the pattern, skip the very first value exactly once. (In other words, offset the
//! whole pattern left by one.) So, for the second element of the output list, the actual pattern
//! used would be: 0, 1, 1, 0, 0, -1, -1, 0, 0, 1, 1, 0, 0, -1, -1, ....
//!
//! After using this process to calculate each element of the output list, the phase is complete,
//! and the output list of this phase is used as the new input list for the next phase, if any.
//!
//! Given the input signal 12345678, below are four phases of FFT. Within each phase, each output
//! digit is calculated on a single line with the result at the far right; each multiplication
//! operation shows the input digit on the left and the pattern value on the right:
//!
//! Input signal: 12345678
//!
//! 1*1  + 2*0  + 3*-1 + 4*0  + 5*1  + 6*0  + 7*-1 + 8*0  = 4
//! 1*0  + 2*1  + 3*1  + 4*0  + 5*0  + 6*-1 + 7*-1 + 8*0  = 8
//! 1*0  + 2*0  + 3*1  + 4*1  + 5*1  + 6*0  + 7*0  + 8*0  = 2
//! 1*0  + 2*0  + 3*0  + 4*1  + 5*1  + 6*1  + 7*1  + 8*0  = 2
//! 1*0  + 2*0  + 3*0  + 4*0  + 5*1  + 6*1  + 7*1  + 8*1  = 6
//! 1*0  + 2*0  + 3*0  + 4*0  + 5*0  + 6*1  + 7*1  + 8*1  = 1
//! 1*0  + 2*0  + 3*0  + 4*0  + 5*0  + 6*0  + 7*1  + 8*1  = 5
//! 1*0  + 2*0  + 3*0  + 4*0  + 5*0  + 6*0  + 7*0  + 8*1  = 8
//!
//! After 1 phase: 48226158
//!
//! 4*1  + 8*0  + 2*-1 + 2*0  + 6*1  + 1*0  + 5*-1 + 8*0  = 3
//! 4*0  + 8*1  + 2*1  + 2*0  + 6*0  + 1*-1 + 5*-1 + 8*0  = 4
//! 4*0  + 8*0  + 2*1  + 2*1  + 6*1  + 1*0  + 5*0  + 8*0  = 0
//! 4*0  + 8*0  + 2*0  + 2*1  + 6*1  + 1*1  + 5*1  + 8*0  = 4
//! 4*0  + 8*0  + 2*0  + 2*0  + 6*1  + 1*1  + 5*1  + 8*1  = 0
//! 4*0  + 8*0  + 2*0  + 2*0  + 6*0  + 1*1  + 5*1  + 8*1  = 4
//! 4*0  + 8*0  + 2*0  + 2*0  + 6*0  + 1*0  + 5*1  + 8*1  = 3
//! 4*0  + 8*0  + 2*0  + 2*0  + 6*0  + 1*0  + 5*0  + 8*1  = 8
//!
//! After 2 phases: 34040438
//!
//! 3*1  + 4*0  + 0*-1 + 4*0  + 0*1  + 4*0  + 3*-1 + 8*0  = 0
//! 3*0  + 4*1  + 0*1  + 4*0  + 0*0  + 4*-1 + 3*-1 + 8*0  = 3
//! 3*0  + 4*0  + 0*1  + 4*1  + 0*1  + 4*0  + 3*0  + 8*0  = 4
//! 3*0  + 4*0  + 0*0  + 4*1  + 0*1  + 4*1  + 3*1  + 8*0  = 1
//! 3*0  + 4*0  + 0*0  + 4*0  + 0*1  + 4*1  + 3*1  + 8*1  = 5
//! 3*0  + 4*0  + 0*0  + 4*0  + 0*0  + 4*1  + 3*1  + 8*1  = 5
//! 3*0  + 4*0  + 0*0  + 4*0  + 0*0  + 4*0  + 3*1  + 8*1  = 1
//! 3*0  + 4*0  + 0*0  + 4*0  + 0*0  + 4*0  + 3*0  + 8*1  = 8
//!
//! After 3 phases: 03415518
//!
//! 0*1  + 3*0  + 4*-1 + 1*0  + 5*1  + 5*0  + 1*-1 + 8*0  = 0
//! 0*0  + 3*1  + 4*1  + 1*0  + 5*0  + 5*-1 + 1*-1 + 8*0  = 1
//! 0*0  + 3*0  + 4*1  + 1*1  + 5*1  + 5*0  + 1*0  + 8*0  = 0
//! 0*0  + 3*0  + 4*0  + 1*1  + 5*1  + 5*1  + 1*1  + 8*0  = 2
//! 0*0  + 3*0  + 4*0  + 1*0  + 5*1  + 5*1  + 1*1  + 8*1  = 9
//! 0*0  + 3*0  + 4*0  + 1*0  + 5*0  + 5*1  + 1*1  + 8*1  = 4
//! 0*0  + 3*0  + 4*0  + 1*0  + 5*0  + 5*0  + 1*1  + 8*1  = 9
//! 0*0  + 3*0  + 4*0  + 1*0  + 5*0  + 5*0  + 1*0  + 8*1  = 8
//!
//! After 4 phases: 01029498
//!
//! Here are the first eight digits of the final output list after 100 phases for some larger
//! inputs:
//!
//!     80871224585914546619083218645595 becomes 24176176.
//!     19617804207202209144916044189917 becomes 73745418.
//!     69317163492948606335995924319873 becomes 52432133.
//!
//! After 100 phases of FFT, what are the first eight digits in the final output list?

use std::fs::File;
use std::io::{self, Read};

struct Repeater<I: Iterator> {
    iter: I,
    repeats: usize,
    count: usize,
    item: Option<I::Item>,
}

impl<I: Iterator> Repeater<I> {
    fn new(iter: I, repeats: usize) -> Repeater<I> {
        Repeater {
            iter,
            repeats,
            count: 0,
            item: None,
        }
    }
}

impl<I: Iterator> Iterator for Repeater<I>
where
    I::Item: Copy,
{
    type Item = I::Item;

    fn next(&mut self) -> Option<Self::Item> {
        if self.count == 0 {
            self.item = self.iter.next();
            self.count = self.repeats - 1;
        } else {
            self.count -= 1;
        }
        self.item
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        let (lower, upper) = self.iter.size_hint();
        (
            1 + lower.saturating_sub(1) * (self.repeats as usize),
            upper.map(|u| u * (self.repeats as usize)),
        )
    }
}

fn main() -> io::Result<()> {
    let file = File::open("input.txt")?;
    let mut input = file
        .bytes()
        .filter_map(|n| match n {
            Ok(n @ b'0'..=b'9') => Some(Ok((n - b'0') as i32)),
            Ok(_) => None,
            Err(e) => Some(Err(e)),
        })
        .collect::<Result<Vec<_>, _>>()?;
    let base_pattern = [0, 1, 0, -1];
    for _ in 0..100 {
        input = (0..input.len())
            .map(|i| {
                let total: i32 = input
                    .iter()
                    .zip(Repeater::new(base_pattern.iter().cycle(), i + 1).skip(1))
                    .map(|(a, b)| a * b)
                    .sum();
                total.abs() % 10
            })
            .collect();
    }
    println!(
        "Result: {}",
        input[..8]
            .iter()
            .map(|&n| char::from((n as u8) + b'0'))
            .collect::<String>()
    );

    Ok(())
}

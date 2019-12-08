//! --- Day 8: Space Image Format ---
//!
//! The Elves' spirits are lifted when they realize you have an opportunity to reboot one of their
//! Mars rovers, and so they are curious if you would spend a brief sojourn on Mars. You land your
//! ship near the rover.
//!
//! When you reach the rover, you discover that it's already in the process of rebooting! It's just
//! waiting for someone to enter a BIOS password. The Elf responsible for the rover takes a picture
//! of the password (your puzzle input) and sends it to you via the Digital Sending Network.
//!
//! Unfortunately, images sent via the Digital Sending Network aren't encoded with any normal
//! encoding; instead, they're encoded in a special Space Image Format. None of the Elves seem to
//! remember why this is the case. They send you the instructions to decode it.
//!
//! Images are sent as a series of digits that each represent the color of a single pixel. The
//! digits fill each row of the image left-to-right, then move downward to the next row, filling
//! rows top-to-bottom until every pixel of the image is filled.
//!
//! Each image actually consists of a series of identically-sized layers that are filled in this
//! way. So, the first digit corresponds to the top-left pixel of the first layer, the second digit
//! corresponds to the pixel to the right of that on the same layer, and so on until the last
//! digit, which corresponds to the bottom-right pixel of the last layer.
//!
//! For example, given an image 3 pixels wide and 2 pixels tall, the image data 123456789012
//! corresponds to the following image layers:
//!
//! Layer 1: 123
//!          456
//!
//! Layer 2: 789
//!          012
//!
//! The image you received is 25 pixels wide and 6 pixels tall.
//!
//! To make sure the image wasn't corrupted during transmission, the Elves would like you to find
//! the layer that contains the fewest 0 digits. On that layer, what is the number of 1 digits
//! multiplied by the number of 2 digits?

use std::fs::File;
use std::io::{self, Read};

const WIDTH: usize = 25;
const HEIGHT: usize = 6;

fn main() -> io::Result<()> {
    let file = File::open("input.txt")?;
    let mut fewest_zeros = std::usize::MAX;
    let mut result = 0;
    let mut counts = [0usize; 3];
    let mut i = WIDTH * HEIGHT;
    for byte in file.bytes() {
        let byte = byte?;
        if byte < b'0' || byte > b'9' {
            continue;
        }

        let digit = byte - b'0';
        if digit < 3 {
            counts[digit as usize] += 1;
        }

        i -= 1;
        if i == 0 {
            if counts[0] < fewest_zeros {
                fewest_zeros = counts[0];
                result = counts[1] * counts[2];
            }
            counts = [0; 3];
            i = WIDTH * HEIGHT;
        }
    }
    println!("Result: {}", result);
    Ok(())
}

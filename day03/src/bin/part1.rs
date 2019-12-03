//! --- Day 3: Crossed Wires ---
//!
//! The gravity assist was successful, and you're well on your way to the Venus refuelling station.
//! During the rush back on Earth, the fuel management system wasn't completely installed, so
//! that's next on the priority list.
//!
//! Opening the front panel reveals a jumble of wires. Specifically, two wires are connected to a
//! central port and extend outward on a grid. You trace the path each wire takes as it leaves the
//! central port, one wire per line of text (your puzzle input).
//!
//! The wires twist and turn, but the two wires occasionally cross paths. To fix the circuit, you
//! need to find the intersection point closest to the central port. Because the wires are on a
//! grid, use the Manhattan distance for this measurement. While the wires do technically cross
//! right at the central port where they both start, this point does not count, nor does a wire
//! count as crossing with itself.
//!
//! For example, if the first wire's path is R8,U5,L5,D3, then starting from the central port (o),
//! it goes right 8, up 5, left 5, and finally down 3:
//!
//! ...........
//! ...........
//! ...........
//! ....+----+.
//! ....|....|.
//! ....|....|.
//! ....|....|.
//! .........|.
//! .o-------+.
//! ...........
//!
//! Then, if the second wire's path is U7,R6,D4,L4, it goes up 7, right 6, down 4, and left 4:
//!
//! ...........
//! .+-----+...
//! .|.....|...
//! .|..+--X-+.
//! .|..|..|.|.
//! .|.-X--+.|.
//! .|..|....|.
//! .|.......|.
//! .o-------+.
//! ...........
//!
//! These wires cross at two locations (marked X), but the lower-left one is closer to the central
//! port: its distance is 3 + 3 = 6.
//!
//! Here are a few more examples:
//!
//!     R75,D30,R83,U83,L12,D49,R71,U7,L72
//!     U62,R66,U55,R34,D71,R55,D58,R83 = distance 159
//!     R98,U47,R26,D63,R33,U87,L62,D20,R33,U53,R51
//!     U98,R91,D20,R16,D67,R40,U7,R15,U6,R7 = distance 135
//!
//! What is the Manhattan distance from the central port to the closest intersection?

use std::cmp::{Eq, PartialEq};
use std::collections::HashSet;
use std::fs::File;
use std::hash::Hash;
use std::io::{self, BufRead, BufReader, ErrorKind};
use std::str;

#[derive(Eq, PartialEq, Hash)]
struct Point(i32, i32);

struct Wire(u8, i32, i32, u8, i32);

impl Wire {
    fn end_point(&self) -> (i32, i32) {
        match self.3 {
            b'U' => (self.1, self.2 + self.4),
            b'R' => (self.1 + self.4, self.2),
            b'D' => (self.1, self.2 - self.4),
            b'L' => (self.1 - self.4, self.2),
            d => panic!("Unknown direction: {}", d),
        }
    }
}

impl Iterator for Wire {
    type Item = Point;

    fn next(&mut self) -> Option<Self::Item> {
        if self.4 <= 0 {
            return None;
        }

        self.4 -= 1;
        match self.3 {
            b'U' => self.2 += 1,
            b'R' => self.1 += 1,
            b'D' => self.2 -= 1,
            b'L' => self.1 -= 1,
            d => panic!("Unknown direction: {}", d),
        };
        Some(Point(self.1, self.2))
    }
}

struct WireReader<B> {
    reader: B,
    buf: Vec<u8>,
    wire: u8,
    p: (i32, i32),
}

impl<B: BufRead> WireReader<B> {
    fn new(reader: B) -> WireReader<B> {
        WireReader {
            reader,
            buf: vec![],
            wire: 0,
            p: (0, 0),
        }
    }
}

impl<B: BufRead> Iterator for WireReader<B> {
    type Item = io::Result<Wire>;

    fn next(&mut self) -> Option<Self::Item> {
        let wire_id = self.wire;
        let (x, y) = self.p;
        self.buf.clear();
        loop {
            let (done, used) = {
                let available = match self.reader.fill_buf() {
                    Ok(n) => n,
                    Err(ref e) if e.kind() == ErrorKind::Interrupted => continue,
                    Err(e) => return Some(Err(e)),
                };

                // find position of comma or newline
                let mut idx = None;
                for i in 0..available.len() {
                    if available[i] == b',' || available[i] == b'\n' {
                        if available[i] == b'\n' {
                            self.wire += 1;
                        }
                        idx = Some(i);
                        break;
                    }
                }

                match idx {
                    Some(i) => {
                        self.buf.extend_from_slice(&available[..i]);
                        (true, i + 1)
                    }
                    None => {
                        self.buf.extend_from_slice(&available);
                        (false, available.len())
                    }
                }
            };
            self.reader.consume(used);
            if used == 0 {
                // no more bytes left; EOF reached
                return None;
            }
            if done {
                // found comma or newline; return result
                let length: i32 = str::from_utf8(&self.buf[1..]).unwrap().parse().unwrap();
                let wire = Wire(wire_id, x, y, self.buf[0], length);
                if wire_id != self.wire {
                    self.p = (0, 0);
                } else {
                    self.p = wire.end_point();
                }
                return Some(Ok(wire));
            }
        }
    }
}

fn main() -> io::Result<()> {
    let file = File::open("input.txt")?;
    let reader = BufReader::new(file);

    let mut sets: [HashSet<Point>; 2] = [HashSet::new(), HashSet::new()];
    for wire in WireReader::new(reader) {
        let wire = wire.unwrap();
        let id = wire.0 as usize;
        for p in wire {
            sets[id].insert(p);
        }
    }

    if let Some(result) = sets[0]
        .intersection(&sets[1])
        .map(|Point(x, y)| x.abs() + y.abs())
        .min()
    {
        println!("Minimum: {}", result);
    }

    Ok(())
}

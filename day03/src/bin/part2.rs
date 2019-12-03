//! --- Part Two ---
//!
//! It turns out that this circuit is very timing-sensitive; you actually need to minimize the
//! signal delay.
//!
//! To do this, calculate the number of steps each wire takes to reach each intersection; choose
//! the intersection where the sum of both wires' steps is lowest. If a wire visits a position on
//! the grid multiple times, use the steps value from the first time it visits that position when
//! calculating the total value of a specific intersection.
//!
//! The number of steps a wire takes is the total number of grid squares the wire has entered to
//! get to that location, including the intersection being considered. Again consider the example
//! from above:
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
//! In the above example, the intersection closest to the central port is reached after 8+5+5+2 =
//! 20 steps by the first wire and 7+6+4+3 = 20 steps by the second wire for a total of 20+20 = 40
//! steps.
//!
//! However, the top-right intersection is better: the first wire takes only 8+5+2 = 15 and the
//! second wire takes only 7+6+2 = 15, a total of 15+15 = 30 steps.
//!
//! Here are the best steps for the extra examples from above:
//!
//!     R75,D30,R83,U83,L12,D49,R71,U7,L72
//!     U62,R66,U55,R34,D71,R55,D58,R83 = 610 steps
//!     R98,U47,R26,D63,R33,U87,L62,D20,R33,U53,R51
//!     U98,R91,D20,R16,D67,R40,U7,R15,U6,R7 = 410 steps
//!
//! What is the fewest combined steps the wires must take to reach an intersection?

use std::cmp::{Eq, PartialEq};
use std::collections::HashSet;
use std::fs::File;
use std::hash::{Hash, Hasher};
use std::io::{self, BufRead, BufReader, ErrorKind};
use std::str;

struct Point(i32, i32, i32);

impl PartialEq for Point {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0 && self.1 == other.1
    }
}

impl Eq for Point { }

impl Hash for Point {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.0.hash(state);
        self.1.hash(state);
    }
}

struct Wire(u8, i32, i32, i32, u8, i32);

impl Wire {
    fn end_point(&self) -> (i32, i32, i32) {
        match self.4 {
            b'U' => (self.1, self.2 + self.5, self.3 + self.5),
            b'R' => (self.1 + self.5, self.2, self.3 + self.5),
            b'D' => (self.1, self.2 - self.5, self.3 + self.5),
            b'L' => (self.1 - self.5, self.2, self.3 + self.5),
            d => panic!("Unknown direction: {}", d),
        }
    }
}

impl Iterator for Wire {
    type Item = Point;

    fn next(&mut self) -> Option<Self::Item> {
        if self.5 <= 0 {
            return None;
        }

        self.3 += 1;
        self.5 -= 1;
        match self.4 {
            b'U' => self.2 += 1,
            b'R' => self.1 += 1,
            b'D' => self.2 -= 1,
            b'L' => self.1 -= 1,
            d => panic!("Unknown direction: {}", d),
        };
        Some(Point(self.1, self.2, self.3))
    }
}

struct WireReader<B> {
    reader: B,
    buf: Vec<u8>,
    wire: u8,
    p: (i32, i32, i32),
}

impl<B: BufRead> WireReader<B> {
    fn new(reader: B) -> WireReader<B> {
        WireReader {
            reader,
            buf: vec![],
            wire: 0,
            p: (0, 0, 0),
        }
    }
}

impl<B: BufRead> Iterator for WireReader<B> {
    type Item = io::Result<Wire>;

    fn next(&mut self) -> Option<Self::Item> {
        let wire_id = self.wire;
        let (x, y, cnt) = self.p;
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
                    },
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
                let wire = Wire(wire_id, x, y, cnt, self.buf[0], length);
                if wire_id != self.wire {
                    self.p = (0, 0, 0);
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

    let mut min = std::i32::MAX;
    for p in sets[0].iter() {
        if let Some(p2) = sets[1].get(p) {
            let new_min = p.2 + p2.2;
            if new_min < min {
                min = new_min;
            }
        }
    }
    println!("Minimum: {}", min);

    Ok(())
}

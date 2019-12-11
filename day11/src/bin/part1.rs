//! --- Day 11: Space Police ---
//!
//! On the way to Jupiter, you're pulled over by the Space Police.
//!
//! "Attention, unmarked spacecraft! You are in violation of Space Law! All spacecraft must have a
//! clearly visible registration identifier! You have 24 hours to comply or be sent to Space Jail!"
//!
//! Not wanting to be sent to Space Jail, you radio back to the Elves on Earth for help. Although
//! it takes almost three hours for their reply signal to reach you, they send instructions for how
//! to power up the emergency hull painting robot and even provide a small Intcode program (your
//! puzzle input) that will cause it to paint your ship appropriately.
//!
//! There's just one problem: you don't have an emergency hull painting robot.
//!
//! You'll need to build a new emergency hull painting robot. The robot needs to be able to move
//! around on the grid of square panels on the side of your ship, detect the color of its current
//! panel, and paint its current panel black or white. (All of the panels are currently black.)
//!
//! The Intcode program will serve as the brain of the robot. The program uses input instructions
//! to access the robot's camera: provide 0 if the robot is over a black panel or 1 if the robot is
//! over a white panel. Then, the program will output two values:
//!
//!   - First, it will output a value indicating the color to paint the panel the robot is over: 0
//!     means to paint the panel black, and 1 means to paint the panel white.
//!   - Second, it will output a value indicating the direction the robot should turn: 0 means it
//!     should turn left 90 degrees, and 1 means it should turn right 90 degrees.
//!
//! After the robot turns, it should always move forward exactly one panel. The robot starts facing
//! up.
//!
//! The robot will continue running for a while like this and halt when it is finished drawing. Do
//! not restart the Intcode computer inside the robot during this process.
//!
//! For example, suppose the robot is about to start running. Drawing black panels as ., white
//! panels as #, and the robot pointing the direction it is facing (< ^ > v), the initial state and
//! region near the robot looks like this:
//!
//! .....
//! .....
//! ..^..
//! .....
//! .....
//!
//! The panel under the robot (not visible here because a ^ is shown instead) is also black, and so
//! any input instructions at this point should be provided 0. Suppose the robot eventually outputs
//! 1 (paint white) and then 0 (turn left). After taking these actions and moving forward one
//! panel, the region now looks like this:
//!
//! .....
//! .....
//! .<#..
//! .....
//! .....
//!
//! Input instructions should still be provided 0. Next, the robot might output 0 (paint black) and
//! then 0 (turn left):
//!
//! .....
//! .....
//! ..#..
//! .v...
//! .....
//!
//! After more outputs (1,0, 1,0):
//!
//! .....
//! .....
//! ..^..
//! .##..
//! .....
//!
//! The robot is now back where it started, but because it is now on a white panel, input
//! instructions should be provided 1. After several more outputs (0,1, 1,0, 1,0), the area looks
//! like this:
//!
//! .....
//! ..<#.
//! ...#.
//! .##..
//! .....
//!
//! Before you deploy the robot, you should probably have an estimate of the area it will cover:
//! specifically, you need to know the number of panels it paints at least once, regardless of
//! color. In the example above, the robot painted 6 panels at least once. (It painted its starting
//! panel twice, but that panel is still only counted once; it also never painted the panel it
//! ended on.)
//!
//! Build a new emergency hull painting robot and run the Intcode program on it. How many panels
//! does it paint at least once?

use std::clone::Clone;
use std::cmp::{Eq, PartialEq};
use std::collections::HashMap;
use std::convert::{TryFrom, TryInto};
use std::error::Error;
use std::fmt::{self, Display};
use std::fs::File;
use std::hash::Hash;
use std::io::{BufRead, BufReader};
use std::str;

#[derive(Debug)]
struct InvalidParameterMode(i64);

impl Display for InvalidParameterMode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} is not a valid parameter mode.", self.0)
    }
}

impl Error for InvalidParameterMode {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        None
    }
}

#[derive(Debug)]
struct InvalidOpcode(i64);

impl Display for InvalidOpcode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} is not a valid opcode.", self.0)
    }
}

impl Error for InvalidOpcode {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        None
    }
}

#[derive(Debug)]
struct InvalidOutputMode(ParameterMode);

impl Display for InvalidOutputMode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} is not a valid parameter mode for output.", self.0)
    }
}

impl Error for InvalidOutputMode {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        None
    }
}

#[derive(Debug, PartialEq)]
enum ParameterMode {
    Position,
    Immediate,
    Relative,
}

impl Display for ParameterMode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                ParameterMode::Position => "'position'",
                ParameterMode::Immediate => "'immediate'",
                ParameterMode::Relative => "'relative'",
            }
        )
    }
}

impl TryFrom<i64> for ParameterMode {
    type Error = InvalidParameterMode;

    fn try_from(code: i64) -> Result<Self, Self::Error> {
        match code {
            0 => Ok(ParameterMode::Position),
            1 => Ok(ParameterMode::Immediate),
            2 => Ok(ParameterMode::Relative),
            _ => Err(InvalidParameterMode(code)),
        }
    }
}

impl ParameterMode {
    fn modes2(code: i64) -> Result<(Self, Self), InvalidParameterMode> {
        Ok(((code % 10).try_into()?, ((code / 10) % 10).try_into()?))
    }

    fn modes3(code: i64) -> Result<(Self, Self, Self), InvalidParameterMode> {
        Ok((
            (code % 10).try_into()?,
            ((code / 10) % 10).try_into()?,
            ((code / 100) % 10).try_into()?,
        ))
    }
}

enum Opcode {
    Add(ParameterMode, ParameterMode, ParameterMode),
    Multiply(ParameterMode, ParameterMode, ParameterMode),
    Input(ParameterMode),
    Output(ParameterMode),
    JumpIfTrue(ParameterMode, ParameterMode),
    JumpIfFalse(ParameterMode, ParameterMode),
    LessThan(ParameterMode, ParameterMode, ParameterMode),
    Equal(ParameterMode, ParameterMode, ParameterMode),
    RelativeBaseOffset(ParameterMode),
    Halt,
}

impl TryFrom<i64> for Opcode {
    type Error = InvalidOpcode;

    fn try_from(code: i64) -> Result<Self, Self::Error> {
        match code % 100 {
            1 => {
                let (mode1, mode2, mode3) =
                    ParameterMode::modes3(code / 100).map_err(|_| InvalidOpcode(code))?;
                Ok(Opcode::Add(mode1, mode2, mode3))
            }
            2 => {
                let (mode1, mode2, mode3) =
                    ParameterMode::modes3(code / 100).map_err(|_| InvalidOpcode(code))?;
                Ok(Opcode::Multiply(mode1, mode2, mode3))
            }
            3 => {
                let mode = (code / 100).try_into().map_err(|_| InvalidOpcode(code))?;
                Ok(Opcode::Input(mode))
            }
            4 => {
                let mode = (code / 100).try_into().map_err(|_| InvalidOpcode(code))?;
                Ok(Opcode::Output(mode))
            }
            5 => {
                let (mode1, mode2) =
                    ParameterMode::modes2(code / 100).map_err(|_| InvalidOpcode(code))?;
                Ok(Opcode::JumpIfTrue(mode1, mode2))
            }
            6 => {
                let (mode1, mode2) =
                    ParameterMode::modes2(code / 100).map_err(|_| InvalidOpcode(code))?;
                Ok(Opcode::JumpIfFalse(mode1, mode2))
            }
            7 => {
                let (mode1, mode2, mode3) =
                    ParameterMode::modes3(code / 100).map_err(|_| InvalidOpcode(code))?;
                Ok(Opcode::LessThan(mode1, mode2, mode3))
            }
            8 => {
                let (mode1, mode2, mode3) =
                    ParameterMode::modes3(code / 100).map_err(|_| InvalidOpcode(code))?;
                Ok(Opcode::Equal(mode1, mode2, mode3))
            }
            9 => {
                let mode = (code / 100).try_into().map_err(|_| InvalidOpcode(code))?;
                Ok(Opcode::RelativeBaseOffset(mode))
            }
            99 => Ok(Opcode::Halt),
            _ => Err(InvalidOpcode(code)),
        }
    }
}

#[derive(Clone)]
struct Intcode {
    code: Vec<i64>,
    ip: usize,
    relative_base: isize,
}

impl Intcode {
    fn load<R: BufRead>(reader: R) -> Result<Intcode, Box<dyn Error>> {
        let code = reader
            .split(b',')
            .map(|code| match code {
                Ok(code) => {
                    let s = str::from_utf8(&code)?;
                    Ok(s.trim().parse()?)
                }
                Err(e) => Err(Box::new(e) as Box<dyn Error>),
            })
            .collect::<Result<Vec<_>, _>>()?;
        Ok(Intcode {
            code,
            ip: 0,
            relative_base: 0,
        })
    }

    fn read(&mut self, i: usize) -> i64 {
        if i >= self.code.len() {
            self.code.resize(i + 1, 0);
        }
        self.code[i]
    }

    fn write(&mut self, i: usize, v: i64) {
        if i >= self.code.len() {
            self.code.resize(i + 1, 0);
        }
        self.code[i] = v;
    }

    fn output_operand(
        &mut self,
        sp: usize,
        mode: ParameterMode,
    ) -> Result<usize, InvalidOutputMode> {
        let v = self.code[sp];
        match mode {
            ParameterMode::Position => Ok(v as usize),
            ParameterMode::Immediate => Err(InvalidOutputMode(mode)),
            ParameterMode::Relative => Ok((v as isize + self.relative_base) as usize),
        }
    }

    fn operand(&mut self, sp: usize, mode: ParameterMode) -> i64 {
        let v = self.code[sp];
        match mode {
            ParameterMode::Position => self.read(v as usize),
            ParameterMode::Immediate => v,
            ParameterMode::Relative => self.read((v as isize + self.relative_base) as usize),
        }
    }

    fn operands2(&mut self, sp: usize, mode1: ParameterMode, mode2: ParameterMode) -> (i64, i64) {
        (self.operand(sp, mode1), self.operand(sp + 1, mode2))
    }

    fn run(&mut self, input: &Vec<i64>) -> Result<(Vec<i64>, bool), Box<dyn Error>> {
        let mut output: Vec<i64> = Vec::new();
        let mut inputp = 0;
        let mut halted = false;
        loop {
            let opcode: Opcode = self.read(self.ip).try_into()?;
            self.ip = match opcode {
                Opcode::Add(mode1, mode2, mode3) => {
                    let (op1, op2) = self.operands2(self.ip + 1, mode1, mode2);
                    let op3 = self.output_operand(self.ip + 3, mode3)?;
                    self.write(op3 as usize, op1 + op2);
                    self.ip + 4
                }
                Opcode::Multiply(mode1, mode2, mode3) => {
                    let (op1, op2) = self.operands2(self.ip + 1, mode1, mode2);
                    let op3 = self.output_operand(self.ip + 3, mode3)?;
                    self.write(op3 as usize, op1 * op2);
                    self.ip + 4
                }
                Opcode::Input(mode) => {
                    if inputp >= input.len() {
                        // not enough input; suspend
                        break;
                    }

                    let op = self.output_operand(self.ip + 1, mode)?;
                    self.write(op, input[inputp]);
                    inputp += 1;
                    self.ip + 2
                }
                Opcode::Output(mode) => {
                    let op = self.operand(self.ip + 1, mode);
                    output.push(op);
                    self.ip + 2
                }
                Opcode::JumpIfTrue(mode1, mode2) => {
                    let (op1, op2) = self.operands2(self.ip + 1, mode1, mode2);
                    if op1 != 0 {
                        op2 as usize
                    } else {
                        self.ip + 3
                    }
                }
                Opcode::JumpIfFalse(mode1, mode2) => {
                    let (op1, op2) = self.operands2(self.ip + 1, mode1, mode2);
                    if op1 == 0 {
                        op2 as usize
                    } else {
                        self.ip + 3
                    }
                }
                Opcode::LessThan(mode1, mode2, mode3) => {
                    let (op1, op2) = self.operands2(self.ip + 1, mode1, mode2);
                    let op3 = self.output_operand(self.ip + 3, mode3)?;
                    self.write(op3 as usize, if op1 < op2 { 1 } else { 0 });
                    self.ip + 4
                }
                Opcode::Equal(mode1, mode2, mode3) => {
                    let (op1, op2) = self.operands2(self.ip + 1, mode1, mode2);
                    let op3 = self.output_operand(self.ip + 3, mode3)?;
                    self.write(op3 as usize, if op1 == op2 { 1 } else { 0 });
                    self.ip + 4
                }
                Opcode::RelativeBaseOffset(mode) => {
                    let op = self.operand(self.ip + 1, mode);
                    self.relative_base += op as isize;
                    self.ip + 2
                }
                Opcode::Halt => {
                    halted = true;
                    break;
                }
            };
        }
        Ok((output, halted))
    }
}

#[derive(Eq, Hash, PartialEq)]
struct HullPanel {
    x: isize,
    y: isize,
}

impl HullPanel {
    fn new(x: isize, y: isize) -> HullPanel {
        HullPanel { x, y }
    }
}

enum Direction {
    Down,
    Left,
    Right,
    Up,
}

impl Direction {
    fn turn(self, how: i64) -> Direction {
        match self {
            Direction::Down => {
                if how == 0 {
                    Direction::Right
                } else {
                    Direction::Left
                }
            }
            Direction::Left => {
                if how == 0 {
                    Direction::Down
                } else {
                    Direction::Up
                }
            }
            Direction::Right => {
                if how == 0 {
                    Direction::Up
                } else {
                    Direction::Down
                }
            }
            Direction::Up => {
                if how == 0 {
                    Direction::Left
                } else {
                    Direction::Right
                }
            }
        }
    }

    fn r#move(&self, x: isize, y: isize) -> (isize, isize) {
        match self {
            Direction::Down => (x, y + 1),
            Direction::Left => (x - 1, y),
            Direction::Right => (x + 1, y),
            Direction::Up => (x, y - 1),
        }
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let file = File::open("input.txt")?;
    let reader = BufReader::new(file);
    let mut program = Intcode::load(reader)?;
    let mut panels = HashMap::new();
    let mut direction = Direction::Up;
    let mut x = 0;
    let mut y = 0;
    let mut input = vec![1];
    loop {
        let panel = HullPanel::new(x, y);
        let color = panels.entry(panel).or_default();
        input[0] = *color;

        let (output, halted) = program.run(&input)?;
        if output.len() == 2 {
            *color = output[0];
            direction = direction.turn(output[1]);

            let (newx, newy) = direction.r#move(x, y);
            x = newx;
            y = newy;
        }
        if halted {
            break;
        }
    }
    println!("Painted panels: {}", panels.len());

    Ok(())
}

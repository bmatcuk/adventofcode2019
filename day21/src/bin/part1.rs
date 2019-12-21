//! --- Day 21: Springdroid Adventure ---
//!
//! You lift off from Pluto and start flying in the direction of Santa.
//!
//! While experimenting further with the tractor beam, you accidentally pull an asteroid directly
//! into your ship! It deals significant damage to your hull and causes your ship to begin tumbling
//! violently.
//!
//! You can send a droid out to investigate, but the tumbling is causing enough artificial gravity
//! that one wrong step could send the droid through a hole in the hull and flying out into space.
//!
//! The clear choice for this mission is a droid that can jump over the holes in the hull - a
//! springdroid.
//!
//! You can use an Intcode program (your puzzle input) running on an ASCII-capable computer to
//! program the springdroid. However, springdroids don't run Intcode; instead, they run a
//! simplified assembly language called springscript.
//!
//! While a springdroid is certainly capable of navigating the artificial gravity and giant holes,
//! it has one downside: it can only remember at most 15 springscript instructions.
//!
//! The springdroid will move forward automatically, constantly thinking about whether to jump. The
//! springscript program defines the logic for this decision.
//!
//! Springscript programs only use Boolean values, not numbers or strings. Two registers are
//! available: T, the temporary value register, and J, the jump register. If the jump register is
//! true at the end of the springscript program, the springdroid will try to jump. Both of these
//! registers start with the value false.
//!
//! Springdroids have a sensor that can detect whether there is ground at various distances in the
//! direction it is facing; these values are provided in read-only registers. Your springdroid can
//! detect ground at four distances: one tile away (A), two tiles away (B), three tiles away (C),
//! and four tiles away (D). If there is ground at the given distance, the register will be true;
//! if there is a hole, the register will be false.
//!
//! There are only three instructions available in springscript:
//!
//!     AND X Y sets Y to true if both X and Y are true; otherwise, it sets Y to false.
//!     OR X Y sets Y to true if at least one of X or Y is true; otherwise, it sets Y to false.
//!     NOT X Y sets Y to true if X is false; otherwise, it sets Y to false.
//!
//! In all three instructions, the second argument (Y) needs to be a writable register (either T or
//! J). The first argument (X) can be any register (including A, B, C, or D).
//!
//! For example, the one-instruction program NOT A J means "if the tile immediately in front of me
//! is not ground, jump".
//!
//! Or, here is a program that jumps if a three-tile-wide hole (with ground on the other side of
//! the hole) is detected:
//!
//! NOT A J
//! NOT B T
//! AND T J
//! NOT C T
//! AND T J
//! AND D J
//!
//! The Intcode program expects ASCII inputs and outputs. It will begin by displaying a prompt;
//! then, input the desired instructions one per line. End each line with a newline (ASCII code
//! 10). When you have finished entering your program, provide the command WALK followed by a
//! newline to instruct the springdroid to begin surveying the hull.
//!
//! If the springdroid falls into space, an ASCII rendering of the last moments of its life will be
//! produced. In these, @ is the springdroid, # is hull, and . is empty space. For example, suppose
//! you program the springdroid like this:
//!
//! NOT D J
//! WALK
//!
//! This one-instruction program sets J to true if and only if there is no ground four tiles away.
//! In other words, it attempts to jump into any hole it finds:
//!
//! .................
//! .................
//! @................
//! #####.###########
//!
//! .................
//! .................
//! .@...............
//! #####.###########
//!
//! .................
//! ..@..............
//! .................
//! #####.###########
//!
//! ...@.............
//! .................
//! .................
//! #####.###########
//!
//! .................
//! ....@............
//! .................
//! #####.###########
//!
//! .................
//! .................
//! .....@...........
//! #####.###########
//!
//! .................
//! .................
//! .................
//! #####@###########
//!
//! However, if the springdroid successfully makes it across, it will use an output instruction to
//! indicate the amount of damage to the hull as a single giant integer outside the normal ASCII
//! range.
//!
//! Program the springdroid with logic that allows it to survey the hull without falling into
//! space. What amount of hull damage does it report?

use std::clone::Clone;
use std::cmp::PartialEq;
use std::convert::{TryFrom, TryInto};
use std::error::Error;
use std::fmt::{self, Display};
use std::fs::File;
use std::io::{self, BufRead, BufReader};
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

fn render_output(output: &Vec<i64>) -> bool {
    let mut s = String::with_capacity(output.len());
    let mut done = false;
    for c in output.iter() {
        if (0..256).contains(c) {
            s.push(char::from(*c as u8));
        } else {
            s.push_str(&c.to_string());
            done = true;
        }
    }
    println!("{}", s);
    done
}

#[allow(dead_code)]
fn read_input() -> Result<Vec<i64>, Box<dyn Error>> {
    let stdio = io::stdin();
    let mut s = String::new();
    let mut input = Vec::new();
    while s != "WALK\n" {
        s.clear();
        stdio.read_line(&mut s)?;
        for c in s.bytes() {
            input.push(c as i64);
        }
    }

    Ok(input)
}

#[allow(dead_code)]
fn interactive(program: &Intcode) -> Result<(), Box<dyn Error>> {
    loop {
        let mut program = program.clone();
        let (output, _) = program.run(&vec![])?;
        render_output(&output);

        let input = read_input()?;
        let (output, _) = program.run(&input)?;
        if render_output(&output) {
            break;
        }
    }
    Ok(())
}

fn main() -> Result<(), Box<dyn Error>> {
    let file = File::open("input.txt")?;
    let reader = BufReader::new(file);
    let mut program = Intcode::load(reader)?;
    let input = [
        "OR A J\n",
        "AND B J\n",
        "AND C J\n",
        "NOT J J\n",
        "AND D J\n",
        "WALK\n",
    ];
    let input = input
        .iter()
        .flat_map(|l| l.bytes().map(|c| c as i64))
        .collect();
    let (output, _) = program.run(&input)?;
    render_output(&output);

    Ok(())
}

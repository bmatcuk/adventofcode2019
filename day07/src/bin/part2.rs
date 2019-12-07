//! --- Part Two ---
//!
//! It's no good - in this configuration, the amplifiers can't generate a large enough output
//! signal to produce the thrust you'll need. The Elves quickly talk you through rewiring the
//! amplifiers into a feedback loop:
//!
//!       O-------O  O-------O  O-------O  O-------O  O-------O
//! 0 -+->| Amp A |->| Amp B |->| Amp C |->| Amp D |->| Amp E |-.
//!    |  O-------O  O-------O  O-------O  O-------O  O-------O |
//!    |                                                        |
//!    '--------------------------------------------------------+
//!                                                             |
//!                                                             v
//!                                                      (to thrusters)
//!
//! Most of the amplifiers are connected as they were before; amplifier A's output is connected to
//! amplifier B's input, and so on. However, the output from amplifier E is now connected into
//! amplifier A's input. This creates the feedback loop: the signal will be sent through the
//! amplifiers many times.
//!
//! In feedback loop mode, the amplifiers need totally different phase settings: integers from 5 to
//! 9, again each used exactly once. These settings will cause the Amplifier Controller Software to
//! repeatedly take input and produce output many times before halting. Provide each amplifier its
//! phase setting at its first input instruction; all further input/output instructions are for
//! signals.
//!
//! Don't restart the Amplifier Controller Software on any amplifier during this process. Each one
//! should continue receiving and sending signals until it halts.
//!
//! All signals sent or received in this process will be between pairs of amplifiers except the
//! very first signal and the very last signal. To start the process, a 0 signal is sent to
//! amplifier A's input exactly once.
//!
//! Eventually, the software on the amplifiers will halt after they have processed the final loop.
//! When this happens, the last output signal from amplifier E is sent to the thrusters. Your job
//! is to find the largest output signal that can be sent to the thrusters using the new phase
//! settings and feedback loop arrangement.
//!
//! Here are some example programs:
//!
//!     Max thruster signal 139629729 (from phase setting sequence 9,8,7,6,5):
//!
//!     3,26,1001,26,-4,26,3,27,1002,27,2,27,1,27,26,
//!     27,4,27,1001,28,-1,28,1005,28,6,99,0,0,5
//!
//!     Max thruster signal 18216 (from phase setting sequence 9,7,8,5,6):
//!
//!     3,52,1001,52,-5,52,3,53,1,52,56,54,1007,54,5,55,1005,55,26,1001,54,
//!     -5,54,1105,1,12,1,53,54,53,1008,54,0,55,1001,55,1,55,2,53,55,53,4,
//!     53,1001,56,-1,56,1005,56,6,99,0,0,0,0,10
//!
//! Try every combination of the new phase settings on the amplifier feedback loop. What is the
//! highest signal that can be sent to the thrusters?

use std::clone::Clone;
use std::convert::{TryFrom, TryInto};
use std::error::Error;
use std::fmt::{self, Display};
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::str;

const AMPLIFIERS: usize = 5;

#[derive(Debug)]
struct InvalidParameterMode(i32);

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
struct InvalidOpcode(i32);

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

enum ParameterMode {
    Position,
    Immediate,
}

impl TryFrom<i32> for ParameterMode {
    type Error = InvalidParameterMode;

    fn try_from(code: i32) -> Result<Self, Self::Error> {
        match code {
            0 => Ok(ParameterMode::Position),
            1 => Ok(ParameterMode::Immediate),
            _ => Err(InvalidParameterMode(code)),
        }
    }
}

impl ParameterMode {
    fn modes2(code: i32) -> Result<(Self, Self), InvalidParameterMode> {
        Ok(((code % 10).try_into()?, ((code / 10) % 10).try_into()?))
    }
}

enum Opcode {
    Add(ParameterMode, ParameterMode),
    Multiply(ParameterMode, ParameterMode),
    Input(ParameterMode),
    Output(ParameterMode),
    JumpIfTrue(ParameterMode, ParameterMode),
    JumpIfFalse(ParameterMode, ParameterMode),
    LessThan(ParameterMode, ParameterMode),
    Equal(ParameterMode, ParameterMode),
    Halt,
}

impl TryFrom<i32> for Opcode {
    type Error = InvalidOpcode;

    fn try_from(code: i32) -> Result<Self, Self::Error> {
        match code % 100 {
            1 => {
                let (mode1, mode2) =
                    ParameterMode::modes2(code / 100).map_err(|_| InvalidOpcode(code))?;
                Ok(Opcode::Add(mode1, mode2))
            }
            2 => {
                let (mode1, mode2) =
                    ParameterMode::modes2(code / 100).map_err(|_| InvalidOpcode(code))?;
                Ok(Opcode::Multiply(mode1, mode2))
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
                let (mode1, mode2) =
                    ParameterMode::modes2(code / 100).map_err(|_| InvalidOpcode(code))?;
                Ok(Opcode::LessThan(mode1, mode2))
            }
            8 => {
                let (mode1, mode2) =
                    ParameterMode::modes2(code / 100).map_err(|_| InvalidOpcode(code))?;
                Ok(Opcode::Equal(mode1, mode2))
            }
            99 => Ok(Opcode::Halt),
            _ => Err(InvalidOpcode(code)),
        }
    }
}

#[derive(Clone)]
struct Intcode(Vec<i32>, usize);

impl Intcode {
    fn load<R: BufRead>(reader: R) -> Result<Intcode, Box<dyn Error>> {
        let program = reader
            .split(b',')
            .map(|code| match code {
                Ok(code) => {
                    let s = str::from_utf8(&code)?;
                    Ok(s.trim().parse()?)
                }
                Err(e) => Err(Box::new(e) as Box<dyn Error>),
            })
            .collect::<Result<Vec<_>, _>>()?;
        Ok(Intcode(program, 0))
    }

    fn operand(&self, sp: usize, mode: ParameterMode) -> i32 {
        let v = self.0[sp];
        match mode {
            ParameterMode::Position => self.0[v as usize],
            ParameterMode::Immediate => v,
        }
    }

    fn operands2(&self, sp: usize, mode1: ParameterMode, mode2: ParameterMode) -> (i32, i32) {
        (self.operand(sp, mode1), self.operand(sp + 1, mode2))
    }

    fn operands3(
        &self,
        sp: usize,
        mode1: ParameterMode,
        mode2: ParameterMode,
        mode3: ParameterMode,
    ) -> (i32, i32, i32) {
        (
            self.operand(sp, mode1),
            self.operand(sp + 1, mode2),
            self.operand(sp + 2, mode3),
        )
    }

    fn run(&mut self, input: &Vec<i32>) -> Result<(Vec<i32>, bool), Box<dyn Error>> {
        let mut output: Vec<i32> = Vec::new();
        let mut inputp = 0;
        let mut ip = self.1;
        let mut halted = false;
        loop {
            let opcode: Opcode = self.0[ip].try_into()?;
            ip = match opcode {
                Opcode::Add(mode1, mode2) => {
                    let (op1, op2, op3) =
                        self.operands3(ip + 1, mode1, mode2, ParameterMode::Immediate);
                    self.0[op3 as usize] = op1 + op2;
                    ip + 4
                }
                Opcode::Multiply(mode1, mode2) => {
                    let (op1, op2, op3) =
                        self.operands3(ip + 1, mode1, mode2, ParameterMode::Immediate);
                    self.0[op3 as usize] = op1 * op2;
                    ip + 4
                }
                Opcode::Input(_mode) => {
                    if inputp >= input.len() {
                        // not enough input; suspend
                        self.1 = ip;
                        break;
                    }

                    // not sure what to do if mode is not immediate?
                    let op = self.operand(ip + 1, ParameterMode::Immediate) as usize;
                    self.0[op] = input[inputp];
                    inputp += 1;
                    ip + 2
                }
                Opcode::Output(mode) => {
                    let op = self.operand(ip + 1, mode);
                    output.push(op);
                    ip + 2
                }
                Opcode::JumpIfTrue(mode1, mode2) => {
                    let (op1, op2) = self.operands2(ip + 1, mode1, mode2);
                    if op1 != 0 {
                        op2 as usize
                    } else {
                        ip + 3
                    }
                }
                Opcode::JumpIfFalse(mode1, mode2) => {
                    let (op1, op2) = self.operands2(ip + 1, mode1, mode2);
                    if op1 == 0 {
                        op2 as usize
                    } else {
                        ip + 3
                    }
                }
                Opcode::LessThan(mode1, mode2) => {
                    let (op1, op2, op3) =
                        self.operands3(ip + 1, mode1, mode2, ParameterMode::Immediate);
                    self.0[op3 as usize] = if op1 < op2 { 1 } else { 0 };
                    ip + 4
                }
                Opcode::Equal(mode1, mode2) => {
                    let (op1, op2, op3) =
                        self.operands3(ip + 1, mode1, mode2, ParameterMode::Immediate);
                    self.0[op3 as usize] = if op1 == op2 { 1 } else { 0 };
                    ip + 4
                }
                Opcode::Halt => {
                    self.1 = ip;
                    halted = true;
                    break;
                },
            };
        }
        Ok((output, halted))
    }
}

struct PhaseSettings([i32; AMPLIFIERS], [usize; AMPLIFIERS], usize);

impl PhaseSettings {
    fn permutations() -> PhaseSettings {
        PhaseSettings([5, 6, 7, 8, 9], [0; AMPLIFIERS], AMPLIFIERS)
    }
}

impl Iterator for PhaseSettings {
    type Item = [i32; AMPLIFIERS];

    // Heap's algorithm for permutations
    // Basically copied from the non-recursive implementation in the Wikipedia article.
    fn next(&mut self) -> Option<Self::Item> {
        if self.2 == AMPLIFIERS {
            self.2 = 0;
            return Some(self.0);
        }

        while self.2 < AMPLIFIERS {
            if self.1[self.2] < self.2 {
                if (self.2 & 1) == 0 {
                    self.0.swap(0, self.2);
                } else {
                    self.0.swap(self.1[self.2], self.2);
                }
                self.1[self.2] += 1;
                self.2 = 0;
                return Some(self.0);
            } else {
                self.1[self.2] = 0;
                self.2 += 1;
            }
        }
        None
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let file = File::open("input.txt")?;
    let reader = BufReader::new(file);
    let program = Intcode::load(reader)?;
    let mut input = vec![0i32; 1];
    let mut best = 0i32;
    for phase_settings in PhaseSettings::permutations() {
        let mut programs = [
            program.clone(),
            program.clone(),
            program.clone(),
            program.clone(),
            program.clone(),
        ];

        // prime programs with phase settings
        for i in 0..AMPLIFIERS {
            input[0] = phase_settings[i];
            programs[i].run(&input)?;
        }

        let mut final_output = [0i32; AMPLIFIERS];
        let mut halted = [false; AMPLIFIERS];
        let mut num_halted = 0;
        input[0] = 0;

        while num_halted < AMPLIFIERS {
            for i in 0..AMPLIFIERS {
                if halted[i] {
                    input[0] = final_output[i];
                } else {
                    let (output, done) = programs[i].run(&input)?;
                    final_output[i] = output[0];
                    if done {
                        halted[i] = true;
                        num_halted += 1;
                    }
                    input[0] = output[0];
                }
            }
        }

        println!(
            "Output for {}{}{}{}{}: {}",
            phase_settings[0],
            phase_settings[1],
            phase_settings[2],
            phase_settings[3],
            phase_settings[4],
            final_output[4]
        );

        if final_output[4] > best {
            best = final_output[4];
        }
    }
    println!("Best: {}", best);

    Ok(())
}

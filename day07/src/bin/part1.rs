//! --- Day 7: Amplification Circuit ---
//!
//! Based on the navigational maps, you're going to need to send more power to your ship's
//! thrusters to reach Santa in time. To do this, you'll need to configure a series of amplifiers
//! already installed on the ship.
//!
//! There are five amplifiers connected in series; each one receives an input signal and produces
//! an output signal. They are connected such that the first amplifier's output leads to the second
//! amplifier's input, the second amplifier's output leads to the third amplifier's input, and so
//! on. The first amplifier's input value is 0, and the last amplifier's output leads to your
//! ship's thrusters.
//!
//!     O-------O  O-------O  O-------O  O-------O  O-------O
//! 0 ->| Amp A |->| Amp B |->| Amp C |->| Amp D |->| Amp E |-> (to thrusters)
//!     O-------O  O-------O  O-------O  O-------O  O-------O
//!
//! The Elves have sent you some Amplifier Controller Software (your puzzle input), a program that
//! should run on your existing Intcode computer. Each amplifier will need to run a copy of the
//! program.
//!
//! When a copy of the program starts running on an amplifier, it will first use an input
//! instruction to ask the amplifier for its current phase setting (an integer from 0 to 4). Each
//! phase setting is used exactly once, but the Elves can't remember which amplifier needs which
//! phase setting.
//!
//! The program will then call another input instruction to get the amplifier's input signal,
//! compute the correct output signal, and supply it back to the amplifier with an output
//! instruction. (If the amplifier has not yet received an input signal, it waits until one
//! arrives.)
//!
//! Your job is to find the largest output signal that can be sent to the thrusters by trying every
//! possible combination of phase settings on the amplifiers. Make sure that memory is not shared
//! or reused between copies of the program.
//!
//! For example, suppose you want to try the phase setting sequence 3,1,2,4,0, which would mean
//! setting amplifier A to phase setting 3, amplifier B to setting 1, C to 2, D to 4, and E to 0.
//! Then, you could determine the output signal that gets sent from amplifier E to the thrusters
//! with the following steps:
//!
//!   - Start the copy of the amplifier controller software that will run on amplifier A. At its
//!     first input instruction, provide it the amplifier's phase setting, 3. At its second input
//!     instruction, provide it the input signal, 0. After some calculations, it will use an output
//!     instruction to indicate the amplifier's output signal.
//!   - Start the software for amplifier B. Provide it the phase setting (1) and then whatever
//!     output signal was produced from amplifier A. It will then produce a new output signal
//!     destined for amplifier C.
//!   - Start the software for amplifier C, provide the phase setting (2) and the value from
//!     amplifier B, then collect its output signal.
//!   - Run amplifier D's software, provide the phase setting (4) and input value, and collect its
//!     output signal.
//!   - Run amplifier E's software, provide the phase setting (0) and input value, and collect its
//!     output signal.
//!
//! The final output signal from amplifier E would be sent to the thrusters. However, this phase
//! setting sequence may not have been the best one; another sequence might have sent a higher
//! signal to the thrusters.
//!
//! Here are some example programs:
//!
//!     Max thruster signal 43210 (from phase setting sequence 4,3,2,1,0):
//!
//!     3,15,3,16,1002,16,10,16,1,16,15,15,4,15,99,0,0
//!
//!     Max thruster signal 54321 (from phase setting sequence 0,1,2,3,4):
//!
//!     3,23,3,24,1002,24,10,24,1002,23,-1,23,
//!     101,5,23,23,1,24,23,23,4,23,99,0,0
//!
//!     Max thruster signal 65210 (from phase setting sequence 1,0,4,3,2):
//!
//!     3,31,3,32,1002,32,10,32,1001,31,-2,31,1007,31,0,33,
//!     1002,33,7,33,1,33,31,31,1,32,31,31,4,31,99,0,0,0
//!
//! Try every combination of phase settings on the amplifiers. What is the highest signal that can
//! be sent to the thrusters?

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
struct Intcode(Vec<i32>);

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
        Ok(Intcode(program))
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

    fn run(&mut self, input: &Vec<i32>) -> Result<Vec<i32>, Box<dyn Error>> {
        let mut output: Vec<i32> = Vec::new();
        let mut inputp = 0;
        let mut ip = 0;
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
                Opcode::Halt => break,
            };
        }
        Ok(output)
    }
}

struct PhaseSettings([i32; AMPLIFIERS], [usize; AMPLIFIERS], usize);

impl PhaseSettings {
    fn permutations() -> PhaseSettings {
        PhaseSettings([0, 1, 2, 3, 4], [0; AMPLIFIERS], AMPLIFIERS)
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
    let mut input = vec![0i32; 2];
    let mut best = 0i32;
    for phase_settings in PhaseSettings::permutations() {
        input[1] = 0;
        for i in 0..AMPLIFIERS {
            let mut program = program.clone();
            input[0] = phase_settings[i];

            let output = program.run(&input)?;
            input[1] = output[0];
        }
        println!(
            "Output for {}{}{}{}{}: {}",
            phase_settings[0],
            phase_settings[1],
            phase_settings[2],
            phase_settings[3],
            phase_settings[4],
            input[1]
        );

        if input[1] > best {
            best = input[1];
        }
    }
    println!("Best: {}", best);

    Ok(())
}

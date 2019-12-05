//! --- Part Two ---
//!
//! The air conditioner comes online! Its cold air feels good for a while, but then the TEST alarms
//! start to go off. Since the air conditioner can't vent its heat anywhere but back into the
//! spacecraft, it's actually making the air inside the ship warmer.
//!
//! Instead, you'll need to use the TEST to extend the thermal radiators. Fortunately, the
//! diagnostic program (your puzzle input) is already equipped for this. Unfortunately, your
//! Intcode computer is not.
//!
//! Your computer is only missing a few opcodes:
//!
//!   - Opcode 5 is jump-if-true: if the first parameter is non-zero, it sets the instruction
//!     pointer to the value from the second parameter. Otherwise, it does nothing.
//!   - Opcode 6 is jump-if-false: if the first parameter is zero, it sets the instruction pointer
//!     to the value from the second parameter. Otherwise, it does nothing.
//!   - Opcode 7 is less than: if the first parameter is less than the second parameter, it stores
//!     1 in the position given by the third parameter. Otherwise, it stores 0.
//!   - Opcode 8 is equals: if the first parameter is equal to the second parameter, it stores 1 in
//!     the position given by the third parameter. Otherwise, it stores 0.
//!
//! Like all instructions, these instructions need to support parameter modes as described above.
//!
//! Normally, after an instruction is finished, the instruction pointer increases by the number of
//! values in that instruction. However, if the instruction modifies the instruction pointer, that
//! value is used and the instruction pointer is not automatically increased.
//!
//! For example, here are several programs that take one input, compare it to the value 8, and then
//! produce one output:
//!
//!   - 3,9,8,9,10,9,4,9,99,-1,8 - Using position mode, consider whether the input is equal to 8;
//!     output 1 (if it is) or 0 (if it is not).
//!   - 3,9,7,9,10,9,4,9,99,-1,8 - Using position mode, consider whether the input is less than 8;
//!     output 1 (if it is) or 0 (if it is not).
//!   - 3,3,1108,-1,8,3,4,3,99 - Using immediate mode, consider whether the input is equal to 8;
//!     output 1 (if it is) or 0 (if it is not).
//!   - 3,3,1107,-1,8,3,4,3,99 - Using immediate mode, consider whether the input is less than 8;
//!     output 1 (if it is) or 0 (if it is not).
//!
//! Here are some jump tests that take an input, then output 0 if the input was zero or 1 if the
//! input was non-zero:
//!
//!   - 3,12,6,12,15,1,13,14,13,4,13,99,-1,0,1,9 (using position mode)
//!   - 3,3,1105,-1,9,1101,0,0,12,4,12,99,1 (using immediate mode)
//!
//! Here's a larger example:
//!
//! 3,21,1008,21,8,20,1005,20,22,107,8,21,20,1006,20,31,
//! 1106,0,36,98,0,0,1002,21,125,20,4,20,1105,1,46,104,
//! 999,1105,1,46,1101,1000,1,20,4,20,1105,1,46,98,99
//!
//! The above example program uses an input instruction to ask for a single number. The program
//! will then output 999 if the input value is below 8, output 1000 if the input value is equal to
//! 8, or output 1001 if the input value is greater than 8.
//!
//! This time, when the TEST diagnostic program runs its input instruction to get the ID of the
//! system to test, provide it 5, the ID for the ship's thermal radiator controller. This
//! diagnostic test suite only outputs one number, the diagnostic code.
//!
//! What is the diagnostic code for system ID 5?

use std::convert::{TryFrom, TryInto};
use std::error::Error;
use std::fmt::{self, Display};
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::str;

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
            },
            4 => {
                let mode = (code / 100).try_into().map_err(|_| InvalidOpcode(code))?;
                Ok(Opcode::Output(mode))
            },
            5 => {
                let (mode1, mode2) =
                    ParameterMode::modes2(code / 100).map_err(|_| InvalidOpcode(code))?;
                Ok(Opcode::JumpIfTrue(mode1, mode2))
            },
            6 => {
                let (mode1, mode2) =
                    ParameterMode::modes2(code / 100).map_err(|_| InvalidOpcode(code))?;
                Ok(Opcode::JumpIfFalse(mode1, mode2))
            },
            7 => {
                let (mode1, mode2) =
                    ParameterMode::modes2(code / 100).map_err(|_| InvalidOpcode(code))?;
                Ok(Opcode::LessThan(mode1, mode2))
            },
            8 => {
                let (mode1, mode2) =
                    ParameterMode::modes2(code / 100).map_err(|_| InvalidOpcode(code))?;
                Ok(Opcode::Equal(mode1, mode2))
            },
            99 => Ok(Opcode::Halt),
            _ => Err(InvalidOpcode(code)),
        }
    }
}

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

    fn operands2(
        &self,
        sp: usize,
        mode1: ParameterMode,
        mode2: ParameterMode,
    ) -> (i32, i32) {
        (
            self.operand(sp, mode1),
            self.operand(sp + 1, mode2),
        )
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

    fn run(&mut self, input: Vec<i32>) -> Result<i32, Box<dyn Error>> {
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
                },
                Opcode::Output(mode) => {
                    let op = self.operand(ip + 1, mode);
                    println!("Output: {}", op);
                    ip + 2
                },
                Opcode::JumpIfTrue(mode1, mode2) => {
                    let (op1, op2) = self.operands2(ip + 1, mode1, mode2);
                    if op1 != 0 { op2 as usize } else { ip + 3 }
                },
                Opcode::JumpIfFalse(mode1, mode2) => {
                    let (op1, op2) = self.operands2(ip + 1, mode1, mode2);
                    if op1 == 0 { op2 as usize } else { ip + 3 }
                },
                Opcode::LessThan(mode1, mode2) => {
                    let (op1, op2, op3) = self.operands3(ip + 1, mode1, mode2, ParameterMode::Immediate);
                    self.0[op3 as usize] = if op1 < op2 { 1 } else { 0 };
                    ip + 4
                },
                Opcode::Equal(mode1, mode2) => {
                    let (op1, op2, op3) = self.operands3(ip + 1, mode1, mode2, ParameterMode::Immediate);
                    self.0[op3 as usize] = if op1 == op2 { 1 } else { 0 };
                    ip + 4
                },
                Opcode::Halt => break,
            };
        }
        Ok(self.0[0])
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let file = File::open("input.txt")?;
    let reader = BufReader::new(file);
    let mut program = Intcode::load(reader)?;
    program.run(vec![5])?;

    Ok(())
}

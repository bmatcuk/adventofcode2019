//! --- Day 5: Sunny with a Chance of Asteroids ---
//!
//! You're starting to sweat as the ship makes its way toward Mercury. The Elves suggest that you
//! get the air conditioner working by upgrading your ship computer to support the Thermal
//! Environment Supervision Terminal.
//!
//! The Thermal Environment Supervision Terminal (TEST) starts by running a diagnostic program
//! (your puzzle input). The TEST diagnostic program will run on your existing Intcode computer
//! after a few modifications:
//!
//! First, you'll need to add two new instructions:
//!
//!   - Opcode 3 takes a single integer as input and saves it to the position given by its only
//!     parameter. For example, the instruction 3,50 would take an input value and store it at
//!     address 50.
//!   - Opcode 4 outputs the value of its only parameter. For example, the instruction 4,50 would
//!     output the value at address 50.
//!
//! Programs that use these instructions will come with documentation that explains what should be
//! connected to the input and output. The program 3,0,4,0,99 outputs whatever it gets as input,
//! then halts.
//!
//! Second, you'll need to add support for parameter modes:
//!
//! Each parameter of an instruction is handled based on its parameter mode. Right now, your ship
//! computer already understands parameter mode 0, position mode, which causes the parameter to be
//! interpreted as a position - if the parameter is 50, its value is the value stored at address 50
//! in memory. Until now, all parameters have been in position mode.
//!
//! Now, your ship computer will also need to handle parameters in mode 1, immediate mode. In
//! immediate mode, a parameter is interpreted as a value - if the parameter is 50, its value is
//! simply 50.
//!
//! Parameter modes are stored in the same value as the instruction's opcode. The opcode is a
//! two-digit number based only on the ones and tens digit of the value, that is, the opcode is the
//! rightmost two digits of the first value in an instruction. Parameter modes are single digits,
//! one per parameter, read right-to-left from the opcode: the first parameter's mode is in the
//! hundreds digit, the second parameter's mode is in the thousands digit, the third parameter's
//! mode is in the ten-thousands digit, and so on. Any missing modes are 0.
//!
//! For example, consider the program 1002,4,3,4,33.
//!
//! The first instruction, 1002,4,3,4, is a multiply instruction - the rightmost two digits of the
//! first value, 02, indicate opcode 2, multiplication. Then, going right to left, the parameter
//! modes are 0 (hundreds digit), 1 (thousands digit), and 0 (ten-thousands digit, not present and
//! therefore zero):
//!
//! ABCDE
//!  1002
//!
//! DE - two-digit opcode,      02 == opcode 2
//!  C - mode of 1st parameter,  0 == position mode
//!  B - mode of 2nd parameter,  1 == immediate mode
//!  A - mode of 3rd parameter,  0 == position mode,
//!                                   omitted due to being a leading zero
//!
//! This instruction multiplies its first two parameters. The first parameter, 4 in position mode,
//! works like it did before - its value is the value stored at address 4 (33). The second
//! parameter, 3 in immediate mode, simply has value 3. The result of this operation, 33 * 3 = 99,
//! is written according to the third parameter, 4 in position mode, which also works like it did
//! before - 99 is written to address 4.
//!
//! Parameters that an instruction writes to will never be in immediate mode.
//!
//! Finally, some notes:
//!
//!   - It is important to remember that the instruction pointer should increase by the number of
//!     values in the instruction after the instruction finishes. Because of the new instructions,
//!     this amount is no longer always 4.
//!   - Integers can be negative: 1101,100,-1,4,0 is a valid program (find 100 + -1, store the
//!     result in position 4).
//!
//! The TEST diagnostic program will start by requesting from the user the ID of the system to test
//! by running an input instruction - provide it 1, the ID for the ship's air conditioner unit.
//!
//! It will then perform a series of diagnostic tests confirming that various parts of the Intcode
//! computer, like parameter modes, function correctly. For each test, it will run an output
//! instruction indicating how far the result of the test was from the expected value, where 0
//! means the test was successful. Non-zero outputs mean that a function is not working correctly;
//! check the instructions that were run before the output instruction to see which one failed.
//!
//! Finally, the program will output a diagnostic code and immediately halt. This final output
//! isn't an error; an output followed immediately by a halt means the program finished. If all
//! outputs were zero except the diagnostic code, the diagnostic program ran successfully.
//!
//! After providing 1 to the only input instruction and passing all the tests, what diagnostic code
//! does the program produce?

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
            let advance = match opcode {
                Opcode::Add(mode1, mode2) => {
                    let (op1, op2, op3) =
                        self.operands3(ip + 1, mode1, mode2, ParameterMode::Immediate);
                    self.0[op3 as usize] = op1 + op2;
                    4
                }
                Opcode::Multiply(mode1, mode2) => {
                    let (op1, op2, op3) =
                        self.operands3(ip + 1, mode1, mode2, ParameterMode::Immediate);
                    self.0[op3 as usize] = op1 * op2;
                    4
                }
                Opcode::Input(_mode) => {
                    // not sure what to do if mode is not immediate?
                    let op = self.operand(ip + 1, ParameterMode::Immediate) as usize;
                    self.0[op] = input[inputp];
                    inputp += 1;
                    2
                },
                Opcode::Output(mode) => {
                    let op = self.operand(ip + 1, mode);
                    println!("Output: {}", op);
                    2
                }
                Opcode::Halt => break,
            };
            ip += advance;
        }
        Ok(self.0[0])
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let file = File::open("input.txt")?;
    let reader = BufReader::new(file);
    let mut program = Intcode::load(reader)?;
    program.run(vec![1])?;

    Ok(())
}

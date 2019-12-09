//! --- Part Two ---
//!
//! You now have a complete Intcode computer.
//!
//! Finally, you can lock on to the Ceres distress signal! You just need to boost your sensors
//! using the BOOST program.
//!
//! The program runs in sensor boost mode by providing the input instruction the value 2. Once run,
//! it will boost the sensors automatically, but it might take a few seconds to complete the
//! operation on slower hardware. In sensor boost mode, the program will output a single value: the
//! coordinates of the distress signal.
//!
//! Run the BOOST program in sensor boost mode. What are the coordinates of the distress signal?

use std::clone::Clone;
use std::convert::{TryFrom, TryInto};
use std::error::Error;
use std::fmt::{self, Display};
use std::fs::File;
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

fn main() -> Result<(), Box<dyn Error>> {
    let file = File::open("input.txt")?;
    let reader = BufReader::new(file);
    let mut program = Intcode::load(reader)?;
    let input = vec![2];
    let (output, _) = program.run(&input)?;
    for out in output {
        println!("Output: {}", out);
    }
    Ok(())
}

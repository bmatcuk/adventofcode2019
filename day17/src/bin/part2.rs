//! --- Part Two ---
//!
//! Now for the tricky part: notifying all the other robots about the solar flare. The vacuum robot
//! can do this automatically if it gets into range of a robot. However, you can't see the other
//! robots on the camera, so you need to be thorough instead: you need to make the vacuum robot
//! visit every part of the scaffold at least once.
//!
//! The vacuum robot normally wanders randomly, but there isn't time for that today. Instead, you
//! can override its movement logic with new rules.
//!
//! Force the vacuum robot to wake up by changing the value in your ASCII program at address 0 from
//! 1 to 2. When you do this, you will be automatically prompted for the new movement rules that
//! the vacuum robot should use. The ASCII program will use input instructions to receive them, but
//! they need to be provided as ASCII code; end each line of logic with a single newline, ASCII
//! code 10.
//!
//! First, you will be prompted for the main movement routine. The main routine may only call the
//! movement functions: A, B, or C. Supply the movement functions to use as ASCII text, separating
//! them with commas (,, ASCII code 44), and ending the list with a newline (ASCII code 10). For
//! example, to call A twice, then alternate between B and C three times, provide the string
//! A,A,B,C,B,C,B,C and then a newline.
//!
//! Then, you will be prompted for each movement function. Movement functions may use L to turn
//! left, R to turn right, or a number to move forward that many units. Movement functions may not
//! call other movement functions. Again, separate the actions with commas and end the list with a
//! newline. For example, to move forward 10 units, turn left, move forward 8 units, turn right,
//! and finally move forward 6 units, provide the string 10,L,8,R,6 and then a newline.
//!
//! Finally, you will be asked whether you want to see a continuous video feed; provide either y or
//! n and a newline. Enabling the continuous video feed can help you see what's going on, but it
//! also requires a significant amount of processing power, and may even cause your Intcode
//! computer to overheat.
//!
//! Due to the limited amount of memory in the vacuum robot, the ASCII definitions of the main
//! routine and the movement functions may each contain at most 20 characters, not counting the
//! newline.
//!
//! For example, consider the following camera feed:
//!
//! #######...#####
//! #.....#...#...#
//! #.....#...#...#
//! ......#...#...#
//! ......#...###.#
//! ......#.....#.#
//! ^########...#.#
//! ......#.#...#.#
//! ......#########
//! ........#...#..
//! ....#########..
//! ....#...#......
//! ....#...#......
//! ....#...#......
//! ....#####......
//!
//! In order for the vacuum robot to visit every part of the scaffold at least once, one path it
//! could take is:
//!
//! R,8,R,8,R,4,R,4,R,8,L,6,L,2,R,4,R,4,R,8,R,8,R,8,L,6,L,2
//!
//! Without the memory limit, you could just supply this whole string to function A and have the
//! main routine call A once. However, you'll need to split it into smaller parts.
//!
//! One approach is:
//!
//!     Main routine: A,B,C,B,A,C
//!     (ASCII input: 65, 44, 66, 44, 67, 44, 66, 44, 65, 44, 67, 10)
//!     Function A:   R,8,R,8
//!     (ASCII input: 82, 44, 56, 44, 82, 44, 56, 10)
//!     Function B:   R,4,R,4,R,8
//!     (ASCII input: 82, 44, 52, 44, 82, 44, 52, 44, 82, 44, 56, 10)
//!     Function C:   L,6,L,2
//!     (ASCII input: 76, 44, 54, 44, 76, 44, 50, 10)
//!
//! Visually, this would break the desired path into the following parts:
//!
//! A,        B,            C,        B,            A,        C
//! R,8,R,8,  R,4,R,4,R,8,  L,6,L,2,  R,4,R,4,R,8,  R,8,R,8,  L,6,L,2
//!
//! CCCCCCA...BBBBB
//! C.....A...B...B
//! C.....A...B...B
//! ......A...B...B
//! ......A...CCC.B
//! ......A.....C.B
//! ^AAAAAAAA...C.B
//! ......A.A...C.B
//! ......AAAAAA#AB
//! ........A...C..
//! ....BBBB#BBBB..
//! ....B...A......
//! ....B...A......
//! ....B...A......
//! ....BBBBA......
//!
//! Of course, the scaffolding outside your ship is much more complex.
//!
//! As the vacuum robot finds other robots and notifies them of the impending solar flare, it also
//! can't help but leave them squeaky clean, collecting any space dust it finds. Once it finishes
//! the programmed set of movements, assuming it hasn't drifted off into space, the cleaning robot
//! will return to its docking station and report the amount of space dust it collected as a large,
//! non-ASCII value in a single output instruction.
//!
//! After visiting every part of the scaffold at least once, how much dust does the vacuum robot
//! report it has collected?

use std::clone::Clone;
use std::cmp::PartialEq;
use std::convert::{From, TryFrom, TryInto};
use std::error::Error;
use std::fmt::{self, Display, Write};
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

fn left(
    map: &Vec<Vec<char>>,
    heading: char,
    x: usize,
    y: usize,
    width: usize,
    height: usize,
) -> Option<(usize, usize, char)> {
    match heading {
        '^' => {
            if x > 0 && map[y][x - 1] == '#' {
                Some((x - 1, y, '<'))
            } else {
                None
            }
        }
        '>' => {
            if y > 0 && map[y - 1][x] == '#' {
                Some((x, y - 1, '^'))
            } else {
                None
            }
        }
        'v' => {
            if x + 1 < width && map[y][x + 1] == '#' {
                Some((x + 1, y, '>'))
            } else {
                None
            }
        }
        '<' => {
            if y + 1 < height && map[y + 1][x] == '#' {
                Some((x, y + 1, 'v'))
            } else {
                None
            }
        }
        _ => unreachable!(),
    }
}

fn right(
    map: &Vec<Vec<char>>,
    heading: char,
    x: usize,
    y: usize,
    width: usize,
    height: usize,
) -> Option<(usize, usize, char)> {
    match heading {
        '^' => {
            if x + 1 < width && map[y][x + 1] == '#' {
                Some((x + 1, y, '>'))
            } else {
                None
            }
        }
        '>' => {
            if y + 1 < height && map[y + 1][x] == '#' {
                Some((x, y + 1, 'v'))
            } else {
                None
            }
        }
        'v' => {
            if x > 0 && map[y][x - 1] == '#' {
                Some((x - 1, y, '<'))
            } else {
                None
            }
        }
        '<' => {
            if y > 0 && map[y - 1][x] == '#' {
                Some((x, y - 1, '^'))
            } else {
                None
            }
        }
        _ => unreachable!(),
    }
}

fn forward(
    map: &Vec<Vec<char>>,
    heading: char,
    x: usize,
    y: usize,
    width: usize,
    height: usize,
) -> Option<(usize, usize)> {
    match heading {
        '^' => {
            if y > 0 && map[y - 1][x] == '#' {
                Some((x, y - 1))
            } else {
                None
            }
        }
        '>' => {
            if x + 1 < width && map[y][x + 1] == '#' {
                Some((x + 1, y))
            } else {
                None
            }
        }
        'v' => {
            if y + 1 < height && map[y + 1][x] == '#' {
                Some((x, y + 1))
            } else {
                None
            }
        }
        '<' => {
            if x > 0 && map[y][x - 1] == '#' {
                Some((x - 1, y))
            } else {
                None
            }
        }
        _ => unreachable!(),
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let file = File::open("input.txt")?;
    let reader = BufReader::new(file);
    let mut observer = Intcode::load(reader)?;
    let mut program = observer.clone();

    // first, we'll build the map so we can determine the route to get from start to finish
    let (map, _) = observer.run(&vec![])?;
    let (mut map, robotidx, mut heading) =
        map.iter().map(|&i| char::from(i as u8)).enumerate().fold(
            (vec![vec![]], 0, '^'),
            |(mut acc, robotidx, heading), (idx, c)| match c {
                '\n' => {
                    acc.push(vec![]);
                    (acc, robotidx, heading)
                }
                c => {
                    let accidx = acc.len() - 1;
                    acc[accidx].push(c);
                    if robotidx == 0 && (c == '^' || c == '>' || c == 'v' || c == '<') {
                        (acc, idx, c)
                    } else {
                        (acc, robotidx, heading)
                    }
                }
            },
        );
    for i in (0..map.len()).rev() {
        if map[i].len() > 0 {
            map.truncate(i + 1);
            break;
        }
    }

    // plan our route - width + 1 in robotx/y calcs to account for newline chars
    let height = map.len();
    let width = map[0].len();
    let mut moved = 0;
    let mut route = String::new();
    let (mut robotx, mut roboty) = (robotidx % (width + 1), robotidx / (width + 1));
    println!(
        "Starting position: idx {} = {},{}; heading = {}",
        robotidx, robotx, roboty, heading
    );
    loop {
        if let Some((x, y)) = forward(&map, heading, robotx, roboty, width, height) {
            robotx = x;
            roboty = y;
        } else if let Some((x, y, newheading)) = left(&map, heading, robotx, roboty, width, height)
        {
            robotx = x;
            roboty = y;
            heading = newheading;
            if moved > 0 {
                write!(route, "{},L,", moved)?;
                moved = 0;
            } else {
                write!(route, "L,")?;
            }
        } else if let Some((x, y, newheading)) = right(&map, heading, robotx, roboty, width, height)
        {
            robotx = x;
            roboty = y;
            heading = newheading;
            if moved > 0 {
                write!(route, "{},R,", moved)?;
                moved = 0;
            } else {
                write!(route, "R,")?;
            }
        } else {
            // must have reached the end
            break;
        }
        moved += 1;
    }
    if moved > 0 {
        write!(route, "{}", moved)?;
    } else {
        route.pop();
    }
    println!("Route: {}", route);

    // now find the longest repeating, non-overlapping substring
    let broute = route.as_bytes();
    let mut lcsre = vec![vec![0; broute.len() + 1]; broute.len() + 1];
    let mut longest_len = 0;
    let mut longest_idx = 0;
    for i in 1..(broute.len() + 1) {
        for j in (i + 1)..(broute.len() + 1) {
            if broute[i - 1] == broute[j - 1] && lcsre[i - 1][j - 1] < (j - i) {
                lcsre[i][j] = lcsre[i - 1][j - 1] + 1;
                if lcsre[i][j] > longest_len {
                    longest_len = lcsre[i][j];
                    longest_idx = longest_idx.max(i);
                }
            }
        }
    }
    println!(
        "Longest Substr {}: {}",
        longest_len,
        route[(longest_idx - longest_len)..longest_idx].to_owned()
    );

    // From here, I'm working manually because I didn't feel like figuring out how to automate
    // this. The route produced above is:
    //   R,6,R,6,R,8,L,10,L,4,R,6,L,10,R,8,R,6,L,10,R,8,R,6,R,6,R,8,L,10,L,4,L,4,L,12,R,6,L,10,R,6,
    //   R,6,R,8,L,10,L,4,L,4,L,12,R,6,L,10,R,6,R,6,R,8,L,10,L,4,L,4,L,12,R,6,L,10,R,6,L,10,R,8
    //
    // And the longest substring is 44 characters:
    //   ,R,6,R,6,R,8,L,10,L,4,L,4,L,12,R,6,L,10,R,6,
    let main = "A,B,B,A,C,A,C,A,C,B\n";
    let func_a = "R,6,R,6,R,8,L,10,L,4\n";
    let func_b = "R,6,L,10,R,8\n";
    let func_c = "L,4,L,12,R,6,L,10\n";
    let continuous = "n\n";
    let input: Vec<i64> = main
        .bytes()
        .chain(func_a.bytes())
        .chain(func_b.bytes())
        .chain(func_c.bytes())
        .chain(continuous.bytes())
        .map(|c| c as i64)
        .collect();
    program.code[0] = 2;

    // apparently the outut is the starting map, a print-out of main and the functions, then the
    // ending map, a blank line, and finally the number of dust... so, what we really care about is
    // the last item in output.
    let (output, _) = program.run(&input)?;
    println!("Dust: {}", output[output.len() - 1]);

    Ok(())
}

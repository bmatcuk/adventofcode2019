//! --- Part Two ---
//!
//! You quickly repair the oxygen system; oxygen gradually fills the area.
//!
//! Oxygen starts in the location containing the repaired oxygen system. It takes one minute for
//! oxygen to spread to all open locations that are adjacent to a location that already contains
//! oxygen. Diagonal locations are not adjacent.
//!
//! In the example above, suppose you've used the droid to explore the area fully and have the
//! following map (where locations that currently contain oxygen are marked O):
//!
//!  ##
//! #..##
//! #.#..#
//! #.O.#
//!  ###
//!
//! Initially, the only location which contains oxygen is the location of the repaired oxygen
//! system. However, after one minute, the oxygen spreads to all open (.) locations that are
//! adjacent to a location containing oxygen:
//!
//!  ##
//! #..##
//! #.#..#
//! #OOO#
//!  ###
//!
//! After a total of two minutes, the map looks like this:
//!
//!  ##
//! #..##
//! #O#O.#
//! #OOO#
//!  ###
//!
//! After a total of three minutes:
//!
//!  ##
//! #O.##
//! #O#OO#
//! #OOO#
//!  ###
//!
//! And finally, the whole region is full of oxygen after a total of four minutes:
//!
//!  ##
//! #OO##
//! #O#OO#
//! #OOO#
//!  ###
//!
//! So, in this example, all locations contain oxygen after 4 minutes.
//!
//! Use the repair droid to get a complete map of the area. How many minutes will it take to fill
//! with oxygen?

use std::clone::Clone;
use std::cmp::PartialEq;
use std::convert::{From, TryFrom, TryInto};
use std::error::Error;
use std::fmt::{self, Display};
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::iter::successors;
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

#[derive(Clone, Copy, PartialEq)]
enum Tile {
    Wall,
    Clear(usize),
    Oxygen(usize),
    Unknown,
}

impl Tile {
    fn set_distance(&mut self, distance: usize) -> bool {
        match self {
            Tile::Clear(d) if *d == 0 || *d >= distance => {
                *self = Tile::Clear(distance);
                true
            }
            Tile::Oxygen(d) if *d == 0 || *d >= distance => {
                *self = Tile::Oxygen(distance);
                true
            }
            _ => false,
        }
    }
}

impl From<i64> for Tile {
    fn from(i: i64) -> Tile {
        match i {
            0 => Tile::Wall,
            1 => Tile::Clear(0),
            2 => Tile::Oxygen(0),
            _ => Tile::Unknown,
        }
    }
}

impl From<&Tile> for char {
    fn from(t: &Tile) -> char {
        match t {
            Tile::Wall => '█',
            Tile::Clear(_) => ' ',
            Tile::Oxygen(_) => 'O',
            Tile::Unknown => '?',
        }
    }
}

struct Board {
    board: Vec<Vec<Tile>>,
    minx: isize,
    maxx: isize,
    miny: isize,
    maxy: isize,
    oxygen: (isize, isize),
}

impl Board {
    fn new() -> Board {
        // 0,0 is the starting position. We're starting with a board that goes from -1 -> 1 in both
        // dimensions, so that means index 1,1 is 0,0. We'll grow the board as the program runs.
        let mut board = vec![vec![Tile::Unknown; 3]; 3];
        board[1][1] = Tile::Clear(0);

        Board {
            board,
            minx: -1,
            maxx: 1,
            miny: -1,
            maxy: 1,
            oxygen: (0, 0),
        }
    }

    fn get_tile(&self, x: isize, y: isize) -> Tile {
        if !(self.minx..=self.maxx).contains(&x) || !(self.miny..=self.maxy).contains(&y) {
            return Tile::Unknown;
        }

        let x = (x - self.minx) as usize;
        let y = (y - self.miny) as usize;
        self.board[y][x]
    }

    fn get_tile_mut(&mut self, x: isize, y: isize) -> Option<&mut Tile> {
        if !(self.minx..=self.maxx).contains(&x) || !(self.miny..=self.maxy).contains(&y) {
            return None;
        }

        let x = (x - self.minx) as usize;
        let y = (y - self.miny) as usize;
        Some(&mut self.board[y][x])
    }

    fn get_surroundings(&self, x: isize, y: isize) -> [Tile; 4] {
        [
            self.get_tile(x, y - 1),
            self.get_tile(x, y + 1),
            self.get_tile(x - 1, y),
            self.get_tile(x + 1, y),
        ]
    }

    fn grow_if_necessary(&mut self, x: isize, y: isize) {
        if !(self.minx..=self.maxx).contains(&x) {
            if x < self.minx {
                self.minx = x;
                self.board
                    .iter_mut()
                    .for_each(|row| row.insert(0, Tile::Unknown));
            } else {
                self.maxx = x;
                self.board
                    .iter_mut()
                    .for_each(|row| row.push(Tile::Unknown));
            }
        }
        if !(self.miny..=self.maxy).contains(&y) {
            let width = self.board[0].len();
            if y < self.miny {
                self.miny = y;
                self.board.insert(0, vec![Tile::Unknown; width]);
            } else {
                self.maxy = y;
                self.board.push(vec![Tile::Unknown; width]);
            }
        }
    }

    fn update_distances(&mut self, x: isize, y: isize, distance: usize) {
        match self.get_tile_mut(x, y) {
            Some(t) => {
                if t.set_distance(distance) {
                    self.update_distances(x, y - 1, distance + 1);
                    self.update_distances(x, y + 1, distance + 1);
                    self.update_distances(x - 1, y, distance + 1);
                    self.update_distances(x + 1, y, distance + 1);
                }
            }
            None => (),
        }
    }

    fn update(&mut self, x: isize, y: isize, outcome: Tile) {
        self.grow_if_necessary(x, y);
        if let Tile::Oxygen(_) = outcome {
            self.oxygen = (x, y);
        }

        let ux = (x - self.minx) as usize;
        let uy = (y - self.miny) as usize;
        self.board[uy][ux] = outcome;
    }
}

const NORTH: i64 = 1;
const SOUTH: i64 = 2;
const WEST: i64 = 3;
const EAST: i64 = 4;

fn turn_left(prev_move: i64) -> i64 {
    match prev_move {
        NORTH => WEST,
        SOUTH => EAST,
        WEST => SOUTH,
        EAST => NORTH,
        _ => unreachable!(),
    }
}

fn turn_right(prev_move: i64) -> i64 {
    match prev_move {
        NORTH => EAST,
        SOUTH => WEST,
        WEST => NORTH,
        EAST => SOUTH,
        _ => unreachable!(),
    }
}

fn backtrack(prev_move: i64) -> i64 {
    match prev_move {
        NORTH => SOUTH,
        SOUTH => NORTH,
        WEST => EAST,
        EAST => WEST,
        _ => unreachable!(),
    }
}

fn next_position(x: isize, y: isize, direction: i64, nextx: &mut isize, nexty: &mut isize) {
    match direction {
        NORTH => {
            *nextx = x;
            *nexty = y - 1;
        }
        SOUTH => {
            *nextx = x;
            *nexty = y + 1;
        }
        WEST => {
            *nextx = x - 1;
            *nexty = y;
        }
        EAST => {
            *nextx = x + 1;
            *nexty = y;
        }
        _ => unreachable!(),
    }
}

#[derive(Debug)]
enum History {
    Alternative(i64, isize, isize),
    Move(i64),
}

fn fill_next_move(
    input: &mut Vec<i64>,
    board: &Board,
    history: &mut Vec<History>,
    x: &mut isize,
    y: &mut isize,
    nextx: &mut isize,
    nexty: &mut isize,
) -> bool {
    input.clear();

    let history_idx = history.len().saturating_sub(1);
    let prev_move = if let Some(hist) = history.get_mut(history_idx) {
        match hist {
            History::Alternative(m, ax, ay) => {
                *x = *ax;
                *y = *ay;
                next_position(*x, *y, *m, nextx, nexty);
                input.push(*m);
                *hist = History::Move(*m);
                return true;
            }
            History::Move(m) => *m,
        }
    } else {
        SOUTH
    };

    // left is inherently evil so we prefer to move left and keep walls to our right
    let surroundings = board.get_surroundings(*x, *y);
    let preferred_move = if surroundings[(prev_move - 1) as usize] == Tile::Wall {
        // we hit a wall, turn left
        turn_left(prev_move)
    } else {
        // hug the wall to the right
        turn_right(prev_move)
    };

    // are there any unknown spots we can move to?
    let possible_moves: Vec<i64> = successors(Some(preferred_move), |&m| Some(turn_left(m)))
        .take(4)
        .filter_map(|m| match surroundings[(m - 1) as usize] {
            Tile::Unknown => Some(m),
            _ => None,
        })
        .collect();
    if possible_moves.len() > 0 {
        // track alternatives
        possible_moves[1..]
            .iter()
            .for_each(|&m| history.push(History::Alternative(m, *x, *y)));

        // and try the first path
        next_position(*x, *y, possible_moves[0], nextx, nexty);
        history.push(History::Move(possible_moves[0]));
        input.push(possible_moves[0]);
        return true;
    }

    // otherwise, we need to backtrack to the last alternative
    loop {
        match history.pop() {
            Some(History::Alternative(m, ax, ay)) => {
                history.push(History::Move(m));
                input.push(m);
                *x = ax;
                *y = ay;
                next_position(*x, *y, m, nextx, nexty);
                return true;
            }
            Some(History::Move(m)) => input.push(backtrack(m)),
            None => return false,
        }
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let file = File::open("input.txt")?;
    let reader = BufReader::new(file);
    let mut program = Intcode::load(reader)?;
    let mut board = Board::new();
    let mut history: Vec<History> = Vec::new();
    let mut input = Vec::new();
    let (mut x, mut y) = (0, 0);
    let (mut nextx, mut nexty) = (0, 0);
    loop {
        let (output, halt) = program.run(&input)?;
        if halt {
            println!("Program halted?");
            break;
        }

        if !output.is_empty() {
            let outcome = Tile::from(output[output.len() - 1]);
            if outcome == Tile::Wall {
                // remove last move, it failed
                history.pop();
            } else {
                x = nextx;
                y = nexty;
            }
            board.update(nextx, nexty, outcome);
        }

        if !fill_next_move(
            &mut input,
            &board,
            &mut history,
            &mut x,
            &mut y,
            &mut nextx,
            &mut nexty,
        ) {
            break;
        }
    }

    // calculate how long it'll take to fill with oxygen
    let (oxygen_x, oxygen_y) = board.oxygen;
    board.update_distances(oxygen_x, oxygen_y, 0);

    let result = board
        .board
        .iter()
        .flat_map(|row| row.iter())
        .filter_map(|t| match t {
            Tile::Clear(d) | Tile::Oxygen(d) => Some(d),
            _ => None,
        })
        .max()
        .unwrap();
    println!("Time: {}", result);

    Ok(())
}

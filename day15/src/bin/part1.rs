//! --- Day 15: Oxygen System ---
//!
//! Out here in deep space, many things can go wrong. Fortunately, many of those things have
//! indicator lights. Unfortunately, one of those lights is lit: the oxygen system for part of the
//! ship has failed!
//!
//! According to the readouts, the oxygen system must have failed days ago after a rupture in
//! oxygen tank two; that section of the ship was automatically sealed once oxygen levels went
//! dangerously low. A single remotely-operated repair droid is your only option for fixing the
//! oxygen system.
//!
//! The Elves' care package included an Intcode program (your puzzle input) that you can use to
//! remotely control the repair droid. By running that program, you can direct the repair droid to
//! the oxygen system and fix the problem.
//!
//! The remote control program executes the following steps in a loop forever:
//!
//!     Accept a movement command via an input instruction.
//!     Send the movement command to the repair droid.
//!     Wait for the repair droid to finish the movement operation.
//!     Report on the status of the repair droid via an output instruction.
//!
//! Only four movement commands are understood: north (1), south (2), west (3), and east (4). Any
//! other command is invalid. The movements differ in direction, but not in distance: in a long
//! enough east-west hallway, a series of commands like 4,4,4,4,3,3,3,3 would leave the repair
//! droid back where it started.
//!
//! The repair droid can reply with any of the following status codes:
//!
//!     0: The repair droid hit a wall. Its position has not changed.
//!     1: The repair droid has moved one step in the requested direction.
//!     2: The repair droid has moved one step in the requested direction; its new position is the location of the oxygen system.
//!
//! You don't know anything about the area around the repair droid, but you can figure it out by
//! watching the status codes.
//!
//! For example, we can draw the area using D for the droid, # for walls, . for locations the droid
//! can traverse, and empty space for unexplored locations. Then, the initial state looks like
//! this:
//!
//!
//!
//!    D
//!
//!
//!
//! To make the droid go north, send it 1. If it replies with 0, you know that location is a wall
//! and that the droid didn't move:
//!
//!
//!    #
//!    D
//!
//!
//!
//! To move east, send 4; a reply of 1 means the movement was successful:
//!
//!
//!    #
//!    .D
//!
//!
//!
//! Then, perhaps attempts to move north (1), south (2), and east (4) are all met with replies of
//! 0:
//!
//!
//!    ##
//!    .D#
//!     #
//!
//!
//! Now, you know the repair droid is in a dead end. Backtrack with 3 (which you already know will
//! get a reply of 1 because you already know that location is open):
//!
//!
//!    ##
//!    D.#
//!     #
//!
//!
//! Then, perhaps west (3) gets a reply of 0, south (2) gets a reply of 1, south again (2) gets a
//! reply of 0, and then west (3) gets a reply of 2:
//!
//!
//!    ##
//!   #..#
//!   D.#
//!    #
//!
//! Now, because of the reply of 2, you know you've found the oxygen system! In this example, it
//! was only 2 moves away from the repair droid's starting position.
//!
//! What is the fewest number of movement commands required to move the repair droid from its
//! starting position to the location of the oxygen system?

use std::clone::Clone;
use std::cmp::PartialEq;
use std::convert::{From, TryFrom, TryInto};
use std::error::Error;
use std::fmt::{self, Display};
use std::fs::File;
use std::io::{self, BufRead, BufReader, Read, Write};
use std::iter::successors;
use std::str;
use std::thread::sleep;
use std::time::Duration;

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
            Tile::Clear(d) if *d >= distance => {
                *self = Tile::Clear(distance);
                true
            }
            Tile::Oxygen(d) if *d >= distance => {
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

    fn distance_from_start(&self, x: isize, y: isize) -> usize {
        self.get_surroundings(x, y)
            .iter()
            .map(|t| match t {
                Tile::Clear(d) => *d + 1,
                Tile::Oxygen(d) => *d + 1,
                _ => std::usize::MAX,
            })
            .min()
            .unwrap()
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

        let distance = if outcome == Tile::Wall {
            0
        } else {
            self.distance_from_start(x, y)
        };

        let mut distances_need_update = false;
        let ux = (x - self.minx) as usize;
        let uy = (y - self.miny) as usize;
        self.board[uy][ux] = match outcome {
            Tile::Clear(_) => {
                if let Tile::Clear(d) = self.board[uy][ux] {
                    distances_need_update = d > distance;
                }
                Tile::Clear(distance)
            }
            Tile::Oxygen(_) => {
                if let Tile::Oxygen(d) = self.board[uy][ux] {
                    distances_need_update = d > distance;
                }
                Tile::Oxygen(distance)
            }
            _ => outcome,
        };

        if distances_need_update {
            self.update_distances(x, y, distance);
        }
    }

    fn draw(&self, x: isize, y: isize) {
        let x = (x - self.minx) as usize;
        let y = (y - self.miny) as usize;
        println!("\x1b[H");
        for (j, row) in self.board.iter().enumerate() {
            println!(
                "{}",
                row.iter()
                    .enumerate()
                    .map(|(i, t)| if i == x && j == y {
                        '⚇'
                    } else if i == -self.minx as usize && j == -self.miny as usize {
                        'S'
                    } else {
                        t.into()
                    })
                    .collect::<String>()
            );
        }
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
    println!("\x1b[?1049h");
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
        board.draw(x, y);
        // for history in history.iter().rev().take(10) {
        //     println!("{:?}", history);
        // }

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
        println!("xy: {},{}", x, y);
        println!("next: {},{}", nextx, nexty);

        sleep(Duration::from_millis(5));

        // let mut stdout = io::stdout();
        // let mut stdin = io::stdin();
        // write!(stdout, "\nPress any key to continue...\n")?;
        // stdout.flush()?;
        // stdin.read(&mut [0u8])?;
    }

    let mut stdout = io::stdout();
    let mut stdin = io::stdin();
    write!(stdout, "\nPress any key to continue...\n")?;
    stdout.flush()?;
    stdin.read(&mut [0u8])?;
    println!("\x1b[?1049l");

    let (oxygen_x, oxygen_y) = board.oxygen;
    if let Tile::Oxygen(distance) = board.get_tile(oxygen_x, oxygen_y) {
        println!("Moves: {}", distance);
    }

    Ok(())
}

use std::str::FromStr;

use lazy_static::lazy_static;
use regex::Regex;

use super::errors::AdventError;

pub fn solve(input: &str) -> Result<(), AdventError> {
    let instructions = input.split_whitespace().map(Instruction::from_str).collect::<Result<Vec<_>, AdventError>>()?;
    let mut ship_state = ShipState::new();
    for instruction in instructions {
        ship_state.apply_instruction(instruction);
    }
    let result = ship_state.manhattan_distance();
    println!("{}", result);
    Ok(())
}

struct ShipState {
    bearing: Bearing,
    x: isize,
    y: isize,
}

impl ShipState {
    pub fn new() -> Self {
        ShipState { bearing: Bearing::East, x: 0, y: 0 }
    }

    pub fn apply_instruction(&mut self, instruction: Instruction) {
        match instruction {
            Instruction::North(n) => self.y -= n,
            Instruction::South(n) => self.y += n,
            Instruction::East(n) => self.x += n,
            Instruction::West(n) => self.x -= n,
            Instruction::Left(n) => self.rotate_right(-n),
            Instruction::Right(n) => self.rotate_right(n),
            Instruction::Forward(n) => self.apply_instruction(Instruction::from_bearing_distance(self.bearing, n))
        }
    }

    pub fn manhattan_distance(&self) -> usize {
        self.x.abs() as usize + self.y.abs() as usize
    }

    fn rotate_right(&mut self, mut degrees: isize) {
        while degrees < 0 { degrees += 360 }
        assert_eq!(degrees % 90, 0);
        for _ in 0..(degrees / 90) {
            self.rotate_90degs_right();
        }
    }

    fn rotate_90degs_right(&mut self) {
        match self.bearing {
            Bearing::North => self.bearing = Bearing::East,
            Bearing::East => self.bearing = Bearing::South,
            Bearing::South => self.bearing = Bearing::West,
            Bearing::West => self.bearing = Bearing::North,
        }

    }
}

#[derive(Clone, Copy)]
pub enum Bearing {
    North,
    East,
    South,
    West,
}

pub enum Instruction {
    North(isize),
    South(isize),
    East(isize),
    West(isize),
    Left(isize),
    Right(isize),
    Forward(isize),
}

impl Instruction {
    pub fn from_bearing_distance(bearing: Bearing, distance: isize) -> Self {
        match bearing {
            Bearing::North => Self::North(distance),
            Bearing::East => Self::East(distance),
            Bearing::South => Self::South(distance),
            Bearing::West => Self::West(distance),
        }
    }
}

impl FromStr for Instruction {
    type Err = AdventError;
    
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        lazy_static! {
            static ref RE: Regex = Regex::new(r"(?P<command>[NSEWLRF])(?P<argument>\d+)").unwrap();
        }
        let captures = RE.captures(s).unwrap();
        let command_char = captures.name("command").unwrap().as_str().chars().nth(0).unwrap();
        let argument: isize = captures.name("argument").unwrap().as_str().parse()?;
        let result = match command_char {
            'N' => Self::North(argument),
            'S' => Self::South(argument),
            'E' => Self::East(argument),
            'W' => Self::West(argument),
            'L' => Self::Left(argument),
            'R' => Self::Right(argument),
            'F' => Self::Forward(argument),
            _ => return Err(AdventError::ParseError),
        };
        Ok(result)
    }
}

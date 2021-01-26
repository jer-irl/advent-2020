use std::str::FromStr;

use super::errors::AdventError;

use super::day12a::Instruction;

pub fn solve(input: &str) -> Result<(), AdventError> {
    let mut ship_state = ShipState::new();
    for instruction in input.split_whitespace().map(Instruction::from_str).collect::<Result<Vec<_>, _>>()? {
        ship_state.apply_instruction(&instruction);
    }
    let result = ship_state.position.x.abs() + ship_state.position.y.abs();
    println!("{}", result);
    Ok(())
}

struct ShipState {
    waypoint: Coordinate,
    position: Coordinate,
}

#[derive(Clone)]
struct Coordinate {
    x: isize,
    y: isize,
}

impl Coordinate {
    pub fn new(x: isize, y: isize) -> Self {
        Coordinate { x, y }
    }
}

impl ShipState {
    pub fn new() -> Self {
        ShipState {
            waypoint: Coordinate::new(10, -1), 
            position: Coordinate::new(0, 0),
        }
    }

    pub fn apply_instruction(&mut self, instruction: &Instruction) {
        match instruction {
            Instruction::North(n) => self.waypoint.y -= n,
            Instruction::East(n) => self.waypoint.x += n,
            Instruction::South(n) => self.waypoint.y += n,
            Instruction::West(n) => self.waypoint.x -= n,
            Instruction::Left(n) => {
                assert_eq!(n % 90, 0);
                let num_rotations = n / 90;
                for _ in 0..num_rotations {
                    let previous = self.waypoint.clone();
                    self.waypoint.x = previous.y;
                    self.waypoint.y = -previous.x;
                }
            }
            Instruction::Right(n) => {
                assert_eq!(n % 90, 0);
                let num_rotations = n / 90;
                for _ in 0..num_rotations {
                    let previous = self.waypoint.clone();
                    self.waypoint.x = -previous.y;
                    self.waypoint.y = previous.x;
                }
            }
            Instruction::Forward(n) => {
                self.position.x += n * self.waypoint.x;
                self.position.y += n * self.waypoint.y;
            }
        }
    }
}

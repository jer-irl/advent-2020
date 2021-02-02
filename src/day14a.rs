use std::collections::HashMap;

use lazy_static::lazy_static;
use regex::{Captures, Regex};

use super::errors::AdventError;

pub fn solve(input: &str) -> Result<(), AdventError> {
    let mut instructions = input.split_terminator('\n').map(Instruction::from);
    let mut mask = "";
    let mut memory: HashMap<u64, u64> = HashMap::new();
    while let Some(Ok(instruction)) = instructions.next() {
        match instruction {
            Instruction::Mask(m) => mask = m,
            Instruction::Assign{ address, value } => {
                memory.insert(address, apply_mask(mask, value));
            }
        }
    }

    if let Some(_) = instructions.peekable().peek() {
        Err(AdventError::ParseError)
    } else {
        let result: u64 = memory.values().sum();
        println!("{}", result);
        Ok(())
    }
}

/// Dumb slow way, could accomplish this with two masks, one with X <- 1, one with
/// X <- 0, then AND and OR
fn apply_mask(mask: &str, mut value: u64) -> u64 {
    for (i, c) in mask.chars().rev().enumerate() {
        let bit = 2u64.pow(i as u32);
        match c {
            'X' => (),
            '1' => value |= bit,
            '0' => value &= !bit,
            _ => unreachable!("Invalid mask value, should have been caught before"),
        }
    }
    value
}

pub enum Instruction<'a> {
    Mask(&'a str),
    Assign{ address: u64, value: u64 },
}

impl<'a> Instruction<'a> {
    pub fn from(line: &'a str) -> Result<Self, AdventError> {
        lazy_static! {
            static ref RE: Regex = Regex::new(r"(mask = (?P<mask_string>\w{36})|mem\[(?P<assign_address>\d+)\] = (?P<assign_value>\d+))").unwrap();
        }

        let captures: Captures = RE.captures(line).ok_or(AdventError::ParseError)?;
        if let Some(mask_string) = captures.name("mask_string") {
            Ok(Self::Mask(mask_string.as_str()))
        } else if let (Some(assign_address), Some(assign_value)) = (captures.name("assign_address"), captures.name("assign_value")) {
            match (assign_address.as_str().parse::<u64>(), assign_value.as_str().parse::<u64>()) {
                (Ok(address), Ok(value)) => Ok(Self::Assign{ address, value }),
                _ => Err(AdventError::ParseError)
            }
        } else {
            Err(AdventError::ParseError)
        }

    }
}

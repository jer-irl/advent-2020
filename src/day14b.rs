use std::collections::HashMap;

use super::errors::AdventError;

use super::day14a::Instruction;

pub fn solve(input: &str) -> Result<(), AdventError> {
    let instructions = input.lines().map(Instruction::from).collect::<Result<Vec<_>, _>>()?;
    let mut mask = "";
    let mut memory = HashMap::new();
    for instruction in instructions {
        match instruction {
            Instruction::Mask(m) => mask = m,
            Instruction::Assign{ address, value } => for addr in apply_mask(mask, address)? {
                memory.insert(addr, value);
            }
        }
    }

    let result: u64 = memory.iter().map(|(_addr, value)| value).sum();
    println!("{}", result);

    Ok(())
}

fn apply_mask(mask: &str, address: u64) -> Result<impl Iterator<Item = u64>, AdventError> {
    Ok(MaskAddrIterator::new(mask, address)?)
}

struct MaskAddrIterator {
    base_addr: u64,
    dfs_stack: Vec<(u8, bool)>,
    done: bool
}

impl MaskAddrIterator {
    pub fn new(mask: &str, base_addr: u64) -> Result<Self, AdventError> {
        let mut base_addr = base_addr;
        let mut dfs_stack = vec![];
        for (i, c) in mask.bytes().rev().enumerate().rev() {
            match c {
                b'1' => base_addr |= 1 << i,
                b'0' => (),
                b'X' => dfs_stack.push((i as u8, false)),
                _ => return Err(AdventError::ParseError),
            }
        }
        Ok(Self { base_addr, dfs_stack, done: false })
    }

    fn increment_stack(&mut self) -> bool {
        let idx_to_start_flip = self.dfs_stack.iter().enumerate().rev().find(|(_outer_i, (_inner_i, flag))| !flag);
        let outer_idx_to_start_flip = match idx_to_start_flip {
            Some((outer_idx, (_inner_idx, _flag))) => outer_idx,
            None => return false,
        };

        for i in outer_idx_to_start_flip..(self.dfs_stack.len()) {
            match self.dfs_stack.get_mut(i) {
                Some((_i, flag)) => *flag = !*flag,
                None => unreachable!("Logic error"),
            }
        }
        true
    }
}

impl Iterator for MaskAddrIterator {
    type Item = u64;

    fn next(&mut self) -> Option<Self::Item> {
        if self.done {
            return None
        }
        
        let mut result = self.base_addr;
        for (i, flag) in self.dfs_stack.iter() {
            match flag {
                true => result |= 1 << i,
                false => result &= !(1 << i),
            }
        }

        self.done = !self.increment_stack();
        Some(result)
    }
}

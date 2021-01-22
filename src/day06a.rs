use bitvec::prelude::*;

use super::errors::AdventError;

pub fn solve(input: &str) -> Result<(), AdventError> {
    let result: usize = input
        .split_terminator("\n\n")
        .map(|s| {
            let mut bits = bitarr![0; 26];
            for b in s.bytes().filter(|b| *b != b'\n') {
                bits.set((b - b'a') as usize, true);
            }
            bits.count_ones()
        })
        .sum();

    println!("{}", result);
    Ok(())
}

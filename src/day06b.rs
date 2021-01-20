use bitvec::prelude::*;

use super::errors::AdventError;

pub fn solve(input: &str) -> Result<(), AdventError> {
    let chunks = input.split_terminator("\n\n");
    let result: usize = chunks.map(handle_chunk).collect::<Result<Vec<_>, AdventError>>()?.iter().sum();
    println!("{}", result);
    Ok(())
}

fn handle_chunk(input: &str) -> Result<usize, AdventError> {
    let bitsets = input
        .split_whitespace()
        .map(|s| {
            let mut bits = bitarr![0; 26];
            s.bytes().for_each(|b| bits.set((b - b'a') as usize, true));
            bits
        });
    let intersection = bitsets.fold(bitarr![1; 26], |acc, bits| acc & bits);
    Ok(intersection.count_ones())
}

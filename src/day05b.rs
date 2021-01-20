use std::collections::BTreeSet;

use super::day05a::id_from_str;
use super::errors::AdventError;

pub fn solve(input: &str) -> Result<(), AdventError> {
    let ids: BTreeSet<_> = input
        .split_whitespace()
        .map(id_from_str)
        .collect::<Result<_, AdventError>>()?;
    for i in 1..ids.iter().max().unwrap() + 1 {
        if ids.contains(&(i - 1)) && !ids.contains(&i) && ids.contains(&(i + 1)) {
            println!("{}", i);
            return Ok(());
        }
    }
    Err(AdventError::NoSolution)
}

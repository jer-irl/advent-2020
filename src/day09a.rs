use super::errors::AdventError;

const PRELUDE_LENGTH: usize = 25;

pub fn solve(input: &str) -> Result<(), AdventError> {
    let numbers: Result<Vec<usize>, _> = input
        .split_whitespace()
        .map(|s| s.parse())
        .collect();
    let numbers = match numbers {
        Ok(v) => v,
        Err(_) => return Err(AdventError::ParseError),
    };

    for i in PRELUDE_LENGTH..numbers.len() {
        if !valid_window(&numbers[i - PRELUDE_LENGTH..i], numbers[i]) {
            println!("{}", numbers[i]);
            return Ok(())
        }
    }

    Err(AdventError::NoSolution)
}

fn valid_window(prelude: &[usize], i: usize) -> bool {
    for a in prelude {
        if *a > i { continue }
        let target = i - *a;
        if target != *a && prelude.contains(&target) {
            return true
        }
    }
    false
}

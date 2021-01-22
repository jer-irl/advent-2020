use super::errors::AdventError;

const PRELUDE_LENGTH: usize = 25;

pub fn solve(input: &str) -> Result<(), AdventError> {
    let numbers = get_numbers_from_input(input)?;

    match get_first_invalid(&numbers) {
        Some(IndexValuePair(_idx, n)) => {
            println!("{}", n);
            Ok(())
        }
        None => Err(AdventError::NoSolution),
    }
}

pub fn get_numbers_from_input(input: &str) -> Result<Vec<usize>, AdventError> {
    let numbers: Result<Vec<usize>, _> = input.split_whitespace().map(|s| s.parse()).collect();
    match numbers {
        Err(_) => Err(AdventError::ParseError),
        Ok(result) => Ok(result),
    }
}

pub struct IndexValuePair<T>(pub usize, pub T);

pub fn get_first_invalid(numbers: &[usize]) -> Option<IndexValuePair<usize>> {
    for i in PRELUDE_LENGTH..numbers.len() {
        if !cursor_is_valid(&numbers[i - PRELUDE_LENGTH..i], numbers[i]) {
            return Some(IndexValuePair(i, numbers[i]));
        }
    }
    None
}

fn cursor_is_valid(prelude: &[usize], i: usize) -> bool {
    for a in prelude {
        if *a > i {
            continue;
        }
        let target = i - *a;
        if target != *a && prelude.contains(&target) {
            return true;
        }
    }
    false
}

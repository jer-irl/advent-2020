use super::errors::AdventError;

pub fn solve(input: &str) -> Result<(), AdventError> {
    let strides = [(1, 1), (3, 1), (5, 1), (7, 1), (1, 2)];
    let result: usize = strides.iter().map(|stride| num_trees(&input, *stride)).collect::<Result<Vec<usize>, AdventError>>()?.iter().product();
    println!("{}", result);
    Ok(())
}

fn num_trees(input: &str, (right, down): (usize, usize)) -> Result<usize, AdventError> {
    let lines = input.split_whitespace();
    let result = lines.step_by(down).try_fold((0, 0), |(count, idx), line| {
        let new_idx = (idx + right) % line.len();
        match line.chars().nth(idx) {
            Some('.') => Some((count, new_idx)),
            Some('#') => Some((count + 1, new_idx)),
            _ => None,
        }
    });
    match result {
        Some((count, _)) => Ok(count),
        _ => Err(AdventError::ParseError),
    }
}
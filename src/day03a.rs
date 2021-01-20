use super::errors::AdventError;

pub fn solve(input: &str) -> Result<(), AdventError> {
    let mut rows = input.split_whitespace();
    if let Some((result, _)) = rows.try_fold((0, 0), |(count, idx), row| {
        let count = match row.chars().nth(idx) {
            Some('.') => count,
            Some('#') => count + 1,
            _ => return None,
        };
        Some((count, (idx + 3) % row.len()))
    }) {
        println!("{}", result);
        Ok(())
    } else {
        Err(AdventError::ParseError)
    }
}

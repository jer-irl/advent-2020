use super::errors::AdventError;

pub fn solve(input: &str) -> Result<(), AdventError> {
    let (row_idx_low, row_idx_high) = (0, 7);
    let (col_idx_low, col_idx_high) = (7, 10);
    let result = input
        .split_whitespace()
        .map(|s| {
            let mut row = 0usize;
            for c in s[row_idx_low..row_idx_high].chars() {
                match c {
                    'F' => {
                        row <<= 1;
                        row |= 0;
                    }
                    'B' => {
                        row <<= 1;
                        row |= 1;
                    }
                    _ => return Err(AdventError::ParseError),
                }
            }
            let mut col = 0usize;
            for c in s[col_idx_low..col_idx_high].chars() {
                match c {
                    'L' => {
                        col <<= 1;
                        col |= 0;
                    }
                    'R' => {
                        col <<= 1;
                        col |= 1;
                    }
                    _ => return Err(AdventError::ParseError),
                }
            }
            Ok(row * 8 + col)
        })
        .collect::<Result<Vec<_>, AdventError>>()?
        .into_iter()
        .max();

    if let Some(result) = result {
        println!("{}", result);
        Ok(())
    } else {
        Err(AdventError::NoSolution)
    }
}

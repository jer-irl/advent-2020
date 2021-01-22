use super::errors::AdventError;
use super::day09a::{get_first_invalid, get_numbers_from_input, IndexValuePair};

pub fn solve(input: &str) -> Result<(), AdventError> {
    let numbers = get_numbers_from_input(input)?;
    let IndexValuePair(_first_invalid_index, first_invalid_value) = match get_first_invalid(&numbers) {
        Some(index_value) => index_value,
        None => return Err(AdventError::NoSolution),
    };

    let target_sum = first_invalid_value;
    let mut lower_idx = 0usize;
    let mut upper_idx = 0usize;
    while upper_idx < numbers.len() {
        match numbers[lower_idx..=upper_idx].iter().sum::<usize>() {
            n if n == target_sum => {
                let min_number = *numbers[lower_idx..=upper_idx].iter().min().unwrap(); 
                let max_number = *numbers[lower_idx..=upper_idx].iter().max().unwrap();
                println!("{}", min_number + max_number);
                return Ok(())
            }
            n if n > target_sum => {
                lower_idx += 1;
                if lower_idx > upper_idx {
                    upper_idx += 1;
                }
            }
            n if n < target_sum => {
                upper_idx += 1;
            }
            _ => unreachable!(),
        }
    }

    Err(AdventError::NoSolution)
}

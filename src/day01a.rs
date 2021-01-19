use super::errors::AdventError;

const TARGET: usize = 2020;

pub fn solve(input: &str) -> Result<(), AdventError> {
    let nums = input
        .split_whitespace()
        .fold(Some(vec![]), |acc, s| match acc {
            None => acc,
            Some(mut v) => match s.parse::<usize>() {
                Ok(i) => {
                    v.push(i);
                    Some(v)
                }
                Err(_) => None,
            },
        });

    let nums = match nums {
        Some(v) => v,
        None => return Err(AdventError::ParseError),
    };

    let mut pairs = itertools::iproduct!(&nums, &nums);

    let result = pairs.find_map(|(lhs, rhs)| match lhs + rhs {
        TARGET => Some(lhs * rhs),
        _ => None,
    });

    match result {
        Some(x) => {
            println!("{}", x);
            Ok(())
        }
        None => Err(AdventError::NoSolution),
    }
}

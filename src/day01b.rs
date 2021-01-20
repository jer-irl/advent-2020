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

    let mut triples = itertools::iproduct!(&nums, &nums, &nums);

    let result = triples.find_map(|(a, b, c)| match a + b + c {
        TARGET => Some(a * b * c),
        _ => None,
    });

    match result {
        Some(n) => {
            println!("{}", n);
            Ok(())
        }
        None => Err(AdventError::NoSolution),
    }
}

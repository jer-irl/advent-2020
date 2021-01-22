use std::collections::HashMap;

use super::errors::AdventError;

pub fn solve(input: &str) -> Result<(), AdventError> {
    let mut numbers = match input
        .split_whitespace()
        .map(|s| s.parse())
        .collect::<Result<Vec<usize>, _>>()
    {
        Ok(n) => n,
        Err(_) => return Err(AdventError::ParseError),
    };
    numbers.push(0);
    numbers.push(*numbers.iter().max().unwrap() + 3);
    numbers.sort();
    let numbers = numbers;

    let mut cache = HashMap::new();

    let result = get_ways_to_arrange_slice(&numbers, &mut cache);
    println!("{}", result);

    Ok(())
}

fn get_ways_to_arrange_slice(numbers: &[usize], cache: &mut HashMap<usize, usize>) -> usize {
    if let Some(&n) = cache.get(&numbers[0]) {
        return n;
    }
    let possible_selections = numbers
        .iter()
        .enumerate()
        .skip(1)
        .take_while(|(_i, &x)| x - numbers[0] <= 3);
    let result = possible_selections
        .map(|(i, _x)| get_ways_to_arrange_slice(&numbers[i..], cache))
        .sum();

    let result = if result == 0 { 1 } else { result };

    cache.insert(numbers[0], result);

    result
}

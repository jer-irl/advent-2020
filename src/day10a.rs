use super::errors::AdventError;

pub fn solve(input: &str) -> Result<(), AdventError> {
    let mut numbers: Vec<usize> = match input.split_whitespace().map(|s| s.parse::<usize>()).collect() {
        Ok(numbers) => numbers,
        Err(_) => return Err(AdventError::ParseError),
    };
    numbers.push(0);
    numbers.sort();

    let numbers = numbers;

    let (n1, n3) = numbers[1..]
        .iter()
        .enumerate()
        .fold((0, 0), |(n1, n3), (i, &x)| match x - numbers[i] {
            1 => (n1 + 1, n3),
            3 => (n1, n3 + 1),
            _ => (n1, n3),
        });

    let result = n1 * (n3 + 1);
    println!("{}", result);

    Ok(())
}

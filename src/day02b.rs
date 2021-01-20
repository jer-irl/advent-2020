use regex::Regex;

use super::errors::AdventError;

pub fn solve(input: &str) -> Result<(), AdventError> {
    let re = Regex::new(r"(?P<lower_bound>\d+)-(?P<upper_bound>\d+) (?P<char>\w): (?P<password>\w+)").unwrap();
    let matches = re.captures_iter(&input);
    let result = matches.filter(|m| {
        let lower: usize = m.name("lower_bound").unwrap().as_str().parse().unwrap();
        let upper: usize = m.name("upper_bound").unwrap().as_str().parse().unwrap();
        let ch: char = m.name("char").unwrap().as_str().parse().unwrap();
        let password = m.name("password").unwrap().as_str();
        let first = password.as_bytes()[lower - 1] == ch as u8;
        let second = password.as_bytes()[upper - 1] == ch as u8;
        (first || second) && !(first && second)
    }).count();

    println!("{}", result);

    Ok(())
}

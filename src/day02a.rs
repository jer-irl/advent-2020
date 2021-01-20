use regex::Regex;

use super::errors::AdventError;

pub fn solve(input: &str) -> Result<(), AdventError> {
    let re =
        Regex::new(r"(?P<lower_bound>\d+)-(?P<upper_bound>\d+) (?P<char>\w): (?P<password>\w+)")
            .unwrap();
    let matches = re.captures_iter(&input);
    let result = matches
        .filter(|m| {
            let lower: usize = m.name("lower_bound").unwrap().as_str().parse().unwrap();
            let upper: usize = m.name("upper_bound").unwrap().as_str().parse().unwrap();
            let ch: char = m.name("char").unwrap().as_str().parse().unwrap();
            let password = m.name("password").unwrap().as_str();
            let occurrences = password.chars().filter(|c| *c == ch).count();
            lower <= occurrences && occurrences <= upper
        })
        .count();

    println!("{}", result);

    Ok(())
}

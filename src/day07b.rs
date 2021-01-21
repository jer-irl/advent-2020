use std::collections::HashMap;

use super::errors::AdventError;
use super::day07a::{get_rules, ContentRule};

const TARGET_COLOR: &str = "shiny gold";

pub fn solve(input: &str) -> Result<(), AdventError> {
    let rules = get_rules(input)?;
    let mut cache = HashMap::new();
    let result = max_capacity(TARGET_COLOR, &rules, &mut cache)?;
    println!("{}", result);
    Ok(())
}

fn max_capacity(color: &str, rules: &HashMap<String, Vec<ContentRule>>, cache: &mut HashMap<String, usize>) -> Result<usize, AdventError> {
    if let Some(n) = cache.get(color) {
        Ok(*n)
    } else if let Some(subrules) = rules.get(color) {
        subrules
            .iter()
            .try_fold(0, |acc, rule| {
                match max_capacity(&rule.color, &rules, cache) {
                    Ok(n) => Ok(acc + (rule.quantity * (n + 1))),
                    e => e,
                }
            })
    } else {
        Err(AdventError::ParseError)
    }
}

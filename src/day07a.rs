use std::collections::HashMap;

use regex::Regex;

use super::errors::AdventError;

const TARGET_COLOR: &str = "shiny gold";

pub fn solve(input: &str) -> Result<(), AdventError> {
    let rules = get_rules(input)?;
    let result = rules
        .keys()
        .map(|k| can_reach_target_color(&rules, k))
        .collect::<Result<Vec<bool>, AdventError>>()?
        .into_iter()
        .filter(|x| *x)
        .count();
    
    println!("{}", result);
    Ok(())
}

struct ContentRule {
    pub quantity: usize,
    pub color: String,
}

fn get_rules(input: &str) -> Result<HashMap<String, Vec<ContentRule>>, AdventError> {
    let rule_re = Regex::new(r"(?P<color>.+) bags contain (?P<contents>.+)").unwrap();
    let contents_re = Regex::new(r"(?P<num>\d+) (?P<color>\w+ \w+) bags?[\.,]?").unwrap();

    let rules: Option<HashMap<_, _>> = input
        .split_terminator('\n')
        .map(|line| {
            let rule = rule_re.captures(line)?;
            let outer_color = rule.name("color")?.as_str();
            let contents = contents_re.captures_iter(rule.name("contents")?.as_str());
            let content_rules: Vec<_> = contents
                .map(|captures| {
                    let quantity = captures.name("num")?.as_str().parse().ok()?;
                    let color = captures.name("color")?.as_str().to_string();
                    Some(ContentRule { quantity, color })
                })
                .collect::<Option<_>>()?;
            Some((outer_color.to_string(), content_rules))
        })
        .collect();
    match rules {
        Some(map) => Ok(map),
        None => Err(AdventError::ParseError),
    }
}

fn can_reach_target_color(rules: &HashMap<String, Vec<ContentRule>>, starting_color: &str) -> Result<bool, AdventError> {
    let color_rules = match rules.get(starting_color) {
        None => return Err(AdventError::NoSolution),
        Some(color_rules) => color_rules,
    };
    for rule in color_rules {
        if rule.color == TARGET_COLOR || can_reach_target_color(rules, &rule.color)? {
            return Ok(true)
        }
    }
    Ok(false)
}

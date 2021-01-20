use regex::Regex;
use std::collections::HashSet;

use super::errors::AdventError;

pub fn solve(input: &str) -> Result<(), AdventError> {
    let required_elems: HashSet<&str> = ["byr", "iyr", "eyr", "hgt", "hcl", "ecl", "pid"]
        .iter()
        .copied()
        .collect();
    let optional_elems: HashSet<&str> = ["cid"].iter().copied().collect();

    let item_re = Regex::new(r"(?P<tag>\w{3}):\S+").unwrap();
    let chunks = input.split_terminator("\n\n");
    let mut valid = 0usize;
    for chunk in chunks {
        let mut present = HashSet::new();
        let elems = item_re
            .captures_iter(&chunk)
            .map(|m| m.name("tag").unwrap().as_str());
        for elem in elems {
            if !required_elems.contains(elem) && !optional_elems.contains(elem) {
                return Err(AdventError::ParseError);
            }
            present.insert(elem);
        }
        if required_elems.is_subset(&present) {
            valid += 1;
        }
    }
    println!("{}", valid);

    Ok(())
}

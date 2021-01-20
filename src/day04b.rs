use std::collections::HashSet;

use regex::Regex;

use super::errors::AdventError;

pub fn solve(input: &str) -> Result<(), AdventError> {
    let result = input
        .split_terminator("\n\n")
        .map(validate_passport)
        .collect::<Result<Vec<bool>, AdventError>>()?
        .into_iter()
        .filter(|x| *x)
        .count();

    println!("{}", result);

    Ok(())
}

fn validate_passport(passport: &str) -> Result<bool, AdventError> {
    lazy_static::lazy_static! {
        static ref RE: Regex = Regex::new(r"(?x)
            (
                byr:(?P<byr>\d{4})
                |iyr:(?P<iyr>\d{4})
                |eyr:(?P<eyr>\d{4})
                |hgt:(?P<hgt>\d+)(?P<unit>cm|in)
                |hcl:(?P<hcl>(?-x)\#[0-9a-f]{6})
                |ecl:(?P<ecl>(amb|blu|brn|gry|grn|hzl|oth))
                |pid:(?P<pid>\d{9})
                |cid:(?P<cid>\S*)
            )(\s+|$)").unwrap();

        static ref REQUIRED_ELEMS: HashSet<&'static str> = ["byr", "iyr", "eyr", "hgt", "hcl", "ecl", "pid"].iter().copied().collect();
    }

    let matches = RE.captures_iter(passport);
    let mut found_items = HashSet::new();
    for m in matches {
        match validate_item(&m)? {
            Some(s) => {
                found_items.insert(s);
            }
            None => return Ok(false),
        }
    }

    Ok(found_items.is_superset(&REQUIRED_ELEMS))
}

fn validate_item(captures: &regex::Captures) -> Result<Option<&'static str>, AdventError> {
    if let Some(match_) = captures.name("byr") {
        let birth_year: usize = match_.as_str().parse()?;
        if 1920 <= birth_year && birth_year <= 2002 { Ok(Some("byr")) } else { Ok(None) }
    } else if let Some(match_) = captures.name("iyr") {
        let issue_year: usize = match_.as_str().parse()?;
        if 2010 <= issue_year && issue_year <= 2020 { Ok(Some("iyr")) } else { Ok(None) }
    } else if let Some(match_) = captures.name("eyr") {
        let expiration_year: usize = match_.as_str().parse()?;
        if 2020 <= expiration_year && expiration_year <= 2030 { Ok(Some("eyr")) } else { Ok(None) }
    } else if let Some(match_) = captures.name("hgt") {
        let height: usize = match_.as_str().parse()?;
        match captures.name("unit").and_then(|m| Some(m.as_str())) {
            Some("cm") => if 150 <= height && height <= 193 { Ok(Some("hgt")) } else { Ok(None) }
            Some("in") => if 59 <= height && height <= 76 { Ok(Some("hgt")) } else { Ok(None) }
            _ => Err(AdventError::ParseError)
        }
    } else if let Some(_) = captures.name("hcl") {
        Ok(Some("hcl"))
    } else if let Some(_) = captures.name("ecl") {
        Ok(Some("ecl"))
    } else if let Some(_) = captures.name("pid") {
        Ok(Some("pid"))
    } else if let Some(_) = captures.name("cid") {
        Ok(Some("cid"))
    } else {
        Err(AdventError::ParseError)
    }
}

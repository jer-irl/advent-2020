use std::cmp;

use regex::Regex;

use super::errors::AdventError;

pub fn solve(input: &str) -> Result<(), AdventError> {
    let structured_input = StructuredInput::from_input(input)?;
    let mut range_predicate = RangePredicate::new();
    for lower_upper_range in structured_input.range_predicate_sets.iter().flatten() {
        range_predicate.add_range(*lower_upper_range);
    }
    range_predicate.coalesce();
    let range_predicate = range_predicate;

    let mut result = 0u64;
    for ticket in structured_input.nearby_tickets.iter() {
        for n in ticket.iter() {
            if !range_predicate.accepts_value(*n) {
                result += *n;
            }
        }
    }

    println!("{}", result);
    Ok(())
}

struct StructuredInput {
    pub range_predicate_sets: Vec<Vec<(u64, u64)>>,
    pub your_ticket: Vec<u64>,
    pub nearby_tickets: Vec<Vec<u64>>,
}

impl StructuredInput {
    pub fn from_input(input: &str) -> Result<Self, AdventError> {
        let range_re = Regex::new(r"(?P<lower>\d+)-(?P<upper>\d+)").unwrap();
        let mut lines = input.lines();

        let mut range_predicate_sets = vec![];
        while let Some(line) = lines.next() {
            if line.len() == 0 {
                break;
            }
            let mut field_ranges = vec![];
            for captures in range_re.captures_iter(line) {
                field_ranges.push(
                    (
                        captures.name("lower").unwrap().as_str().parse::<u64>().unwrap(), 
                        captures.name("upper").unwrap().as_str().parse::<u64>().unwrap(),
                    )
                );
            }
            range_predicate_sets.push(field_ranges);
        }

        assert_eq!(lines.next(), Some("your ticket:"));
        let your_ticket: Vec<u64> = lines.next().unwrap().split_terminator(',').map(|n_str| n_str.parse().unwrap()).collect();
        assert_eq!(lines.next().unwrap().len(), 0);

        assert_eq!(lines.next(), Some("nearby tickets:"));
        let nearby_tickets: Vec<Vec<u64>> = lines
            .map(|l| -> Vec<u64> {
                l.split_terminator(',').map(|n_str| n_str.parse().unwrap()).collect()
            })
            .collect();

        Ok(Self { range_predicate_sets, your_ticket, nearby_tickets })
    }
}

struct RangePredicate {
    ranges: Vec<(u64, u64)>,
}

impl RangePredicate {
    pub fn new() -> Self {
        Self { ranges: vec![] }
    }

    pub fn add_range(&mut self, (low_inclusive, high_inclusive): (u64, u64)) {
        self.ranges.push((low_inclusive, high_inclusive))
    }

    pub fn coalesce(&mut self) {
        self.ranges.sort();
        let mut new_ranges = vec![];
        let mut current_range = None;
        for (low_inclusive, high_inclusive) in self.ranges.iter() {
            if current_range.is_none() {
                current_range = Some((*low_inclusive, *high_inclusive))
            } else if current_range.unwrap().1 >= *low_inclusive {
                current_range = Some((current_range.unwrap().0, cmp::max(current_range.unwrap().1, *high_inclusive)));
            } else {
                new_ranges.push(current_range.unwrap());
                current_range = Some((*low_inclusive, *high_inclusive));
            }
        }
        new_ranges.push(current_range.unwrap());

        self.ranges = new_ranges;
    }

    pub fn accepts_value(&self, value: u64) -> bool {
        self.ranges.iter().any(|(lower, upper)| (*lower..=*upper).contains(&value))
    }
}

//! Heavy use of newtype pattern here

use std::{cmp, hash::{Hash, Hasher}};

use regex::Regex;

use super::errors::AdventError;

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
pub struct Range(pub u64, pub u64);
#[derive(Debug, PartialEq, Eq)]
pub struct RangePredicate(pub Vec<Range>);
#[derive(Debug, PartialEq, Eq)]
pub struct Field(pub String, pub RangePredicate);

impl Hash for Field {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.0.hash(state)
    }
}

pub fn solve(input: &str) -> Result<(), AdventError> {
    let structured_input = StructuredInput::from_input(input)?;
    let mut range_predicate = RangePredicate::new();
    for lower_upper_range in structured_input.fields.iter().map(|field| &field.1.0).flatten() {
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

pub struct StructuredInput {
    pub fields: Vec<Field>,
    pub your_ticket: Vec<u64>,
    pub nearby_tickets: Vec<Vec<u64>>,
}

impl StructuredInput {
    pub fn from_input(input: &str) -> Result<Self, AdventError> {
        let label_re = Regex::new(r"(?P<label>[\w\s]+):").unwrap();
        let range_re = Regex::new(r"(?P<lower>\d+)-(?P<upper>\d+)").unwrap();
        let mut lines = input.lines();

        let mut fields = vec![];
        while let Some(line) = lines.next() {
            if line.len() == 0 {
                break;
            }
            let mut field_ranges = vec![];
            let label = label_re.captures(line).unwrap().name("label").unwrap().as_str();
            for captures in range_re.captures_iter(line) {
                field_ranges.push(
                    Range(
                        captures.name("lower").unwrap().as_str().parse::<u64>().unwrap(), 
                        captures.name("upper").unwrap().as_str().parse::<u64>().unwrap(),
                    )
                );
            }
            fields.push(Field(label.to_string(), RangePredicate(field_ranges)));
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

        Ok(Self { fields, your_ticket, nearby_tickets })
    }
}

impl RangePredicate {
    pub fn new() -> Self {
        Self(vec![])
    }

    pub fn add_range(&mut self, Range(low_inclusive, high_inclusive): Range) {
        self.0.push(Range(low_inclusive, high_inclusive))
    }

    pub fn coalesce(&mut self) {
        self.0.sort();
        let mut new_ranges = vec![];
        let mut current_range = None;
        for Range(low_inclusive, high_inclusive) in self.0.iter() {
            if current_range.is_none() {
                current_range = Some(Range(*low_inclusive, *high_inclusive))
            } else if current_range.unwrap().1 >= *low_inclusive {
                current_range = Some(Range(current_range.unwrap().0, cmp::max(current_range.unwrap().1, *high_inclusive)));
            } else {
                new_ranges.push(current_range.unwrap());
                current_range = Some(Range(*low_inclusive, *high_inclusive));
            }
        }
        new_ranges.push(current_range.unwrap());

        self.0 = new_ranges;
    }

    pub fn accepts_value(&self, value: u64) -> bool {
        self.0.iter().any(|Range(lower, upper)| (*lower..=*upper).contains(&value))
    }
}

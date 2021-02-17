use std::collections::{HashSet, HashMap};

use super::errors::AdventError;

use crate::day16a::{RangePredicate, StructuredInput};

pub fn solve(input: &str) -> Result<(), AdventError> {
    let structured_input = StructuredInput::from_input(input)?;
    let valid_tickets = valid_tickets(&structured_input);

    let mut possible_fields: HashMap<u64, HashSet<&(String, Vec<(u64, u64)>)>> = HashMap::new();
    for field in structured_input.range_predicate_sets.iter() {
        let (_label, ranges) = field;
        let mut label_range_predicate = RangePredicate::new();
        for range in ranges.iter() {
            label_range_predicate.add_range(*range);
        }
        label_range_predicate.coalesce();

        for column in 0..(valid_tickets[0].len()) {
            let all_tickets_work_for_column = valid_tickets.iter()
                .map(|vec| vec[column])
                .all(|val_for_column| label_range_predicate.accepts_value(val_for_column));
            if all_tickets_work_for_column {
                if !possible_fields.contains_key(&(column as u64)) {
                    possible_fields.insert(column as u64, HashSet::new());
                }
                possible_fields.get_mut(&(column as u64)).unwrap().insert(field);
            }
        }
    }

    let mut resolved_fields: HashMap<u64, String> = HashMap::new();

    while possible_fields.len() > 0 {
        if possible_fields.iter().any(|(_col, pending_fields)| pending_fields.len() == 0) {
            return Err(AdventError::NoSolution)
        }
        
        let newly_resolved_columns = {
            let newly_resolved: Vec<_> = possible_fields.iter()
                .filter(|(_column, pending_fields)| pending_fields.len() == 1)
                .collect();
            if newly_resolved.len() == 0 {
                return Err(AdventError::UnimplementedPartError)
            }

            let mut newly_resolved_columns = vec![];
            for (column, pending_fields) in newly_resolved {
                let (label, _) = pending_fields.iter().next().unwrap();
                resolved_fields.insert(*column, label.clone());
                newly_resolved_columns.push(*column);
            }
            newly_resolved_columns
        };

        for newly_resolved_column in newly_resolved_columns {
            possible_fields.remove(&newly_resolved_column);
        }
    }


    Err(AdventError::UnimplementedPartError)
}

fn valid_tickets(structured_input: &StructuredInput) -> Vec<&Vec<u64>> {
    let mut overall_range_predicate = RangePredicate::new();
    for lower_upper_range in structured_input.range_predicate_sets.iter().map(|(_label, ranges)| ranges).flatten() {
        overall_range_predicate.add_range(*lower_upper_range);
    }
    overall_range_predicate.coalesce();
    let overall_range_predicate = overall_range_predicate;

    structured_input.nearby_tickets.iter()
        .filter(|&ticket_numbers| {
            ticket_numbers.iter().all(|n| overall_range_predicate.accepts_value(*n))
        })
        .collect()
}



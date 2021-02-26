use std::collections::{HashSet, HashMap};

use itertools::Itertools;

use super::errors::AdventError;

use crate::day16a::{Field, RangePredicate, StructuredInput};

#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
struct ColumnIdentifier(u64);
type PossibleColumnAssignments<'a> = HashMap<ColumnIdentifier, HashSet<&'a Field>>;
type ResolvedColumnAssignments<'a> = HashMap<ColumnIdentifier, &'a Field>;

pub fn solve(input: &str) -> Result<(), AdventError> {
    let structured_input = StructuredInput::from_input(input)?;
    let valid_tickets = valid_tickets(&structured_input);

    let mut possible_fields: PossibleColumnAssignments = HashMap::new();
    for field in structured_input.fields.iter() {
        let Field(_label, ranges) = field;
        let mut label_range_predicate = RangePredicate::new();
        for range in ranges.0.iter() {
            label_range_predicate.add_range(range.clone());
        }
        label_range_predicate.coalesce();

        for column in 0..(valid_tickets[0].len()) {
            let column = ColumnIdentifier(column as u64);
            let all_tickets_work_for_column = valid_tickets.iter()
                .map(|vec| vec[column.0 as usize])
                .all(|val_for_column| label_range_predicate.accepts_value(val_for_column));
            if all_tickets_work_for_column {
                if !possible_fields.contains_key(&column) {
                    possible_fields.insert(column, HashSet::new());
                }
                possible_fields.get_mut(&column).unwrap().insert(field);
            }
        }
    }

    let mut resolved_fields: ResolvedColumnAssignments = HashMap::new();
    while possible_fields.len() > 0 {
        if possible_fields.iter().any(|(_col, pending_fields)| pending_fields.len() == 0) {
            return Err(AdventError::NoSolution)
        }
        
        let newly_resolved_columns = {
            let newly_resolved: Vec<_> = possible_fields.iter()
                .filter(|(_column, pending_fields)| pending_fields.len() == 1)
                .collect();
            if newly_resolved.len() == 0 {
                break;
            }

            let mut newly_resolved_columns = vec![];
            for (column, pending_fields) in newly_resolved {
                let field = pending_fields.iter().next().unwrap();
                resolved_fields.insert(*column, field);
                newly_resolved_columns.push(*column);
            }
            newly_resolved_columns
        };

        for newly_resolved_column in newly_resolved_columns {
            possible_fields.remove(&newly_resolved_column);
        }
    }

    let possible_fields = possible_fields;

    if let Some(solved_fields) = recursive_solution_helper(&possible_fields, &resolved_fields) {
        println!("{:?}", solved_fields);

        let prefix = "departure";

        let result: u64 = solved_fields.iter()
            .filter(|(_col_id, Field(label, _predicate))| label.starts_with(prefix))
            .map(|(col_id, _)| *structured_input.your_ticket.get(col_id.0 as usize).unwrap())
            .product();
            //.collect();

        println!("{:?}", result);
        return Ok(())
    }
    Err(AdventError::NoSolution)
}

fn recursive_solution_helper<'a>(pending: &PossibleColumnAssignments<'a>, resolved: &ResolvedColumnAssignments<'a>) -> Option<ResolvedColumnAssignments<'a>> {
    let column_to_solve = match pending.iter().sorted_by(|(_, fields1), (_, fields2)| fields2.len().cmp(&fields1.len())).rev().next() {
        Some((column_id, _)) => column_id,
        None => return Some(resolved.clone()),
    };

    let mut pretend_resolved_column_assignments = resolved.clone();
    for possible_field in pending.get(column_to_solve).unwrap() {
        pretend_resolved_column_assignments.insert(*column_to_solve, possible_field);
        let pretend_pending_column_assignments = pending.iter()
            .filter(|(col_id, _)| *col_id != column_to_solve)
            .map(|(col_id, allowed_fields)| (*col_id, allowed_fields.iter().filter(|f| *f != possible_field).copied().collect::<HashSet<&Field>>()))
            .collect();

        if let Some(resolved_solution) = recursive_solution_helper(&pretend_pending_column_assignments, &pretend_resolved_column_assignments) {
            return Some(resolved_solution)
        }
    }

    None
}

fn valid_tickets(structured_input: &StructuredInput) -> Vec<&Vec<u64>> {
    let mut overall_range_predicate = RangePredicate::new();
    for lower_upper_range in structured_input.fields.iter().map(|Field(_label, range_predicate)| range_predicate.0.iter()).flatten() {
        overall_range_predicate.add_range(lower_upper_range.clone());
    }
    overall_range_predicate.coalesce();
    let overall_range_predicate = overall_range_predicate;

    let mut result: Vec<_> = structured_input.nearby_tickets.iter()
        .filter(|&ticket_numbers| {
            ticket_numbers.iter().all(|n| overall_range_predicate.accepts_value(*n))
        })
        .collect();
    result.push(&structured_input.your_ticket);
    result
}

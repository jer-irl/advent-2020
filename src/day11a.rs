use std::collections::HashMap;

use itertools::iproduct;

use super::errors::AdventError;

const NEIGHBOR_TOLERANCE: usize = 4;

pub fn solve(input: &str) -> Result<(), AdventError> {
    let mut waiting_area = WaitingArea::new(input, day11arules)?;
    while waiting_area.simulate_step() {}
    println!("{}", waiting_area.num_occupied());
    Ok(())
}

#[derive(PartialEq)]
pub struct WaitingArea<F>
    where F: Fn(SeatState, usize) -> SeatState
{
    state: HashMap<(isize, isize), SeatState>,
    rules: F,
}

impl<F> WaitingArea<F> 
    where F: Fn(SeatState, usize) -> SeatState
{
    pub fn new(input: &str, rules: F) -> Result<Self, AdventError> {
        let mut state = HashMap::new();
        for (row, line) in input.split_whitespace().enumerate() {
            for (col, c) in line.chars().enumerate() {
                match c {
                    '.' => (),
                    'L' => {
                        if state
                            .insert((row as isize, col as isize), SeatState::Vacant)
                            .is_some()
                        {
                            return Err(AdventError::ParseError);
                        }
                    }
                    '#' | _ => return Err(AdventError::ParseError),
                }
            }
        }
        Ok(Self { state, rules })
    }

    pub fn simulate_step(&mut self) -> bool {
        let prev_state = self.state.clone();
        let neighbor_index_offsets = iproduct!((-1..=1), (-1..=1))
            .into_iter()
            .filter(|(x, y)| *x != 0 || *y != 0);
        let mut any_changes = false;
        for ((row, col), seat_state) in prev_state.iter() {
            let neighbor_index_offsets = neighbor_index_offsets.clone();
            let num_neighbors = neighbor_index_offsets
                .filter(|(drow, dcol)| {
                    let neighbor_state = prev_state.get(&(*row + drow, *col + dcol));
                    match neighbor_state {
                        Some(SeatState::Occupied) => true,
                        Some(SeatState::Vacant) => false,
                        None => false,
                    }
                })
                .count();
            let new_state = (self.rules)(*seat_state, num_neighbors);
            any_changes = any_changes || new_state != *seat_state;
            self.state.insert((*row, *col), new_state).expect("Coding error");
        }
        any_changes
    }

    pub fn num_occupied(&self) -> usize {
        self.state
            .iter()
            .filter(|&(_, &seat_state)| seat_state == SeatState::Occupied)
            .count()
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum SeatState {
    Occupied,
    Vacant,
}

fn day11arules(current_state: SeatState, num_neighbors: usize) -> SeatState {
    match (current_state, num_neighbors) {
        (SeatState::Vacant, 0) => SeatState::Occupied,
        (SeatState::Occupied, n) if n >= NEIGHBOR_TOLERANCE => SeatState::Vacant,
        _ => current_state,
    }
}

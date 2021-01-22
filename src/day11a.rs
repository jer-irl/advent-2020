use std::collections::HashMap;

use itertools::iproduct;

use super::errors::AdventError;

pub fn solve(input: &str) -> Result<(), AdventError> {
    let mut waiting_area = WaitingArea::new(input)?;
    while waiting_area.simulate_step() {}
    println!("{}", waiting_area.num_occupied());
    Ok(())
}

#[derive(PartialEq)]
struct WaitingArea {
    state: HashMap<(isize, isize), SeatState>,
}

impl WaitingArea {
    pub fn new(input: &str) -> Result<Self, AdventError> {
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
        Ok(Self { state })
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
            match seat_state {
                SeatState::Vacant => {
                    if num_neighbors == 0 {
                        self.state
                            .insert((*row, *col), SeatState::Occupied)
                            .expect("Coding error");
                        any_changes = true;
                    }
                }
                SeatState::Occupied => {
                    if num_neighbors >= 4 {
                        self.state
                            .insert((*row, *col), SeatState::Vacant)
                            .expect("Coding error");
                        any_changes = true;
                    }
                }
            }
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
enum SeatState {
    Occupied,
    Vacant,
}

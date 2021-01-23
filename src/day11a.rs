use std::collections::HashMap;

use itertools::iproduct;

use super::errors::AdventError;
use super::waiting_area::{
    private,
    SeatState, 
    WaitingArea
};

const NEIGHBOR_TOLERANCE: usize = 4;

pub fn solve(input: &str) -> Result<(), AdventError> {
    let mut waiting_area = Day11aWaitingArea::new(input)?;
    while waiting_area.simulate_step() {}
    println!("{}", waiting_area.num_occupied());
    Ok(())
}

#[derive(Clone, PartialEq)]
struct Day11aWaitingArea {
    state: HashMap<(isize, isize), SeatState>,
}

impl Day11aWaitingArea {
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
}

impl private::WaitingAreaPrivate for Day11aWaitingArea {
    fn apply_rules(current_state: SeatState, num_neighbors: usize) -> SeatState {
        match (current_state, num_neighbors) {
            (SeatState::Vacant, 0) => SeatState::Occupied,
            (SeatState::Occupied, n) if n >= NEIGHBOR_TOLERANCE => SeatState::Vacant,
            _ => current_state,
        }
    }

    fn num_neighbors(&self, (row, col): (usize, usize)) -> usize {
        iproduct!((-1..=1), (-1..=1))
            .filter(|(drow, dcol)| *drow != 0 || *dcol != 0)
            .filter(|(drow, dcol)| self.state.get(&(row as isize + drow, col as isize + dcol)) == Some(&SeatState::Occupied))
            .count()
    }

    fn iter<'a>(&'a self) -> Box<dyn 'a + Iterator<Item = (SeatState, usize)>> {
        Box::new(
            self.state
                .iter()
                .map(move |((row, col), seat_state)| {
                    (*seat_state, self.num_neighbors((*row as usize, *col as usize)))
                })
        )
    }

    fn iter_mut(&mut self) -> Box<dyn '_ + Iterator<Item = (&mut SeatState, usize)>> {
        let prev = self.clone();
        Box::new(
            self.state
                .iter_mut()
                .map(move |((row, col), seat_state)| {
                    (seat_state, prev.num_neighbors((*row as usize, *col as usize)))
                })
        )
    }
}

impl WaitingArea for Day11aWaitingArea {}

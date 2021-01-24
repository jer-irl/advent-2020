use itertools::chain;

use super::errors::AdventError;
use super::waiting_area::{
    private,
    SeatState, 
    WaitingArea
};

const NEIGHBOR_TOLERANCE: usize = 5;

pub fn solve(input: &str) -> Result<(), AdventError> {
    let mut waiting_area = Day11bWaitingArea::new(input)?;
    while waiting_area.simulate_step() {}
    println!("{}", waiting_area.num_occupied());
    Ok(())
}

#[derive(Clone, PartialEq)]
struct Day11bWaitingArea {
    state: Vec<Vec<SeatState>>,
}

impl Day11bWaitingArea {
    pub fn new(input: &str) -> Result<Self, AdventError> {
        let state = input
            .split_whitespace()
            .map(|line| {
                line.chars().map(SeatState::from).collect()
            })
            .collect();
        Ok(Self { state })
    }

    pub fn get(&self, (row, col): (isize, isize)) -> Option<SeatState> {
        if row < 0 || col < 0 {
            None
        } else {
            self.state.get(row as usize)?.get(col as usize).map(|s| *s)
        }
    }
}

impl private::WaitingAreaPrivate for Day11bWaitingArea {
    fn apply_rules(current_state: SeatState, num_neighbors: usize) -> SeatState {
        match (current_state, num_neighbors) {
            (SeatState::Vacant, 0) => SeatState::Occupied,
            (SeatState::Occupied, n) if n >= NEIGHBOR_TOLERANCE => SeatState::Vacant,
            _ => current_state,
        }
    }

    fn num_neighbors(&self, (row, col): (usize, usize)) -> usize {
        let offsets = [(-1, -1), (-1, 0), (-1, 1), (0, -1), (0, 1), (1, -1), (1, 0), (1, 1)];
        offsets.iter()
            .filter(|(drow, dcol)| {
                let mut look_distance = 1;
                while let Some(state) = self.get((row as isize + (drow * look_distance), col as isize + (dcol * look_distance))) {
                    look_distance += 1;
                    match state {
                        SeatState::Occupied => return true,
                        SeatState::Vacant => return false,
                        SeatState::NoChair => continue,
                    }
                }
                false
            })
            .count()
    }

    fn iter<'a>(&'a self) -> Box<dyn 'a + Iterator<Item = (SeatState, usize)>> {
        Box::new(
            self.state
                .iter()
                .enumerate()
                .flat_map(move |(row, v)| {
                    v.iter().enumerate().map(move |(col, state)| (*state, self.num_neighbors((row, col))))
                })
        )
    }

    fn iter_mut(&mut self) -> Box<dyn '_ + Iterator<Item = (&mut SeatState, usize)>> {
        let prev = self.clone();
        Box::new(
            self.state
                .iter_mut()
                .enumerate()
                .flat_map(move |(row, v)| {
                    let prev = prev.clone();
                    v.iter_mut().enumerate().map(move |(col, state)| (state, prev.num_neighbors((row, col))))
                })
        )
    }
}

impl WaitingArea for Day11bWaitingArea {}

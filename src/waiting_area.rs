pub trait WaitingArea: PartialEq + Clone + private::WaitingAreaPrivate {
    fn num_occupied(&self) -> usize {
        self.iter()
            .filter(|(state, _)| *state == SeatState::Occupied)
            .count()
    }

    fn simulate_step(&mut self) -> bool {
        let prev = self.clone();
        let mut any_changes = false;
        for ((prev_state, num_neighbors), (state, _)) in prev.iter().zip(self.iter_mut()) {
            let new_state = Self::apply_rules(prev_state, num_neighbors);
            any_changes = any_changes || prev_state != new_state;
            *state = new_state;
        }
        any_changes
    }
}

pub(crate) mod private {
    use super::*;

    pub trait WaitingAreaPrivate {
        fn apply_rules(current_state: SeatState, num_neighbors: usize) -> SeatState;
        fn num_neighbors(&self, coordinates: (usize, usize)) -> usize;
        fn iter<'a>(&'a self) -> Box<dyn 'a + Iterator<Item = (SeatState, usize)>>;
        fn iter_mut(&mut self) -> Box<dyn '_ + Iterator<Item = (&mut SeatState, usize)>>;
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum SeatState {
    Occupied,
    Vacant,
    NoChair,
}

impl From<char> for SeatState {
    fn from(c: char) -> Self {
        match c {
            '#' => Self::Occupied,
            'L' => Self::Vacant,
            '.' => Self::NoChair,
            _ => unreachable!(),
        }
    }
}

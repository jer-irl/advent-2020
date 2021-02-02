//! I messed this one up.  It's inefficient and unclear.  Maybe I'll improve it at some point.

use std::collections::BTreeMap as HashMap;

use super::errors::AdventError;

pub fn solve(input: &str) -> Result<(), AdventError> {
    let seed_numbers: Vec<_> = input.split_terminator(',').map(|s| s.parse::<u64>().unwrap()).collect();
    let result = solve_parsed(&seed_numbers, 2020)?;

    println!("{}", result);

    Ok(())
}

pub fn solve_parsed(seed_numbers: &[u64], i: usize) -> Result<u64, AdventError> {
    let iter = MemoryGameIterator::new(&seed_numbers);
    let _debug = iter.take(i).collect::<Vec<_>>();
    let mut iter = MemoryGameIterator::new(&seed_numbers);
    iter.nth(i - 1).ok_or(AdventError::NoSolution)
}

struct MemoryGameIterator<'a> {
    seed_numbers: &'a [u64],
    current_idx: u64,
    spoken_numbers: Vec<u64>,
    last_spoken_time: HashMap<u64, LastTwoIdxs>,
}

struct LastTwoIdxs(Option<u64>, Option<u64>);

impl LastTwoIdxs {
    pub fn new() -> Self {
        Self(None, None)
    }

    pub fn push(&mut self, next_idx: u64) {
        self.0 = self.1;
        self.1 = Some(next_idx);
    }

    pub fn most_recently_pushed(&self) -> Option<u64> {
        self.1
    }

    pub fn next_most_recently_pushed(&self) -> Option<u64> {
        self.0
    }
}

impl<'a> MemoryGameIterator<'a> {
    pub fn new(seed_numbers: &'a [u64]) -> Self {
        let current_idx = 0u64;
        let spoken_numbers = vec![];
        let last_spoken_time = HashMap::new();

        Self { seed_numbers, current_idx, spoken_numbers, last_spoken_time }
    }
}

impl<'a> Iterator for MemoryGameIterator<'a> {
    type Item = u64;

    fn next(&mut self) -> Option<Self::Item> {
        let current_number = self.seed_numbers.get(self.current_idx as usize).copied().unwrap_or_else(|| {

            let last_spoken_number = self.spoken_numbers.last().unwrap();
            match self.last_spoken_time.get(&last_spoken_number) {
                Some(prev_idxs) if prev_idxs.next_most_recently_pushed().is_some() => {
                    self.current_idx - 1 - prev_idxs.next_most_recently_pushed().unwrap()
                },
                Some(_) | None => 0
            }
        });

        self.spoken_numbers.push(current_number);
        if !self.last_spoken_time.contains_key(&current_number) {
            self.last_spoken_time.insert(current_number, LastTwoIdxs::new());
        }
        self.last_spoken_time.get_mut(&current_number).unwrap().push(self.current_idx);
        assert_eq!(*self.spoken_numbers.get(self.current_idx as usize).unwrap(), current_number);
        assert_eq!(self.last_spoken_time.get(&current_number).unwrap().most_recently_pushed().unwrap(), self.current_idx);
        self.current_idx += 1;
        Some(current_number)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example0() -> Result<(), AdventError> {
        let seed_numbers = [0, 3, 6];
        assert_eq!(solve_parsed(&seed_numbers, 1)?, 0);
        assert_eq!(solve_parsed(&seed_numbers, 2)?, 3);
        assert_eq!(solve_parsed(&seed_numbers, 3)?, 6);
        assert_eq!(solve_parsed(&seed_numbers, 4)?, 0);
        assert_eq!(solve_parsed(&seed_numbers, 5)?, 3);
        assert_eq!(solve_parsed(&seed_numbers, 6)?, 3);
        assert_eq!(solve_parsed(&seed_numbers, 7)?, 1);
        assert_eq!(solve_parsed(&seed_numbers, 8)?, 0);
        assert_eq!(solve_parsed(&seed_numbers, 9)?, 4);
        assert_eq!(solve_parsed(&seed_numbers, 10)?, 0);
        Ok(())
    }

    #[test]
    fn example1() -> Result<(), AdventError> {
        assert_eq!(solve_parsed(&[1, 3, 2], 2020)?, 1);
        Ok(())
    }

    #[test]
    fn example2() -> Result<(), AdventError> {
        assert_eq!(solve_parsed(&[2, 1, 3], 2020)?, 10);
        Ok(())
    }

    #[test]
    fn example3() -> Result<(), AdventError> {
        assert_eq!(solve_parsed(&[1, 2, 3], 2020)?, 27);
        Ok(())
    }

    #[test]
    fn example4() -> Result<(), AdventError> {
        assert_eq!(solve_parsed(&[2, 3, 1], 2020)?, 78);
        Ok(())
    }

    #[test]
    fn example5() -> Result<(), AdventError> {
        assert_eq!(solve_parsed(&[3, 2, 1], 2020)?, 438);
        Ok(())
    }

    #[test]
    fn example6() -> Result<(), AdventError> {
        assert_eq!(solve_parsed(&[3, 1, 2], 2020)?, 1836);
        Ok(())
    }
}

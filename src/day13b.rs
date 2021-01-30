//! I peeked at a solution.  The insight I was missing is that not only are values that contain any (>= 1)
//! match equally spaced, but each subset of N matches is also equally spaced with equivalent subsets.
//! This means we can stride forward and whenever a match increments the number of matches found, we can
//! increment the stride.

use itertools::Itertools;

use super::errors::AdventError;

pub fn solve(input: &str) -> Result<(), AdventError> {
    let lines: Vec<_> = input.split_whitespace().collect();
    let buses: Vec<_> = lines[1]
        .split_terminator(',')
        .enumerate()
        .filter_map(|(offset, id)| 
            id.parse::<usize>().map(|id| (offset, id)).ok()
        )
        .sorted_by_key(|(_, id)| *id)
        .rev()
        .collect();
    let result = calculate(&buses[..]);

    println!("{}", result);
    Ok(())
}

fn calculate(input: &[(usize, usize)]) -> usize {
    struct BusRecord {
        index: usize,
        id: usize,
        first_solution: Option<usize>,
        solution_interval: Option<usize>,
    }

    let mut bus_records: Vec<_> = input.iter().map(|(offset, id)| BusRecord { index: *offset, first_solution: None, solution_interval: None, id: *id}).collect();
    let mut stride = 1;
    let mut t = 0usize;
    let mut number_matching = 0;

    loop {
        let number_matching_attempt = bus_records.iter().take_while(|BusRecord{ index: offset, id, .. }| (t + *offset) % *id == 0).count();
        if number_matching_attempt == input.len() {
            break
        } else if number_matching_attempt > 0 && number_matching_attempt >= number_matching {
            number_matching = number_matching_attempt;
            let record = bus_records.get_mut(number_matching - 1).unwrap();
            if record.first_solution.is_none() {
                record.first_solution = Some(t);
            } else if record.solution_interval.is_none() {
                record.solution_interval = Some(t - record.first_solution.unwrap());
                stride = record.solution_interval.unwrap();
            }
        }
        t += stride
    }
    t
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example1() {
        let input = [(0, 17), (2, 13), (3, 19)];
        assert_eq!(calculate(&input), 3417);
    }

    #[test]
    fn example2() {
        let input = [(0, 67), (1, 7), (2, 59), (3, 61)];
        assert_eq!(calculate(&input), 754018);
    }

    #[test]
    fn example3() {
        let input = [(0, 67), (2, 7), (3, 59), (4, 61)];
        assert_eq!(calculate(&input), 779210);
    }

    #[test]
    fn example4() {
        let input = [(0, 67), (1, 7), (3, 59), (4, 61)];
        assert_eq!(calculate(&input), 1261476);
    }

    #[test]
    fn example5() {
        let input = [(0, 1789), (1, 37), (2, 47), (3, 1889)];
        assert_eq!(calculate(&input), 1202161486);
    }

}

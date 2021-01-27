// use std::convert;

// use itertools::Itertools;

use super::errors::AdventError;

pub fn solve(input: &str) -> Result<(), AdventError> {
    Err(AdventError::UnimplementedPartError)
    // let lines: Vec<_> = input.split_whitespace().collect();
    // let buses: Vec<_> = lines[1]
    //     .split_terminator(',')
    //     .enumerate()
    //     .filter_map(|(offset, id)| 
    //         id.parse::<usize>().map(|id| (offset, id)).ok()
    //     )
    //     .sorted_by_key(|(_, id)| *id)
    //     .rev()
    //     .collect();
    // let result = calculate(&buses[..]);

    // println!("{}", result);
    // Ok(())
}

fn calculate(input: &[(usize, usize)]) -> usize {
    let increment = input[0].1;
    let mut t = 0usize;
    loop {
        if input.iter().skip(1).all(|(offset, id)| (t + *offset) % *id == 0) {
            break
        }

        t += increment;
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

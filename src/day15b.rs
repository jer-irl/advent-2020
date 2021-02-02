use super::{
    errors::AdventError,
    day15a,
};

pub fn solve(input: &str) -> Result<(), AdventError> {
    let seed_numbers: Vec<_> = input.split_terminator(',').map(|s| s.parse::<u64>().unwrap()).collect();
    let result = day15a::solve_parsed(&seed_numbers, 30_000_000)?;

    println!("{}", result);

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    const N: usize = 30_000_000;

    #[test]
    fn example1() -> Result<(), AdventError> {
        assert_eq!(day15a::solve_parsed(&[0, 3, 6], N)?, 175594);
        Ok(())
    }

    #[test]
    fn example2() -> Result<(), AdventError> {
        assert_eq!(day15a::solve_parsed(&[1, 3, 2], N)?, 2578);
        Ok(())
    }

    #[test]
    fn example3() -> Result<(), AdventError> {
        assert_eq!(day15a::solve_parsed(&[2, 1, 3], N)?, 3544142);
        Ok(())
    }

    #[test]
    fn example4() -> Result<(), AdventError> {
        assert_eq!(day15a::solve_parsed(&[1, 2, 3], N)?, 261214);
        Ok(())
    }

    #[test]
    fn example5() -> Result<(), AdventError> {
        assert_eq!(day15a::solve_parsed(&[2, 3, 1], N)?, 6895259);
        Ok(())
    }

    #[test]
    fn example6() -> Result<(), AdventError> {
        assert_eq!(day15a::solve_parsed(&[3, 2, 1], N)?, 18);
        Ok(())
    }

    #[test]
    fn example7() -> Result<(), AdventError> {
        assert_eq!(day15a::solve_parsed(&[3, 1, 2], N)?, 362);
        Ok(())
    }
}

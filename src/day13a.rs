use super::errors::AdventError;

pub fn solve(input: &str) -> Result<(), AdventError> {
    let lines: Vec<_> = input.split_whitespace().collect();
    let earliest: usize = lines[0].parse().unwrap();
    let buses = lines[1].split_terminator(',').filter_map(|s| s.parse::<usize>().ok());
    let (id, earliest_departure_wait) = buses
        .map(|interval| {
            let remainder = earliest % interval;
            let earliest_departure_wait = interval - remainder;
            (interval, earliest_departure_wait)
        })
        .min_by_key(|(_, earliest_departure)| *earliest_departure)
        .unwrap();
    let result = id * earliest_departure_wait;
    println!("{}", result);
    Ok(())
}

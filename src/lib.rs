use std::fmt;
use std::fs::File;
use std::io;
use std::io::Read;

use clap::ArgMatches;

mod day01a;
mod day01b;
mod day02a;
mod day02b;
mod day03a;
mod day03b;
mod day04a;
mod day04b;
mod day05a;
mod day05b;
mod day06a;
mod day06b;
mod day07a;
mod day07b;
mod day08a;
mod day08b;
mod day09a;
mod day09b;
mod day10a;
mod day10b;
mod day11a;
mod day11b;
mod day12a;
mod day12b;
mod day13a;
mod day13b;

mod waiting_area;
mod vm;

const NUM_PROBLEMS: u8 = 25;
const EXCLUDE_FROM_DEFAULT: [u8; 1] = [11];

pub struct Config {
    read_stdin: bool,
    day: Option<u8>,
    part: Option<char>,
    include_all: bool,
}

impl Config {
    pub fn new(matches: &ArgMatches) -> Result<Self, errors::AdventError> {
        let read_stdin = matches.is_present("stdin");
        let day: Option<u8> = matches
            .value_of("DAY")
            .map(|s| s.parse::<u8>())
            .transpose()?
            .and_then(|n| match n {
                0 => None,
                n => Some(n),
            });
        let part: Option<char> = matches
            .value_of("PART")
            .map(|s| s.parse::<char>())
            .transpose()?
            .and_then(|c| match c {
                '0' => None,
                c => Some(c),
            });
        let include_all = matches.is_present("include-all");

        if read_stdin && (day == None || part == None) {
            Err(errors::AdventError::InvalidDayError)
        } else {
            Ok(Config { read_stdin, day, part, include_all })
        }
    }
}

pub fn run(matches: &ArgMatches) -> Result<(), errors::AdventError> {
    let config = Config::new(&matches)?;

    match config.day {
        None => {
            for d in 1..=NUM_PROBLEMS {
                if EXCLUDE_FROM_DEFAULT.contains(&d) && !config.include_all {
                    continue
                }
                match run_day(config.read_stdin, d, config.part) {
                    Ok(_) => (),
                    Err(e) => {
                        println!("Error on day {} with result {:?}", d, e);
                    }
                }
            }
        }
        Some(d) if d <= 25 => run_day(config.read_stdin, d, config.part)?,
        _ => panic!("Invalid day input"),
    }
    Ok(())
}

fn run_day(read_stdin: bool, day: u8, part: Option<char>) -> Result<(), errors::AdventError> {
    match day {
        n if 0 < n && n <= NUM_PROBLEMS => match part {
            None => {
                for p in ['a', 'b'].iter() {
                    run_part(read_stdin, day, *p)?;
                }
                Ok(())
            }
            Some(p) => run_part(read_stdin, day, p),
        },
        _ => Err(errors::AdventError::InvalidDayError),
    }
}

fn run_part(read_stdin: bool, day: u8, part: char) -> Result<(), errors::AdventError> {
    println!("Running problem {:0>2}{}:", day, part);

    match part {
        'a' | 'b' => {
            let solver = get_solver(day, part)?;
            let mut input = String::new();
            if read_stdin {
                match io::stdin().read_to_string(&mut input) {
                    Ok(0) | Err(_) => {
                        return Err(errors::AdventError::NoInput);
                    }
                    Ok(_) => (),
                }
            } else {
                let problem_str = format!("{:0>2}", day);
                let mut in_file = File::open("input/".to_string() + &problem_str)?;
                in_file.read_to_string(&mut input)?;
            }
            solver(&input)
        }
        _ => Err(errors::AdventError::InvalidPartError),
    }
}

type Solver = dyn Fn(&str) -> Result<(), errors::AdventError>;

fn get_solver(day: u8, part: char) -> Result<&'static Solver, errors::AdventError> {
    match (day, part) {
        (1, 'a') => Ok(&day01a::solve),
        (1, 'b') => Ok(&day01b::solve),
        (2, 'a') => Ok(&day02a::solve),
        (2, 'b') => Ok(&day02b::solve),
        (3, 'a') => Ok(&day03a::solve),
        (3, 'b') => Ok(&day03b::solve),
        (4, 'a') => Ok(&day04a::solve),
        (4, 'b') => Ok(&day04b::solve),
        (5, 'a') => Ok(&day05a::solve),
        (5, 'b') => Ok(&day05b::solve),
        (6, 'a') => Ok(&day06a::solve),
        (6, 'b') => Ok(&day06b::solve),
        (7, 'a') => Ok(&day07a::solve),
        (7, 'b') => Ok(&day07b::solve),
        (8, 'a') => Ok(&day08a::solve),
        (8, 'b') => Ok(&day08b::solve),
        (9, 'a') => Ok(&day09a::solve),
        (9, 'b') => Ok(&day09b::solve),
        (10, 'a') => Ok(&day10a::solve),
        (10, 'b') => Ok(&day10b::solve),
        (11, 'a') => Ok(&day11a::solve),
        (11, 'b') => Ok(&day11b::solve),
        (12, 'a') => Ok(&day12a::solve),
        (12, 'b') => Ok(&day12b::solve),
        (13, 'a') => Ok(&day13a::solve),
        (13, 'b') => Ok(&day13b::solve),
        (d, _p) if 0 < d && d <= NUM_PROBLEMS => Err(errors::AdventError::UnimplementedDayError),
        _ => Err(errors::AdventError::InvalidDayError),
    }
}

pub mod errors {
    use super::*;

    #[derive(Debug)]
    #[non_exhaustive]
    pub enum AdventError {
        InvalidDayError,
        UnimplementedDayError,
        NoInput,
        IoError(io::Error),
        ParseError,
        NoSolution,
        InvalidPartError,
        UnimplementedPartError,
    }

    impl fmt::Display for AdventError {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            match &self {
                Self::InvalidDayError => write!(f, "InvalidDayError"),
                Self::UnimplementedDayError => write!(f, "UnimplementedDayError"),
                Self::NoInput => write!(f, "NoInput"),
                Self::IoError(e) => write!(f, "{:?}", e),
                Self::ParseError => write!(f, "ParseError"),
                Self::NoSolution => write!(f, "NoSolution"),
                Self::InvalidPartError => write!(f, "InvalidPartError"),
                Self::UnimplementedPartError => write!(f, "UnimplementedPartError"),
            }
        }
    }

    impl From<io::Error> for AdventError {
        fn from(e: io::Error) -> Self {
            Self::IoError(e)
        }
    }

    impl From<std::num::ParseIntError> for AdventError {
        fn from(_e: std::num::ParseIntError) -> Self {
            Self::InvalidDayError
        }
    }

    impl From<std::char::ParseCharError> for AdventError {
        fn from(_e: std::char::ParseCharError) -> Self {
            Self::InvalidPartError
        }
    }
}

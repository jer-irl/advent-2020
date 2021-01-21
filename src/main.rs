extern crate atty;
extern crate bitvec;
extern crate clap;
extern crate itertools;
extern crate regex;

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

use clap::{App, Arg};
use std::fmt;
use std::fs::File;
use std::io;
use std::io::Read;

const NUM_PROBLEMS: u8 = 25;

fn main() -> Result<(), errors::AdventError> {
    let matches = App::new("Advent of Code 2020")
        .author("Jeremy Schroeder <jpschroeder2014@gmail.com>")
        .about("Simple rust solutions to AoC2020 to help me get better at rust.")
        .arg(
            Arg::with_name("DAY")
                .help("The day index to run")
                .index(1)
                .default_value("0"),
        )
        .arg(
            Arg::with_name("PART")
                .help("The part to run")
                .index(2)
                .default_value("0"),
        )
        .arg(
            Arg::with_name("stdin")
                .help("Pass if input is read from stdin")
                .takes_value(false),
        )
        .get_matches();

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

    assert!(!(day == None && read_stdin));
    assert!(!(part == None && read_stdin));
    match day {
        None => {
            for d in 1..=NUM_PROBLEMS {
                match run_day(read_stdin, d, part) {
                    Ok(_) => (),
                    Err(e) => {
                        println!("Error on day {} with result {:?}", d, e);
                    }
                }
            }
        }
        Some(d) if d <= 25 => run_day(read_stdin, d, part)?,
        _ => panic!("Invalid day input"),
    }

    println!("All done!");

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
        }
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

fn get_solver(day: u8, part: char) -> Result<Box<Solver>, errors::AdventError> {
    match (day, part) {
        (1, 'a') => Ok(Box::from(day01a::solve)),
        (1, 'b') => Ok(Box::from(day01b::solve)),
        (2, 'a') => Ok(Box::from(day02a::solve)),
        (2, 'b') => Ok(Box::from(day02b::solve)),
        (3, 'a') => Ok(Box::from(day03a::solve)),
        (3, 'b') => Ok(Box::from(day03b::solve)),
        (4, 'a') => Ok(Box::from(day04a::solve)),
        (4, 'b') => Ok(Box::from(day04b::solve)),
        (5, 'a') => Ok(Box::from(day05a::solve)),
        (5, 'b') => Ok(Box::from(day05b::solve)),
        (6, 'a') => Ok(Box::from(day06a::solve)),
        (6, 'b') => Ok(Box::from(day06b::solve)),
        (7, 'a') => Ok(Box::from(day07a::solve)),
        (7, 'b') => Ok(Box::from(day07b::solve)),
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

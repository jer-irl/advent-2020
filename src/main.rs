extern crate clap;
extern crate atty;

mod day01;

use clap::{App, Arg};
use std::io;
use std::io::Read;
use std::fmt;
use std::fs::File;

const NUM_IMPLEMENTED: u8 = 1;
const NUM_PROBLEMS: u8 = 25;

fn main() -> Result<(), errors::AdventError>{
    let matches = App::new("Advent of Code 2020")
        .author("Jeremy Schroeder <jpschroeder2014@gmail.com>")
        .about("Simple rust solutions to AoC2020 to help me get better at rust.")
        .arg(
            Arg::with_name("DAY")
                .help("The day index to run")
                .index(1)
                .default_value("0")
        )
        .arg(
            Arg::with_name("stdin")
                .help("Pass if input is read from stdin")
                .takes_value(false)
        )
        .get_matches();

    let read_stdin = matches.is_present("stdin");
    let day: u8 = matches.value_of("DAY").unwrap_or("00").parse()?;

    assert!(!(read_stdin && day == 0));
    match day {
        0 => {
            for d in 1..26 {
                match run_day(read_stdin, d) {
                    Ok(_) => (),
                    Err(e) => {
                        println!("Error on day {} with exception {:?}", d, e);
                        break;
                    },
                }
            }
        },
        d if d <= 25 => run_day(read_stdin, d)?,
        _ => panic!("Invalid day input"),
    }

    println!("All done!");

    Ok(())
}

fn run_day(read_stdin: bool, day: u8) -> Result<(), errors::AdventError> {
    println!("Running day {:0>2}:", day);

    match day {
        n if 0 < n && n <= NUM_IMPLEMENTED => {
            let mut input = String::new();
            if read_stdin {
                match io::stdin().read_to_string(&mut input) {
                    Ok(0) | Err(_) => {
                        return Err(errors::AdventError::NoInput);
                    },
                    Ok(_) => (),
                }
            } else {
                let day_str = format!("{:0>2}", day);
                let mut in_file = File::open("input/".to_string() + &day_str)?;
                in_file.read_to_string(&mut input)?;
            }
            get_solver(day)?(&input)
        },
        n if 0 < n && n <= NUM_PROBLEMS => Err(errors::AdventError::UnimplementedDayError),
        _ => Err(errors::AdventError::InvalidDayError),
    }
}

type Solver = dyn Fn(&str) -> Result<(), errors::AdventError>;

fn get_solver(day: u8) -> Result<Box<Solver>, errors::AdventError> {
    match day {
        1 => Ok(Box::from(day01::solve)),
        n if 0 < n && n <= NUM_IMPLEMENTED => unreachable!("Coding error"),
        n if 0 < n && n <= NUM_PROBLEMS => Err(errors::AdventError::UnimplementedDayError),
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
    }

    impl fmt::Display for AdventError {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            match &self {
                Self::InvalidDayError => write!(f, "InvalidDayError"),
                Self::UnimplementedDayError => write!(f, "UnimplementedDayError"),
                Self::NoInput => write!(f, "NoInput"),
                Self::IoError(e) => write!(f, "{:?}", e),
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
}

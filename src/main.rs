use clap::{App, Arg};

fn main() {
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
        .arg(
            Arg::with_name("include-all")
                .help("Force all problems to run, including those excluded by default.")
                .takes_value(false),
        )
        .get_matches();

    advent_2020::run(&matches).unwrap_or_else(|_| {
        println!("Unsuccessful run");
        std::process::exit(1);
    });

    println!("All done!");
}

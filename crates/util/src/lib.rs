use std::path::{Path, PathBuf};
use clap::{App, Arg};

/* Provides the standard CLI I'm using for most of the
 * 2021 Advent-of-Code puzzles.  Returns a path to the
 * test*/
pub fn advent_cli(puzzle_name: &str, day: u32) -> PathBuf {
     let title = puzzle_name;
     let about = format!("Advent of Code 2021 - Day {}: {}", day, title).to_owned();
     let argmatches = App::new(title)
        .version("0.1")
        .author("Jack Bradach <jack@bradach.net>")
        .about(&about[..])
        .arg(Arg::new("input")
            .about("Input dataset from website")
            .index(1)
            .required(true))
        .get_matches();

    let input = match argmatches.value_of("input") {
        Some(input) => Path::new(input),
        None => {
            eprintln!("No input data file specified!");
            std::process::exit(1);
        }
    };
    input.to_owned()
}

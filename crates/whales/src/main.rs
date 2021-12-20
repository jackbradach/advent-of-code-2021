/*
 * https://adventofcode.com/2021/day/7
 */

use std::path::{Path};
use std::io::{BufReader, BufRead};
use std::fs::File;

use clap::{App, Arg};

#[derive(Clone, Debug)]
struct CrabSub {
    position: u32,
}

#[derive(Debug)]
struct SubPositions {
    positions: Vec<CrabSub>
}

impl SubPositions {

    fn from_file(input: &Path) -> SubPositions {
        let file = File::open(input).unwrap();
        let position_string=
            BufReader::new(file)
                .lines()
                .map(|s| s.unwrap())
                .collect();
        SubPositions::new(&position_string)
    }

    fn new(text: &String) -> SubPositions {
        let positions: Vec<CrabSub> =
            text.split(",")
            .collect::<Vec<&str>>()
            .iter()
            .map(|s| CrabSub { position: s.parse::<u32>().unwrap() })
            .collect();
        // FIXME: need to convert crab locations to how many crabs
        // FIXME: are at each position.
        SubPositions {
            positions,
        }
    }

    fn maximum_position(&self) -> u32 {
        let mut max_position = 0;
        for sub in &self.positions {
            if sub.position > max_position {
                max_position = sub.position;
            }
        }
        max_position
    }

    fn find_minimal_fuel_constant_burn(&self) -> u32 {
        let max_position = self.maximum_position();
        let mut min_fuel: Option<i32> = None;
        for i in 0..(max_position + 1) {
            let mut fuel_cost:i32 = 0;
            for sub in &self.positions {
                fuel_cost += (sub.position as i32 - i as i32).abs();
            }
            if min_fuel.is_none() || (min_fuel.is_some() && fuel_cost < min_fuel.unwrap()) {
                min_fuel = Some(fuel_cost);
            }
        }
        min_fuel.unwrap() as u32
    }

    // FIXME - this could be way faster, but the naive implementation
    // FIXME - does arrive at a solution after a couple seconds.
    // FIXME - Good enough for a puzzle solution!
    // FIXME - maybe Gauss' method?
    fn find_minimal_fuel_variable_burn(&self) -> u32 {
        let max_position = self.maximum_position();
        let mut min_fuel: Option<i32> = None;
        for i in 0..(max_position + 1) {
            let mut fuel_cost:i32 = 0;
            for sub in &self.positions {
                let distance = (sub.position as i32 - i as i32).abs();
                for j in 0..distance {
                    fuel_cost += j + 1;
                }
            }
            if min_fuel.is_none() || (min_fuel.is_some() && fuel_cost < min_fuel.unwrap()) {
                min_fuel = Some(fuel_cost);
            }
        }
        min_fuel.unwrap() as u32
    }
}

fn main() {
    // Argument parsing using the Clap builder pattern.
    let argmatches = App::new("The Treachery of Whales")
        .version("0.1")
        .author("Jack Bradach <jack@bradach.net>")
        .about("Advent of Code 2021 - Day 7: The Treachery of Whales")
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

    let positions = SubPositions::from_file(&input);
    let min_fuel = positions.find_minimal_fuel_constant_burn();
    println!("Part 1: Minimum fuel: {}", min_fuel);

    let min_fuel = positions.find_minimal_fuel_variable_burn();
    println!("Part 2: Minimum fuel: {}", min_fuel);
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::{PathBuf};
    
    #[test]
    fn test_whales_part1() {
        const MINIMUM_FUEL: u32 = 37;
        let mut input = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        input.push("whales_test.txt");
        let positions = SubPositions::from_file(&input);
        let min_fuel = positions.find_minimal_fuel_constant_burn();
        assert_eq!(min_fuel, MINIMUM_FUEL);
    }

    #[test]
    fn test_whales_part2() {
        const MINIMUM_FUEL: u32 = 168;
        let mut input = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        input.push("whales_test.txt");
        let positions = SubPositions::from_file(&input);
        let min_fuel = positions.find_minimal_fuel_variable_burn();
        assert_eq!(min_fuel, MINIMUM_FUEL);
    }
}
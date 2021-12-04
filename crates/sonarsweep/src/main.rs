/*
 * https://adventofcode.com/2021/day/1
 */
use clap::{App, Arg};
use std::path::Path;
use std::io::{BufReader, BufRead};
use std::fs::File;

struct SonarSweep {
    depths: Vec<u32>
}

impl SonarSweep {
    fn from_file(input: &Path) -> SonarSweep {
        let file = File::open(input).unwrap();
        let depths =
            BufReader::new(file)
                .lines()
                .map(|s| s.unwrap().parse::<u32>().unwrap())
                .collect();
        SonarSweep {
            depths,
        }
    }

    /* Counts the number of times a depth measurement increases
     * from the previous measurement.
     */
    fn increase_count(&self) -> u32 {
        let mut increase_count = 0;
        let mut last_depth: Option<u32> = None;
        for depth in &self.depths {
            if let Some(last_depth) = last_depth {
                if depth > &last_depth {
                    increase_count += 1;
                }    
            }
            last_depth = Some(*depth);
        }
        increase_count
    }
}

fn main() {
    // Argument parsing using the Clap builder pattern.
    let argmatches = App::new("sonarsweep")
        .version("0.1")
        .author("Jack Bradach <jack@bradach.net>")
        .about("Advent of Code 2021 - Day 1: Sonar Sweep")
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

    let sweep = SonarSweep::from_file(input);
    println!("depth increases: {}", sweep.increase_count());
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_sonarsweep_count_depth_increase() {
        /* The dataset and increase count were given on the webpage. */
        const INCREASE_COUNT: u32 = 7;
        let depths = vec![
            199, 200, 208, 210, 200,
            207, 240, 269, 260, 263
        ];

        let sweep = SonarSweep { depths, };
        assert_eq!(sweep.increase_count(), INCREASE_COUNT);
    }
}
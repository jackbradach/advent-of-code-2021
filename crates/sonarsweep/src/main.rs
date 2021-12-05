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
        self.increase_count_sliding_window(1)
    }

    
    fn increase_count_sliding_window(&self, window_size: usize) -> u32 {
        let mut increase_count = 0;
        let mut chunks: Vec<u32> = Vec::new();

        for i in 0..(self.depths.len() - (window_size - 1)) {
            let mut sum = 0;
            for j in 0..window_size {
                sum += self.depths[i+j];
            }
            chunks.push(sum);
        }

        let mut last: Option<u32> = None;
        for chunk in chunks {
            if let Some(last) = last {
                if chunk > last {
                    increase_count += 1;
                }    
            }
            last = Some(chunk);
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
    println!("Part 1: depth increases = {}", sweep.increase_count());
    println!("Part 2: depth increases = {}", sweep.increase_count_sliding_window(3));
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

    #[test]
    fn test_sonarsweep_count_depth_increase_sliding_window() {
        /* This one tests the result with the sliding window. */
        const INCREASE_COUNT: u32 = 5;
        const WINDOW_SIZE: usize = 3;
        let depths = vec![
            199, 200, 208, 210, 200,
            207, 240, 269, 260, 263
        ];

        let sweep = SonarSweep { depths, };
        assert_eq!(sweep.increase_count_sliding_window(WINDOW_SIZE), INCREASE_COUNT);
    }
}
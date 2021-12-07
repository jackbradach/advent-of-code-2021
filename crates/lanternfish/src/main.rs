/*
 * https://adventofcode.com/2021/day/4
 */
use clap::{App, Arg};
use std::path::Path;
use std::io::{BufReader, BufRead};
use std::fs::File;


// FIXME - jbradach - should change implementation so we're tracking number of fish in
// FIXME - each counter state, so there's only 8 bins.
// 
// Each bin in the Vec represents how many fish in the population have a particular
// counter.  Each round, pop the head off (shifting everyone's counter down by 1),
// Add this value to the now counter[6] and push the new value to the end of the
// vec (bringing us back up to 8 elements) 

#[derive(Clone, Debug)]
struct Lanternfish {
    counter: Vec<u64>
}

impl Lanternfish {

    fn from_file(input: &Path) -> Lanternfish {
        let file = File::open(input).unwrap();
        let population: Vec<String> =
            BufReader::new(file)
                .lines()
                .map(|s| s.unwrap())
                .collect();
        Lanternfish::from_string(&population[0])
    }

    fn from_string(population: &String) -> Lanternfish {
        let initial_population: Vec<u32> =
            population
                .split(",")
                .map(|v| v.parse::<u32>().unwrap())
                .collect();
        let mut lanternfish = Lanternfish { counter: Vec::new() };
        for _ in 0..9 {
            lanternfish.counter.push(0);
        }

        for v in initial_population {
            lanternfish.counter[v as usize] += 1;
        }
        lanternfish
    }

    fn population_after_day(&self, day: u32) -> u64 {
        let mut fish = self.counter.clone();

        for _ in 0..day {
            let spawn_count = fish.remove(0);
            fish[6] += spawn_count;
            fish.push(spawn_count);
        }
        let mut population: u64 = 0;
        for f in fish {
            population += f as u64;
        }
        population
    }
}

fn main() {
    // Argument parsing using the Clap builder pattern.
    let argmatches = App::new("Lanternfish")
        .version("0.1")
        .author("Jack Bradach <jack@bradach.net>")
        .about("Advent of Code 2021 - Day 6: Lanternfish")
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

    let fishes = Lanternfish::from_file(input);
    println!("Part 1: Population after 80 days = {}", fishes.population_after_day(80));
    println!("Part 2: Population after 256 days = {}", fishes.population_after_day(256));
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_lanternfish_part1() {
        const DAY_A: u32 = 18;
        const EXPECTED_FISH_COUNT_A: u64 = 26;
        const DAY_B: u32 = 80;
        const EXPECTED_FISH_COUNT_B: u64 = 5934;
        let fish = "3,4,3,1,2".to_string();
        let fishes = Lanternfish::from_string(&fish);
        assert_eq!(fishes.population_after_day(DAY_A), EXPECTED_FISH_COUNT_A);
        assert_eq!(fishes.population_after_day(DAY_B), EXPECTED_FISH_COUNT_B);
    }

    // #[test]
    // fn test_lanternfish_part2() {
    //     const DAY: u32 = 256;
    //     const EXPECTED_FISH_COUNT: u64 = 26984457539;
    //     let fish = "3,4,3,1,2".to_string();
    //     let fishes = Lanternfishes::from_string(&fish);
    //     assert_eq!(fishes.population_after_day(DAY), EXPECTED_FISH_COUNT);
    // }


}
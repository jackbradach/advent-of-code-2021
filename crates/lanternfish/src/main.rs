/*
 * https://adventofcode.com/2021/day/4
 */
use clap::{App, Arg};
use std::path::Path;
use std::io::{BufReader, BufRead};
use std::fs::File;


#[derive(Clone, Debug)]
struct Lanternfish {
    counter: u32
}

#[derive(Clone, Debug)]
struct Lanternfishes {
    fish: Vec<Lanternfish>
}

impl Lanternfishes {

    fn from_file(input: &Path) -> Lanternfishes {
        let file = File::open(input).unwrap();
        let population: Vec<String> =
            BufReader::new(file)
                .lines()
                .map(|s| s.unwrap())
                .collect();
        Lanternfishes::from_string(&population[0])
    }

    fn from_string(population: &String) -> Lanternfishes {
        let lanternfishes: Vec<Lanternfish> =
            population
                .split(",")
                .map(|counter| Lanternfish { counter: counter.parse::<u32>().unwrap() })
                .collect();
        Lanternfishes {
            fish: lanternfishes,
        }
    }

    fn population_after_day(&self, day: u32) -> u64 {
        let mut fish = self.fish.clone();

        for _ in 0..day {
            let mut add_fish: u32 = 0;
            for mut f in &mut fish {
                match f.counter {
                    0 => {
                        add_fish += 1;
                        f.counter = 6;
                    },
                    _ => {
                        f.counter -= 1;
                    }
                }
            }
            if add_fish > 0 {
                for _ in 0..add_fish {
                    fish.push(Lanternfish{ counter: 8});
                }
            }
        }
        fish.len() as u64
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

    let fishes = Lanternfishes::from_file(input);
    println!("Part 1: Population after 80 days = {}", fishes.population_after_day(80));
    // println!("Part 1: Population after 256 days = {}", fishes.population_after_day(256));
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_lanternfish_part1() {
        const DAY: u32 = 80;
        const EXPECTED_FISH_COUNT: u64 = 5934;
        let fish = "3,4,3,1,2".to_string();
        let fishes = Lanternfishes::from_string(&fish);
        assert_eq!(fishes.population_after_day(DAY), EXPECTED_FISH_COUNT);
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
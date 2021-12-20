/*
 * https://adventofcode.com/2021/day/6
 */
use std::path::Path;
use std::io::{BufReader, BufRead};
use std::fs::File;

use util;

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
    let input = util::advent_cli("Lanternfish", 6);
    let fishes = Lanternfish::from_file(&input);
    println!("Part 1: Population after 80 days = {}", fishes.population_after_day(80));
    println!("Part 2: Population after 256 days = {}", fishes.population_after_day(256));
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_lanternfish_part1() {
        const DAY: u32 = 18;
        const EXPECTED_FISH_COUNT: u64 = 26;
        let fish = "3,4,3,1,2".to_string();
        let fishes = Lanternfish::from_string(&fish);
        assert_eq!(fishes.population_after_day(DAY), EXPECTED_FISH_COUNT);
    }

    #[test]
    fn test_lanternfish_part2() {
        const DAY: u32 = 256;
        const EXPECTED_FISH_COUNT: u64 = 26984457539;
        let fish = "3,4,3,1,2".to_string();
        let fishes = Lanternfish::from_string(&fish);
        assert_eq!(fishes.population_after_day(DAY), EXPECTED_FISH_COUNT);
    }
}
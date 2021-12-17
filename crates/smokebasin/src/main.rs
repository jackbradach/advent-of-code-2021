/*
 * https://adventofcode.com/2021/day/8
 */

use std::path::{Path};
use std::io::{BufReader, BufRead};
use std::fs::File;
use std::fmt;
use std::convert::TryInto;
use clap::{App, Arg};

#[derive(Debug)]
struct Cell {
    x: usize,
    y: usize,
}

#[derive(Debug)]
struct HeightMap {
    width: usize,
    length: usize,
    heights: Vec<u8>
}

impl HeightMap {

    fn new(input: &Path) -> HeightMap {
        let file = File::open(input).unwrap();
        let ventline_strings: Vec<String> =
            BufReader::new(file)
                .lines()
                .map(|s| s.unwrap())
                .collect();
        let width = ventline_strings[0].len();
        let length = ventline_strings.len();
        let mut heights: Vec<u8> = Vec::with_capacity(width * length);
        for line in ventline_strings.iter() {
            let line_len = line.len();
            for j in 0..line_len {
                let v = line.chars().nth(j).unwrap().to_digit(10).unwrap();
                heights.push(v as u8);
            }
        }
        HeightMap {
            width,
            length,
            heights
        }
    }

    fn risk_level(&self) -> u32 {
        let cells = self.get_lowest_points();
        let mut risk_level: u32 = 0;
        for cell in cells {
            let v = self.get_cell(cell.x as i32, cell.y as i32).unwrap();
            risk_level += v as u32 + 1;
        }
        risk_level
    }

    /* Iterate over the vector, checking each cell to
     * see if it's less than it's cardinal direction neighbors.
     */
    fn get_lowest_points(&self) -> Vec<Cell> {
        let mut cells: Vec<Cell> = Vec::new();
        for i in 0..self.heights.len() {
            let x: i32 = (i % self.width).try_into().unwrap();
            let y: i32 = (i / self.width).try_into().unwrap();
            if self.check_low_point(x, y) {
                cells.push(Cell { x: x as usize, y: y as usize, });
            }
        }
        cells
    }

    fn check_low_point(&self, x: i32, y: i32) -> bool {
        let cell = self.get_cell(x, y).unwrap();
        let north = self.get_cell(x, y - 1);
        let east = self.get_cell(x + 1, y);
        let west = self.get_cell(x - 1, y);
        let south = self.get_cell(x, y + 1);
        let mut low_point: bool = true;
        
        if north.is_some() && !(cell < north.unwrap()) {
            low_point = false;
        }

        if east.is_some() && !(cell < east.unwrap()) {
            low_point = false;
        }

        if west.is_some() && !(cell < west.unwrap()) {
            low_point = false;
        }

        if south.is_some() && !(cell < south.unwrap()) {
            low_point = false;
        }
        low_point
    }

    fn get_cell(&self, x: i32, y: i32) -> Option<u8> {
        let width: i32 = self.width as i32;

        if x < 0 || x >= width {
            return None;
        }

        let length: i32 = self.length as i32;
        if y < 0 || y >= length {
            return None;
        }
        Some(self.heights[((y * width) + x) as usize])
    }

    /* For a given cell, find all the basins */
    fn find_basin(&self, cell: &Cell) {

    }
}

impl fmt::Display for HeightMap {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for y in 0..self.length {
            for x in 0..self.width {
                write!(f, "{}", self.heights[(y * self.width) + x])?
            }
            write!(f, "\n")?
        }
        write!(f, "\n")
    }
}

fn main() {
    // Argument parsing using the Clap builder pattern.
    let argmatches = App::new("Smoke Basin")
        .version("0.1")
        .author("Jack Bradach <jack@bradach.net>")
        .about("Advent of Code 2021 - Day 8: Smoke Basin")
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

    let heightmap = HeightMap::new(&input);
    println!("Part 1: Risk Level = {}", heightmap.risk_level());
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::{PathBuf};
    
    #[test]
    fn test_smokebasin_part1() {
        const RISK_LEVEL: u32 = 15;
        let mut input = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        input.push("smokebasin_test.txt");
        let heightmap = HeightMap::new(&input);
        assert_eq!(heightmap.risk_level(), RISK_LEVEL);
    }

    // #[test]
    // fn test_vents_part2() {
    //     const OVERLAPPING_VENT_THRESHOLD: i32 = 2;
    //     const OVERLAPPING_VENT_COUNT: i32 = 12;
    //     let mut input = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    //     input.push("vents_test.txt");
    //     let vents_map = VentsMap::from_file(&input, true);
    //     println!("{}", vents_map);
    //     assert_eq!(vents_map.overlapping_vent_count(OVERLAPPING_VENT_THRESHOLD), OVERLAPPING_VENT_COUNT);
    // }

    
  
}
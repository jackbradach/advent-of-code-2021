/*
 * https://adventofcode.com/2021/day/5
 */

use std::collections::HashMap;
use std::cmp::Ordering;
use std::path::{Path};
use std::io::{BufReader, BufRead};
use std::fs::File;
use std::fmt;
use std::cmp;

use clap::{App, Arg};
use colored::{ColoredString, Colorize};

#[derive(Clone, Copy, Debug, Eq, Hash)]
struct Vent {
    x: u32,
    y: u32,
}

impl PartialEq for Vent {
    fn eq(&self, other: &Self) -> bool {
        if self.x == other.x && self.y == other.y {
            true
        } else {
            false
        }
    }
}

/* Line from point 1 -> point 2 */
#[derive(Clone, Copy, Debug)]
struct VentLine {
    x1: u32,
    y1: u32,
    x2: u32,
    y2: u32,
}

impl VentLine {
    fn from_string(text: &String) -> VentLine {
        // 0,9 -> 5,9
        let v: Vec<&str> = text.split(" -> ").collect();
        let origin: Vec<u32> = v[0].split(",").map(|v| v.parse::<u32>().unwrap()).collect();
        let ending: Vec<u32> = v[1].split(",").map(|v| v.parse::<u32>().unwrap()).collect();
        VentLine {
            x1: origin[0],
            y1: origin[1],
            x2: ending[0],
            y2: ending[1],
        }
    }
}

#[derive(Debug)]
struct VentsMap {
    map: HashMap<Vent, u32>
}

impl VentsMap {
    fn from_file(input: &Path, use_diagonals: bool) -> VentsMap {
        let file = File::open(input).unwrap();
        let ventline_strings: Vec<String> =
            BufReader::new(file)
                .lines()
                .map(|s| s.unwrap())
                .collect();
        VentsMap::new(ventline_strings, use_diagonals)
    }

    fn new(ventline_strings: Vec<String>, use_diagonals: bool) -> VentsMap {
        // Convert ventline strings into ventline objects
        let ventlines: Vec<VentLine> =
            ventline_strings.iter()
            .map(|v| VentLine::from_string(v))
            .collect();
            
            let mut ventmap = VentsMap {
                map: HashMap::new()
            };
            
            for ventline in ventlines {
                ventmap.apply_line(&ventline, use_diagonals);
            }
        // println!("ventmap: {:?}", ventmap);        
        ventmap
    }

    /* We're guaranteed that all ventlines will be */
    fn apply_line(&mut self, ventline: &VentLine, use_diagonals: bool) {
        // Determine if we're a horizontal or vertical line,
        // call appropriate function to apply.
        if ventline.y1 == ventline.y2 {
            self.apply_horiz(ventline);
        } else if ventline.x1 == ventline.x2 {
            self.apply_vert(ventline);
        } else {
            // println!("Skipping diagonal line: {:?}", ventline);
            // let vent = Vent{ ventline.x1, ventline.x2, };
            // self.map.insert(vent, v);
        }
    }

    // FIXME - need to handle lines going from large->small direction.

    fn apply_horiz(&mut self, ventline: &VentLine) {
        let y = ventline.y1;
        let begin = cmp::min(ventline.x1, ventline.x2);
        let end = cmp::max(ventline.x1, ventline.x2);
        // println!("Drawing horizontal line y={} from x1={} to x2={}",
        //     y, begin, end);
        for x in begin..(end + 1) {
            let vent = Vent{ x, y, };
            self.increment_vent(&vent);
        }
    }

    fn apply_vert(&mut self, ventline: &VentLine) {
        let x = ventline.x1;
        let begin = cmp::min(ventline.y1, ventline.y2);
        let end = cmp::max(ventline.y1, ventline.y2);
        // println!("Drawing vertical line x={} from y1={} to y2={}",
            // x, begin, end);
        for y in begin..(end + 1) {
            let vent = Vent{ x, y, };
            self.increment_vent(&vent);
        }
    }

    fn increment_vent(&mut self, vent: &Vent) {
        if let Some(v) = self.map.get_mut(vent) {
            *v += 1;
            // println!("({}, {}) = {}", vent.x, vent.y, *v);
        } else {
            let v: u32 = 1;
            self.map.insert(vent.clone(), v);
            // println!("({}, {}) = {}", vent.x, vent.y, v);
        }
    }

    fn get_max_corner(&self) -> Vent {
        let mut max_vent = Vent { x: 0, y: 0, };
        for vent in self.map.iter() {
            if vent.0.x > max_vent.x {
                max_vent.x = vent.0.x;
            }
            if vent.0.y > max_vent.y {
                max_vent.y = vent.0.y;
            }
        }
        // println!("max_vent = {:?}", max_vent);
        max_vent
    }

    fn overlapping_vent_count(&self, threshold: u32) -> u32 {
        self.map.iter().filter(|v| v.1 >= &threshold).count() as u32
    }
}

impl fmt::Display for VentsMap {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let max_corner = self.get_max_corner();
        for y in 0..max_corner.y + 1 {
            for x in 0..max_corner.x + 1 {
                let vent = Vent { x, y };
                if let Some(v) = self.map.get(&vent) {
                    write!(f, "{}", v)?
                } else {
                    write!(f, ".")?
                }
            }
            write!(f, "\n")?
        }
        write!(f, "\n")
    }
}

impl fmt::Display for VentLine {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{},{} -> {},{}", self.x1, self.y1, self.x2, self.y2)
    }
}

fn main() {
    // Argument parsing using the Clap builder pattern.
    let argmatches = App::new("Hydrothermal Vents")
        .version("0.1")
        .author("Jack Bradach <jack@bradach.net>")
        .about("Advent of Code 2021 - Day 5: Hydrothermal Vents")
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

    const OVERLAPPING_VENT_THRESHOLD: u32 = 2;
    let vents_map = VentsMap::from_file(&input, false);
    // println!("{}", vents_map);
    println!("Part 1: Overlapping vent count: {}", vents_map.overlapping_vent_count(OVERLAPPING_VENT_THRESHOLD));
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::{PathBuf};
    
    #[test]
    fn test_ventline_from_string() {
        let text = "0,2 -> 9,4".to_string();
        let ventline = VentLine::from_string(&text);
        assert_eq!(ventline.x1, 0);
        assert_eq!(ventline.y1, 2);
        assert_eq!(ventline.x2, 9);
        assert_eq!(ventline.y2, 4);
    }
    
    #[test]
    fn test_vents_part1() {
        const OVERLAPPING_VENT_THRESHOLD: u32 = 2;
        const OVERLAPPING_VENT_COUNT: u32 = 5;
        let mut input = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        input.push("vents_test.txt");
        let vents_map = VentsMap::from_file(&input, false);
        println!("{}", vents_map);
        assert_eq!(vents_map.overlapping_vent_count(OVERLAPPING_VENT_THRESHOLD), OVERLAPPING_VENT_COUNT);
    }

    
  
}
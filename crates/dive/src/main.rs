/*
 * https://adventofcode.com/2021/day/2
 */
use clap::{App, Arg};
use std::path::Path;
use std::io::{BufReader, BufRead};
use std::fs::File;

#[derive(Clone, Copy, Debug)]
enum SubmarineDirection {
    Forward,
    Down,
    Up
}

impl SubmarineDirection {
    fn from_str(direction: &str) -> SubmarineDirection {
        match direction {
            "forward" => SubmarineDirection::Forward,
            "down" => SubmarineDirection::Down,
            "up" => SubmarineDirection::Up,
            _ => panic!("Bad SubmarineDirection: {}", direction),
        }
    }
}

#[derive(Clone, Copy, Debug)]
struct SubmarineCommand {
    direction: SubmarineDirection,
    distance: u32,
}

impl SubmarineCommand {
    fn from_str(command: &str) -> SubmarineCommand {
        let command_tuple: Vec<&str> = command.split_whitespace().collect();
        let direction: SubmarineDirection = SubmarineDirection::from_str(command_tuple[0]);
        let distance = command_tuple[1].parse::<u32>().unwrap();
        SubmarineCommand {
            direction,
            distance,
        }
    }
}

#[derive(Clone, Debug)]
struct SubmarineCommands {
    commands: Vec<SubmarineCommand>
}

impl SubmarineCommands {
    fn from_file(input: &Path) -> SubmarineCommands {
        let file = File::open(input).unwrap();
        let commands = BufReader::new(file).lines().filter_map(|s| s.ok()).collect();
        SubmarineCommands::from_strs(&commands)
    }

    fn from_strs(command_strs: &Vec<String>) -> SubmarineCommands {
        let mut submarine_commands: Vec<SubmarineCommand> = Vec::new();
        for command in command_strs {
            submarine_commands.push(
                SubmarineCommand::from_str(&command)
            );
        }
        SubmarineCommands {
            commands: submarine_commands
        }
    }

}

impl IntoIterator for &SubmarineCommands {
    type Item = SubmarineCommand;
    type IntoIter = std::vec::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        self.commands.clone().into_iter()
    }
}

struct Submarine {
    aim: u32,
    pos_horizontal: u32,
    pos_depth: u32,
}


impl Submarine {
    fn new() -> Submarine {
        Submarine {
            aim: 0,
            pos_horizontal: 0,
            pos_depth: 0,
        }
    }

    /* Part 1 has us apply commands using a misunderstanding of how the
     * Submarine works.  I left it in so I can still print out both
     * part's solutions.
     */
    fn apply_commands_wrong(&mut self, commands: &SubmarineCommands) {
        for command in commands {
            match command.direction {
                SubmarineDirection::Forward => {
                    self.pos_horizontal += command.distance;
                },
                SubmarineDirection::Up => {
                    self.pos_depth -= command.distance;
                },
                SubmarineDirection::Down => {
                    self.pos_depth += command.distance;
                },
            }
        }
    }

    fn apply_commands(&mut self, commands: &SubmarineCommands) {
        for command in commands {
            match command.direction {
                SubmarineDirection::Forward => {
                    self.pos_horizontal += command.distance;
                    self.pos_depth += command.distance * self.aim;
                },
                SubmarineDirection::Up => {
                    self.aim -= command.distance;
                },
                SubmarineDirection::Down => {
                    self.aim += command.distance;
                },
            }
        }
    }

    fn position_horizontal(&self) -> u32 {
        self.pos_horizontal
    }

    fn position_depth(&self) -> u32 {
        self.pos_depth
    }
}

fn main() {
    // Argument parsing using the Clap builder pattern.
    let argmatches = App::new("Dive!")
        .version("0.1")
        .author("Jack Bradach <jack@bradach.net>")
        .about("Advent of Code 2021 - Day 2: Dive!")
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

    let mut submarine = Submarine::new();
    let submarine_commands = SubmarineCommands::from_file(input);
    submarine.apply_commands_wrong(&submarine_commands);
    println!("Part 1: x({}) * y({}) = {}",
        submarine.position_horizontal(),
        submarine.position_depth(),
        submarine.position_horizontal() * submarine.position_depth()
    );

    let mut submarine = Submarine::new();
    submarine.apply_commands(&submarine_commands);
    println!("Part 2: x({}) * y({}) = {}",
        submarine.position_horizontal(),
        submarine.position_depth(),
        submarine.position_horizontal() * submarine.position_depth()
    );
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_dive_part1() {
        /* The dataset and increase count were given on the webpage. */
        const POSITION_HORIZONTAL: u32 = 15;
        const POSITION_DEPTH: u32 = 10;
        let commands = vec![
            "forward 5".to_string(),
            "down 5".to_string(),
            "forward 8".to_string(),
            "up 3".to_string(),
            "down 8".to_string(),
            "forward 2".to_string(),
        ];
        let mut submarine = Submarine::new();
        let commands = SubmarineCommands::from_strs(&commands);
        submarine.apply_commands_wrong(&commands);
        assert_eq!(submarine.position_horizontal(), POSITION_HORIZONTAL);
        assert_eq!(submarine.position_depth(), POSITION_DEPTH);
    }

    #[test]
    fn test_dive_part2() {
        /* The dataset and increase count were given on the webpage. */
        const POSITION_HORIZONTAL: u32 = 15;
        const POSITION_DEPTH: u32 = 60;
        let commands = vec![
            "forward 5".to_string(),
            "down 5".to_string(),
            "forward 8".to_string(),
            "up 3".to_string(),
            "down 8".to_string(),
            "forward 2".to_string(),
        ];

        let mut submarine = Submarine::new();
        let commands = SubmarineCommands::from_strs(&commands);
        submarine.apply_commands(&commands);
        assert_eq!(submarine.position_horizontal(), POSITION_HORIZONTAL);
        assert_eq!(submarine.position_depth(), POSITION_DEPTH);
    }
}
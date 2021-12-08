/*
 * https://adventofcode.com/2021/day/4
 */

use std::path::{Path, PathBuf};
use std::io::{BufReader, BufRead};
use std::fs::File;
use std::fs;
use std::fmt;

use clap::{App, Arg};
use colored::{ColoredString, Colorize};

#[derive(Clone, Copy, Debug)]
struct BingoCard {
    card: [[u8; 5]; 5],
    marked: [[bool; 5]; 5],
}

impl BingoCard {

    fn new(card_raw: Vec<String>) -> BingoCard {
        let mut card: [[u8; 5]; 5] = [[0; 5]; 5];
        for y in 0..5 {
            let vals: Vec<u8> =
                card_raw[y]
                .split_whitespace()
                .map(|s| s.parse::<u8>().unwrap())
                .collect();
            for x in 0..5 {
                card[y][x] = vals[x];
            }
        }
        BingoCard {
            card,
            marked: [[false; 5]; 5],
        }
    }

    /* Check the card for number called; if present,
     * set the marked flag.
     */
    fn call_number(&mut self, number: u8) {
        for y in 0..5 {
            for x in 0..5 {
                if self.card[y][x] == number {
                    self.marked[y][x] = true;
                }
            }
        }
    }

    fn check_win(&self) -> bool {
        let h = self.check_win_horiz();
        let v = self.check_win_vert();
        println!("h={:?} v={:?}", h, v);
        return h || v;
    }

    /* Check for 5-in-a-row horizontally */
    fn check_win_horiz(&self) -> bool {
        let mut run: u32 = 0;
        for y in 0..5 {
            run = 0;
            for x in 0..5 {
                if self.marked[y][x] {
                    run += 1;
                } else {
                    break;
                }
            }
            if run == 5 {
                break;
            }
        }
        return run == 5;
    }

    /* Check for 5-in-a-row vertically */
    fn check_win_vert(&self) -> bool {
        let mut run: u32 = 0;
        for x in 0..5 {
            run = 0;
            for y in 0..5 {
                if self.marked[y][x] {
                    run += 1;
                } else {
                    break;
                }
            }
            if run == 5 {
                break;
            }
        }
        return run == 5;
    }

    /* Check for 5-in-a-row on the diagonals */
    // TODO: 2021/12/07 - jbradach - remove this, it wasn't needed for the puzzle.
    // fn check_win_diag(&self) -> Option<Vec<u8>> {
    //     let mut run = 0;
    //     /* Upper left to lower right */
    //     for d in 0..5 {
    //         if self.marked[d][d] {
    //             run += 1;
    //         } else {
    //             break;
    //         }
    //     }
    //     if run == 5 {
    //         let mut winner: Vec<u8> = Vec::new();
    //         for d in 0..5 {
    //             winner.push(self.card[d][d]);
    //         }
    //         return Some(winner);
    //     }

    //     /* Lower left to upper right */
    //     for d in 0..5 {
    //         if self.marked[4-d][d] {
    //             run += 1;
    //         } else {
    //             break;
    //         }
    //     }
    //     if run == 5 {
    //         let mut winner: Vec<u8> = Vec::new();
    //         for d in 0..5 {
    //             winner.push(self.card[4-d][d]);
    //         }
    //         return Some(winner);
    //     }
    //     None
    // }

    fn score(&self) -> u32 {
        let mut score: u32 = 0;
        for y in 0..5 {
            for x in 0..5 {
                if !self.marked[y][x] {
                    score += self.card[y][x] as u32;
                }
            }
        }
        score
    }
}

impl fmt::Display for BingoCard {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for y in 0..5 {
            for x in 0..5 {
                let v = self.card[y][x];
                let t: ColoredString;
                if self.marked[y][x] {
                    t = v.to_string().red().bold();
                } else {
                    t = v.to_string().white();
                }
                write!(f, "{:>2} ", t);
            }
            write!(f, "\n");
        }
        write!(f, "\n")
    }
}

#[derive(Clone, Debug)]
struct BingoGame {
    last_called: u8,
    score: u32,
}

impl BingoGame {
    fn from_file(input: &Path) -> BingoGame {
        println!("Path={:?}", input);
        println!("c={:?}", fs::canonicalize(&input));
        let file = File::open(input).unwrap();
        let game_state: Vec<String> =
            BufReader::new(file)
                .lines()
                .map(|s| s.unwrap())
                .collect();
        BingoGame::new(game_state)
    }

    fn new(mut game_state: Vec<String>) -> BingoGame {
        // First line is the calls
        // Line break
        // Card
        let mut game_boards: Vec<BingoCard> = Vec::new();
        let call_order: Vec<u8> =
            game_state
            .remove(0)
            .split(",")
            .map(|v| v.parse::<u8>()
            .unwrap())
            .collect();
        println!("call_order={:?}",call_order);

        // Iterate over next chunks of newline
        // and 5 x 5 game boards until end-of-lines
        // Each of these [&String; 5] chunks should
        // be passed to BingoBoard::new()
        while game_state.len() > 0 && game_state.remove(0) == "" {
            let mut card_vecstr: Vec<String> = Vec::new();
            for _ in 0..5 {
                card_vecstr.push(game_state.remove(0));
            }
            let card = BingoCard::new(card_vecstr);
            println!("card=\n{}", card);
            game_boards.push(card);
        }

        let mut last_called: u8 = 0;
        let mut score: u32 = 0;
        'outer: for number in call_order {
            println!("Calling number = {}", number);
            for card in &mut game_boards {
                card.call_number(number);
                println!("card=\n{}", card);
                if card.check_win() {
                    score = card.score();
                    last_called = number;
                    break 'outer
                }
            }
        }
        BingoGame {
            last_called,
            score,
        }
    }

    fn final_score(&self) -> u32 {
        return self.last_called as u32 * self.score;
    }
}

fn main() {
    // Argument parsing using the Clap builder pattern.
    let argmatches = App::new("Giant Squid")
        .version("0.1")
        .author("Jack Bradach <jack@bradach.net>")
        .about("Advent of Code 2021 - Day 4: Giant Squid")
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

    let bingo_game = BingoGame::from_file(input);
    println!("Part 1: final_score={}", bingo_game.final_score());
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_giantsquid_part1() {
        let mut input = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        input.push("giantsquid_test.txt");
        const FINAL_SCORE: u32 = 4512;
        let bingo_game = BingoGame::from_file(&input);
        assert_eq!(bingo_game.final_score(), FINAL_SCORE);
    }

  
}
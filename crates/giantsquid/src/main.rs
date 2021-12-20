/*
 * https://adventofcode.com/2021/day/4
 */

use std::path::{Path};
use std::io::{BufReader, BufRead};
use std::fs::File;
use std::fmt;

use colored::{ColoredString, Colorize};

use util;

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

    /* Returns true if this card is a winner */
    fn is_winner(&self) -> bool {
        let h = self.is_winner_horiz();
        let v = self.is_winner_vert();
        return h || v;
    }

    /* Check for 5-in-a-row horizontally */
    fn is_winner_horiz(&self) -> bool {
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
    fn is_winner_vert(&self) -> bool {
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

    fn score(&self) -> u32 {
        let mut score: u32 = 0;
        for x in 0..5 {
            for y in 0..5 {
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
                write!(f, "{:>2} ", t).unwrap();
            }
            write!(f, "\n").unwrap();
        }
        write!(f, "\n")
    }
}

#[derive(Clone, Debug)]
struct BingoGame {
    call_order: Vec<u8>,
    cards: Vec<BingoCard>,
}

impl BingoGame {
    fn from_file(input: &Path) -> BingoGame {
        let file = File::open(input).unwrap();
        let game_state: Vec<String> =
            BufReader::new(file)
                .lines()
                .map(|s| s.unwrap())
                .collect();
        BingoGame::new(game_state)
    }

    fn new(mut game_state: Vec<String>) -> BingoGame {
        /* First line is the number calls, and then a series of line-break and cards. */
        let mut cards: Vec<BingoCard> = Vec::new();
        let call_order: Vec<u8> =
            game_state
            .remove(0)
            .split(",")
            .map(|v| v.parse::<u8>()
            .unwrap())
            .collect();

        /* Iterate over next chunks of newline
         * and 5 x 5 game boards until end-of-lines
         * Each of these [&String; 5] chunks should
         * be passed to BingoBoard::new()
         */
        while game_state.len() > 0 && game_state.remove(0) == "" {
            let mut card_vecstr: Vec<String> = Vec::new();
            for _ in 0..5 {
                card_vecstr.push(game_state.remove(0));
            }
            let card = BingoCard::new(card_vecstr);
            cards.push(card);
        }

        BingoGame {
            call_order,
            cards,
        }
    }

    /* Returns the score of the winner.
     * Winner is the first card to have 5-in-a-row. 
     * Card score is the sum of all numbers that weren't called on the card.
     * This is multiplied by the last number called to produce the score.
     */
    fn score_winner(&self) -> u32 {
        let mut last_called: u8 = 0;
        let mut score: u32 = 0;
        let mut cards = self.cards.clone();
        'outer: for number in &self.call_order {
            // println!("Calling number = {}", number);
            for card in &mut cards {
                card.call_number(number.clone());
                if card.is_winner() {
                    score = card.score();
                    last_called = number.clone();
                    break 'outer
                }
            }
        }
        score = score * last_called as u32;
        score
    }
    
    /* Returns the score of the loser.
     * Loser is the last card to have 5-in-a-row. 
     * Card score is the sum of all numbers that weren't called on the card.
     * This is multiplied by the last number called to produce the score.
     */
    fn score_loser(&self) -> u32 {
        let mut cards = self.cards.clone();
        let mut last_called: u8 = 0;
        let mut score: u32 = 0;
        for number in &self.call_order {
            cards.iter_mut().for_each(|c| c.call_number(number.clone()));
            if cards.len() == 1 && cards[0].is_winner() {
                last_called = number.clone();
                score = cards[0].score();
                break;
            }
            cards.retain(|c| !c.is_winner());
        }
        score = score * last_called as u32;
        score
    }
}

fn main() {
    let input = util::advent_cli("Giant Squid", 4);
    let bingo_game = BingoGame::from_file(&input);
    println!("Part 1: score_winner={}", bingo_game.score_winner());
    println!("Part 2: score_winner={}", bingo_game.score_loser());
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::{PathBuf};
    
    #[test]
    fn test_giantsquid_part1() {
        let mut input = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        input.push("giantsquid_test.txt");
        const SCORE_WINNER: u32 = 4512;
        let bingo_game = BingoGame::from_file(&input);
        assert_eq!(bingo_game.score_winner(), SCORE_WINNER);
    }

    #[test]
    fn test_giantsquid_part2() {
        let mut input = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        input.push("giantsquid_test.txt");
        const SCORE_LOSER: u32 = 1924;
        let bingo_game = BingoGame::from_file(&input);
        assert_eq!(bingo_game.score_loser(), SCORE_LOSER);
    }
  
}
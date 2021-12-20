/*
 * https://adventofcode.com/2021/day/3
 */
use std::path::Path;
use std::io::{BufReader, BufRead};
use std::fs::File;

use util;

#[derive(Clone, Debug)]
struct BinaryDiagnostic {
    report_width: usize,
    report: Vec<u32>,
}

impl BinaryDiagnostic {
    fn from_file(input: &Path) -> BinaryDiagnostic {
        let file = File::open(input).unwrap();
        let raw_report: Vec<String> =
            BufReader::new(file)
                .lines()
                .map(|s| s.unwrap())
                .collect();
        BinaryDiagnostic::from_vecstring(raw_report)
    }

    fn from_vecstring(raw_report: Vec<String>) -> BinaryDiagnostic {
        let report_width = raw_report[0].len();
        let report = raw_report.iter().map(|s| u32::from_str_radix(s, 2).unwrap()).collect();
        BinaryDiagnostic {
            report_width,
            report,
        }
    }

    fn episilon(&self) -> u32 {
        const MAX: usize = 32;
        let mut density_ones: [u32; MAX] = [0; MAX];
        let mut density_zeroes: [u32; MAX] = [0; MAX];
        for i in 0..self.report_width {
            for v in &self.report {
                if v & (1 << i) == 0 {
                    density_zeroes[i] += 1;
                } else {
                    density_ones[i] += 1;
                }
            }
        }
        let mut episilon: u32 = 0;
        for i in 0..self.report_width {
            if density_zeroes[i] > density_ones[i] {
                episilon |= 1 << i;
            }
        }
        episilon
    }

    fn gamma(&self) -> u32 {
        const REPORT_WIDTH: usize = 32;
        let mut density_ones: [u32; REPORT_WIDTH] = [0; REPORT_WIDTH];
        let mut density_zeroes: [u32; REPORT_WIDTH] = [0; REPORT_WIDTH];
        for i in 0..REPORT_WIDTH {
            for v in &self.report {
                if v & (1 << i) != 0 {
                    density_ones[i] += 1;
                } else {
                    density_zeroes[i] += 1;
                }
            }
        }
        let mut gamma = 0;
        for i in 0..REPORT_WIDTH {
            if density_ones[i] > density_zeroes[i] {
                gamma |= 1 << i;
            }
        }
        gamma
    }

    fn oxygen(&self) -> u32 {
        const MAX: usize = 32;
        let mut density_ones: [u32; MAX] = [0; MAX];
        let mut density_zeroes: [u32; MAX] = [0; MAX];
        /* Need to scan through each bit, find the most common,
         * and then use it to prune the rest of the vector. */
        let mut diag = self.report.clone();
        for i in (0..self.report_width).rev() {
            for v in &diag {
                if v & (1 << i) == 0 {
                    density_zeroes[i] += 1;
                } else {
                    density_ones[i] += 1;
                }
            }
            let polarity = if density_ones[i] >= density_zeroes[i] { 1 } else { 0 };
            diag = diag
                .iter()
                .filter_map(|&v| {
                    if ((v & (1 << i)) >> i) == polarity {
                        Some(v)
                    } else { 
                        None 
                    }
                }).collect::<Vec<_>>();
            if diag.len() == 1 {
                break;
            }
        }
        let oxygen = diag.pop().unwrap();
        oxygen
    }

    fn co2(&self) -> u32 {
        const MAX: usize = 32;
        let mut density_ones: [u32; MAX] = [0; MAX];
        let mut density_zeroes: [u32; MAX] = [0; MAX];
        /* Need to scan through each bit, find the most common,
         * and then use it to prune the rest of the vector. */
        let mut diag = self.report.clone();
        for i in (0..self.report_width).rev() {
            for v in &diag {
                if v & (1 << i) == 0 {
                    density_zeroes[i] += 1;
                } else {
                    density_ones[i] += 1;
                }
            }
            let polarity = if density_zeroes[i] <= density_ones[i] { 0 } else { 1 };
            diag = diag
                .iter()
                .filter_map(|&v| {
                    if ((v & (1 << i)) >> i) == polarity {
                        // println!("+");
                        Some(v)
                    } else { 
                        // println!("-");
                        None 
                    }
                }).collect::<Vec<_>>();
            if diag.len() == 1 {
                break;
            }
        }
        let co2 = diag.pop().unwrap();
        co2
    }
}

fn main() {
    let input = util::advent_cli("Binary Diagnostic", 3);
    let diag = BinaryDiagnostic::from_file(&input);
    println!("Part 1: gamma = {}, epsilon = {}, power = {}",
        diag.gamma(),
        diag.episilon(),
        diag.gamma() * diag.episilon()
    );

    println!("Part 2: oxygen = {}, CO2 = {}, life support = {}",
        diag.oxygen(),
        diag.co2(),
        diag.oxygen() * diag.co2()
    );
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_bindiag_part1() {
        const GAMMA: u32 = 22;
        const EPSILON: u32 = 9;
        let report = vec![
            "00100", "11110", "10110", "10111", "10101", "01111",
            "00111", "11100", "10000", "11001", "00010", "01010",
        ];
        // let diag = BinaryDiagnostic { report_width: 5, report, };
        let report = report.iter().map(|s| s.to_string()).collect();
        let diag = BinaryDiagnostic::from_vecstring(report);
        assert_eq!(GAMMA, diag.gamma());
        assert_eq!(EPSILON, diag.episilon());
    }

    #[test]
    fn test_bindiag_part2() {
        const OXYGEN: u32 = 23;
        const CO2: u32 = 10;
        let report = vec![
            "00100", "11110", "10110", "10111", "10101", "01111",
            "00111", "11100", "10000", "11001", "00010", "01010",
        ];
        // let diag = BinaryDiagnostic { report_width: 5, report, };
        let report = report.iter().map(|s| s.to_string()).collect();
        let diag = BinaryDiagnostic::from_vecstring(report);
        assert_eq!(OXYGEN, diag.oxygen());
        assert_eq!(CO2, diag.co2());
    }

}
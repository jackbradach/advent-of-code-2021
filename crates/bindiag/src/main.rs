/*
 * https://adventofcode.com/2021/day/2
 */
use clap::{App, Arg};
use std::path::Path;
use std::io::{BufReader, BufRead};
use std::fs::File;


#[derive(Clone, Debug)]
struct BinaryDiagnostic {
    report_width: usize,
    report: Vec<u32>,
}

impl BinaryDiagnostic {

    // FIXME - needs to convert from binary string
    // FIXME - also needs to count number of bits for report_width
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

}

fn main() {
    // Argument parsing using the Clap builder pattern.
    let argmatches = App::new("Binary Diagnostic")
        .version("0.1")
        .author("Jack Bradach <jack@bradach.net>")
        .about("Advent of Code 2021 - Day 3: Binary Diagnostic")
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

    let diag = BinaryDiagnostic::from_file(input);
    println!("Part 1: gamma = {}, epsilon = {}, power = {}",
        diag.gamma(),
        diag.episilon(),
        diag.gamma() * diag.episilon()
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

}
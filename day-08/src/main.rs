use commons::io::load_file_lines;
use itertools::Itertools;
use lazy_static::lazy_static;
use std::collections::HashMap;
use std::str::FromStr;
use thiserror::Error;

lazy_static! {
    static ref DIGITS: Vec<&'static str> = vec![
        "ABCEFG", "CF", "ACDEG", "ACDFG", "BCDF", "ABDFG", "ABDEFG", "ACF", "ABCDEFG", "ABCDFG",
    ];
}

#[derive(Debug)]
struct PuzzleInput {
    digits: Vec<String>,
    wanted_numbers: Vec<String>,
}

impl PuzzleInput {
    pub fn solve(&self) -> Vec<u8> {
        let mut digit_map = HashMap::new();
        for i in vec![1, 4, 7, 8] {
            let digit = self
                .digits
                .iter()
                .find(|s| s.len() == DIGITS[i].len())
                .unwrap();
            digit_map.insert(digit, i as u8);
        }

        self.wanted_numbers
            .iter()
            .map(|s| digit_map.get(s).unwrap_or(&u8::MAX))
            .copied()
            .collect()
    }
}

#[derive(Debug, Error)]
enum InputParseError {
    #[error("Missing field")]
    MissingField,
}

impl FromStr for PuzzleInput {
    type Err = InputParseError;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let input_string = input.to_string();

        let mut digits = Vec::new();
        let mut wanted_numbers = Vec::new();
        let mut seen_sep = false;

        for word in input_string.split_whitespace() {
            let ordered_word: String = word.chars().sorted().collect();
            if word == "|" {
                seen_sep = true;
            } else if !seen_sep {
                digits.push(ordered_word);
            } else {
                wanted_numbers.push(ordered_word);
            }
        }

        if !seen_sep {
            return Err(InputParseError::MissingField);
        }

        Ok(PuzzleInput {
            digits,
            wanted_numbers,
        })
    }
}

fn main() {
    let input: Vec<PuzzleInput> = load_file_lines("input.txt")
        .map(|res| res.unwrap())
        .collect();

    let wanted_part1 = vec![1, 4, 7, 8];
    let part1: usize = input
        .iter()
        .map(|i| {
            i.solve()
                .iter()
                .filter(|x| wanted_part1.contains(x))
                .count()
        })
        .sum();
    println!("{}", part1);
}

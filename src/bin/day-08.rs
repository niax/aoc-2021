use aoc2021::commons::io::load_stdin_lines;
use itertools::Itertools;
use lazy_static::lazy_static;
use std::collections::{HashMap, HashSet};
use std::str::FromStr;
use thiserror::Error;

lazy_static! {
    static ref DIGITS: Vec<&'static str> = vec![
        "ABCEFG", "CF", "ACDEG", "ACDFG", "BCDF", "ABDFG", "ABDEFG", "ACF", "ABCDEFG", "ABCDFG",
    ];
}

#[derive(Debug)]
struct PuzzleInput {
    digits: Vec<HashSet<char>>,
    wanted_numbers: Vec<HashSet<char>>,
}

impl PuzzleInput {
    pub fn new(digits: Vec<HashSet<char>>, wanted_numbers: Vec<HashSet<char>>) -> Self {
        Self {
            digits,
            wanted_numbers,
        }
    }

    pub fn solve(&mut self) -> Vec<u8> {
        let mut digit_to_wires = HashMap::new();

        let digits = self.digits.clone();
        for i in &[1, 4, 7, 8] {
            let digit = digits.iter().find(|s| s.len() == DIGITS[*i].len()).unwrap();
            digit_to_wires.insert(*i as u8, digit.clone());
        }
        let one = digit_to_wires[&1].clone();
        let mut five_long: Vec<_> = digits.iter().filter(|s| s.len() == 5).collect();

        let (threes, five_long): (Vec<&HashSet<char>>, Vec<&HashSet<char>>) = five_long
            .iter()
            .partition(|s| s.difference(&one).count() == 3);
        let three = threes[0];
        digit_to_wires.insert(3, three.clone());

        let six = {
            let six_long: Vec<_> = digits.iter().filter(|s| s.len() == 6).collect();
            let (sixes, others): (Vec<&HashSet<char>>, Vec<&HashSet<char>>) = six_long
                .iter()
                .partition(|s| s.difference(&one).count() == 5);
            let six = sixes[0];
            digit_to_wires.insert(6, six.clone());

            let (nines, zeros): (Vec<&HashSet<char>>, Vec<&HashSet<char>>) = others
                .iter()
                .partition(|s| s.difference(&three).count() == 1);
            let nine = nines[0];
            digit_to_wires.insert(9, nine.clone());
            let zeros = zeros[0];
            digit_to_wires.insert(0, zeros.clone());

            six
        };

        let (twos, fives): (Vec<&HashSet<char>>, Vec<&HashSet<char>>) = five_long
            .iter()
            .partition(|s| s.difference(&six).count() == 1);
        digit_to_wires.insert(2, twos[0].clone());
        digit_to_wires.insert(5, fives[0].clone());

        let mut wires_to_digit = HashMap::new();
        for (k, v) in digit_to_wires {
            let s = v.iter().sorted().collect::<String>();
            wires_to_digit.insert(s, k);
        }

        self.wanted_numbers
            .iter()
            .map(|s| s.iter().sorted().collect::<String>())
            .map(|s| wires_to_digit.get(&s).unwrap_or(&u8::MAX))
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
            let letter_set: HashSet<char> = word.chars().collect();
            if word == "|" {
                seen_sep = true;
            } else if !seen_sep {
                digits.push(letter_set);
            } else {
                wanted_numbers.push(letter_set);
            }
        }

        if !seen_sep {
            return Err(InputParseError::MissingField);
        }

        Ok(PuzzleInput::new(digits, wanted_numbers))
    }
}

fn main() {
    let mut input: Vec<PuzzleInput> = load_stdin_lines()
        .map(|res| res.unwrap())
        .collect();

    let wanted_part1 = vec![1, 4, 7, 8];
    let part1: usize = input
        .iter_mut()
        .map(|i| {
            i.solve()
                .iter()
                .filter(|x| wanted_part1.contains(x))
                .count()
        })
        .sum();
    println!("{}", part1);
    let part2: u64 = input
        .iter_mut()
        .map(|i| i.solve().iter().fold(0, |a, n| a * 10 + *n as u64))
        .sum();
    println!("{}", part2);
}

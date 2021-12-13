use aoc2021::commons::io::load_stdin_lines;
use lazy_static::lazy_static;
use std::collections::HashMap;
use std::str::FromStr;
use thiserror::Error;

lazy_static! {
    static ref DIGITS: Vec<u32> = vec![6, 2, 5, 5, 4, 5, 6, 3, 7, 6,];
}

#[derive(Debug)]
struct PuzzleInput {
    digits: Vec<u8>,
    wanted_numbers: Vec<u8>,
    solution: Option<Vec<u8>>,
}

fn bit_difference(a: u8, b: u8) -> u32 {
    (a & (a ^ b)).count_ones()
}

impl PuzzleInput {
    pub fn new(digits: Vec<u8>, wanted_numbers: Vec<u8>) -> Self {
        Self {
            digits,
            wanted_numbers,
            solution: None,
        }
    }

    fn actually_solve(&mut self) {
        let mut digit_to_wires = vec![0; 10];

        let digits = self.digits.clone();
        for i in &[1, 4, 7, 8] {
            let digit = digits
                .iter()
                .find(|s| s.count_ones() == DIGITS[*i])
                .unwrap();
            digit_to_wires[*i] = *digit;
        }
        let one = digit_to_wires[1];
        let five_long: Vec<u8> = digits
            .iter()
            .filter(|s| s.count_ones() == 5)
            .copied()
            .collect();

        let (threes, five_long): (Vec<&u8>, Vec<&u8>) = five_long
            .iter()
            .partition(|s| bit_difference(**s, one) == 3);
        let three = *threes[0];
        digit_to_wires[3] = three;

        let six = {
            let six_long: Vec<_> = digits
                .iter()
                .filter(|s| s.count_ones() == 6)
                .copied()
                .collect();
            let (sixes, others): (Vec<&u8>, Vec<&u8>) =
                six_long.iter().partition(|s| bit_difference(**s, one) == 5);
            let six = *sixes[0];
            digit_to_wires[6] = six;

            let (nines, zeros): (Vec<&u8>, Vec<&u8>) = others
                .iter()
                .partition(|s| bit_difference(***s, three) == 1);
            digit_to_wires[9] = *nines[0];
            digit_to_wires[0] = *zeros[0];

            six
        };

        let (twos, fives): (Vec<&u8>, Vec<&u8>) = five_long
            .iter()
            .partition(|s| bit_difference(***s, six) == 1);
        digit_to_wires[2] = *twos[0];
        digit_to_wires[5] = *fives[0];

        let mut wires_to_digit = HashMap::new();
        for (k, v) in digit_to_wires.iter().enumerate() {
            wires_to_digit.insert(v, k);
        }

        self.solution = Some(
            self.wanted_numbers
                .iter()
                .map(|s| wires_to_digit.get(s).map(|s| *s as u8).unwrap_or(u8::MAX))
                .collect(),
        );
    }

    pub fn solve(&mut self) -> Vec<u8> {
        if self.solution.is_none() {
            self.actually_solve();
        }
        self.solution.as_ref().unwrap().clone()
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
            if word == "|" {
                seen_sep = true;
                continue;
            }
            let mut letter_set = 0;
            for c in word.chars() {
                let offset = c as usize - 'a' as usize;
                letter_set |= 1 << offset;
            }
            if !seen_sep {
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
    let mut input: Vec<PuzzleInput> = load_stdin_lines().map(|res| res.unwrap()).collect();

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

use std::io::prelude::*;
use std::io::{self, BufReader};

fn most_common_bit(nums: &[u16], pos: u8) -> bool {
    let mask = 1 << pos;
    let mut bits_on = 0;
    for num in nums.iter() {
        if num & mask != 0 {
            bits_on += 1;
        }
    }

    bits_on as f64 >= (nums.len() as f64 / 2.0)
}

fn main() {
    let reader = BufReader::new(io::stdin());
    let nums: Vec<u16> = reader
        .lines()
        .map(|res| u16::from_str_radix(&res.unwrap(), 2).unwrap())
        .collect();
    let bit_len = 12;
    let total_mask = 0xfff; // All 12 bits on

    // Part 1
    let mut gamma = 0;
    for i in 0..bit_len {
        if most_common_bit(&nums, i) {
            gamma |= 1 << i;
        }
    }
    let epslion = !gamma & total_mask;
    println!("{}", epslion as u32 * gamma as u32);

    // Part 2
    let mut generator_possibles = nums.clone();
    let mut scrubber_possibles = nums;

    let mut i = bit_len;
    while generator_possibles.len() > 1 {
        i -= 1;
        let mask = 1 << i;
        let most_common = if most_common_bit(&generator_possibles, i) {
            mask
        } else {
            0
        };
        let next = generator_possibles
            .iter()
            .filter(|x| *x & mask == most_common)
            .copied()
            .collect();
        generator_possibles = next;
    }

    i = bit_len;
    while scrubber_possibles.len() > 1 {
        i -= 1;
        let mask = 1 << i;
        let most_common = if most_common_bit(&scrubber_possibles, i) {
            0
        } else {
            mask
        };
        let next = scrubber_possibles
            .iter()
            .filter(|x| *x & mask == most_common)
            .copied()
            .collect();
        scrubber_possibles = next;
    }
    println!(
        "{}",
        scrubber_possibles[0] as u32 * generator_possibles[0] as u32
    );
}

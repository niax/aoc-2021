use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

fn main() {
    let file = File::open("input.txt").expect("Could not open input file");
    let reader = BufReader::new(file);
    let nums: Vec<u16> = reader
        .lines()
        .map(|res| u16::from_str_radix(&res.unwrap(), 2).unwrap())
        .collect();
    // Part 1
    let bit_len = 12;
    let mut gamma = 0;
    let mut total_mask = 0;
    for i in 0..bit_len {
        let mask = 1 << i;
        let mut bits_on = 0;
        for num in nums.iter() {
            if num & mask != 0 {
                bits_on += 1;
            }
        }
        if bits_on > (nums.len() / 2) {
            gamma |= mask;
        }
        total_mask |= mask;
    }
    let epslion = !gamma & total_mask;

    println!("{}", epslion as u32 * gamma as u32);
}

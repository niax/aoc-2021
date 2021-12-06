use std::fs::File;
use std::io::prelude::*;

fn main() {
    let mut file = File::open("input.txt").expect("Could not open input file");
    let mut line = String::new();
    file.read_to_string(&mut line).expect("File read");
    line = line.split_whitespace().collect();
    let mut initial = vec![0; 8];
    for s in line.split(',') {
        let i: usize = s.parse().unwrap();
        initial[i] += 1;
    }

    let mut current = initial;
    let mut next = vec![0; 9];
    for day in 1..=256 {
        for (i, v) in current.iter().enumerate() {
            if i == 0 {
                next[8] += v;
                next[6] += v;
            } else {
                next[i - 1] += v;
            }
        }
        current = next;
        next = vec![0; 9];
        // Part 1
        if day == 80 {
            println!("{}", current.iter().sum::<u64>());
        }
    }

    // Part 2
    println!("{}", current.iter().sum::<u64>());
}

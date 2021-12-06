use std::fs::File;
use std::io::prelude::*;

fn main() {
    let mut file = File::open("input.txt").expect("Could not open input file");
    let mut line = String::new();
    file.read_to_string(&mut line).expect("File read");
    line = line.split_whitespace().collect();
    let initial: Vec<u32> = line.split(',').map(|x| x.parse().unwrap()).collect();

    let mut current = initial;
    let mut next = Vec::new();
    for _ in 0..80 {
        for v in &current {
            if *v == 0 {
                next.push(8);
                next.push(6);
            } else {
                next.push(v - 1);
            }
        }
        current = next;
        next = Vec::new();
    }
    println!("{}", current.len());
}

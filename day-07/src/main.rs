use std::fs::File;
use std::io::prelude::*;

fn main() {
    let mut file = File::open("input.txt").expect("Could not open input file");
    let mut line = String::new();
    file.read_to_string(&mut line).expect("File read");
    line = line.split_whitespace().collect();
    let positions: Vec<u32> = line.split(',').map(|x| x.parse().unwrap()).collect();

    let max = positions.iter().max().unwrap();
    let min = positions.iter().min().unwrap();

    let part1 = (*min..=*max)
        .map(|possible| {
            (
                possible,
                positions
                    .iter()
                    .map(|position| {
                        if *position > possible {
                            position - possible
                        } else {
                            possible - position
                        }
                    })
                    .sum(),
            )
        })
        .min_by_key(|x: &(u32, u32)| x.1);

    println!("{:?}", part1);
}

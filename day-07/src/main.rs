use std::cmp;
use std::fs::File;
use std::io::prelude::*;

fn find_solution<F>(positions: &Vec<u32>, fuel_metric: F) -> u32
where
    F: Fn(u32, u32) -> u32,
{
    let max = positions.iter().max().unwrap();
    let min = positions.iter().min().unwrap();

    (*min..=*max)
        .map(|possible| {
            (
                possible,
                positions
                    .iter()
                    .map(|position| fuel_metric(possible, *position))
                    .sum(),
            )
        })
        .min_by_key(|x: &(u32, u32)| x.1)
        .unwrap()
        .1
}

fn main() {
    let mut file = File::open("input.txt").expect("Could not open input file");
    let mut line = String::new();
    file.read_to_string(&mut line).expect("File read");
    line = line.split_whitespace().collect();
    let positions: Vec<u32> = line.split(',').map(|x| x.parse().unwrap()).collect();

    let part1 = find_solution(&positions, |position, possible| {
        if position > possible {
            position - possible
        } else {
            possible - position
        }
    });
    println!("{:?}", part1);

    let part2 = find_solution(&positions, |position, possible| {
        let min = cmp::min(position, possible);
        let max = cmp::max(position, possible);
        let steps = max - min;

        steps * (1 + steps)/2
    });
    println!("{:?}", part2);
}

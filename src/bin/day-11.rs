use aoc2021::commons::{
    grid::{Grid, SingleVecGrid},
    io::load_stdin_lines,
};
use std::collections::VecDeque;

#[allow(dead_code)]
fn print_grid(g: &SingleVecGrid<u32>) {
    for y in 0..g.height() {
        let row = (0..g.width())
            .map(|x| g.at(&(x, y)).unwrap().to_string())
            .collect::<String>();
        println!("{}", row);
    }
}

fn step(grid: &mut SingleVecGrid<u32>) {
    let mut flashers = VecDeque::new();

    for y in 0..grid.height() {
        for x in 0..grid.width() {
            let coord = (x, y);
            let v = *grid.at(&coord).unwrap() + 1;
            grid.set(coord, v);
            if v > 9 {
                flashers.push_back(coord);
            }
        }
    }

    let adjacent: Vec<(isize, isize)> = vec![
        (0, -1),
        (0, 1),
        (1, 0),
        (-1, 0),
        (-1, -1),
        (-1, 1),
        (1, -1),
        (1, 1),
    ];

    while !flashers.is_empty() {
        let coord = flashers.pop_front().unwrap();

        if *grid.at(&coord).unwrap() == 0 {
            continue;
        }

        for adj in &adjacent {
            let x = coord.0 as isize + adj.0;
            let y = coord.1 as isize + adj.1;
            if x < 0 || x >= grid.width() as isize || y < 0 || y >= grid.height() as isize {
                continue;
            }
            let coord = (x as usize, y as usize);
            let v = *grid.at(&coord).unwrap();
            if v == 0 {
                continue;
            }
            let v = v + 1;
            grid.set(coord, v);
            if v > 9 {
                flashers.push_back(coord);
            }
        }

        grid.set(coord, 0);
    }
}

fn count_flashes(grid: &SingleVecGrid<u32>) -> usize {
    (0..grid.width())
        .map(|y| {
            (0..grid.height())
                .filter(|x| *grid.at(&(*x, y)).unwrap() == 0)
                .count()
        })
        .sum()
}

fn main() {
    let mut grid = SingleVecGrid::new(10, 10);

    for (y, line) in load_stdin_lines::<String>().enumerate() {
        for (x, c) in line.unwrap().chars().enumerate() {
            grid.set((x, y), c.to_digit(10).unwrap());
        }
    }

    let mut part1 = 0;
    for _ in 0..100 {
        step(&mut grid);
        part1 += count_flashes(&grid);
    }
    println!("{}", part1);
}

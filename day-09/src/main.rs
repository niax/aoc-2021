use commons::grid::{Grid, SparseGrid};
use commons::io::load_file_lines;
use itertools::Itertools;
use std::collections::{HashSet, VecDeque};

fn main() {
    let mut grid = SparseGrid::new();
    let adjacent: Vec<(isize, isize)> = vec![(0, -1), (0, 1), (1, 0), (-1, 0)];

    for (y, line) in load_file_lines::<String>("input.txt").enumerate() {
        for (x, c) in line.unwrap().chars().enumerate() {
            grid.set(
                (x as isize, y as isize),
                c.to_string().parse::<u8>().unwrap(),
            );
        }
    }

    let mut low_points = Vec::new();
    for y in 0..grid.height() {
        for x in 0..grid.width() {
            let coord = (x as isize, y as isize);
            let value = grid.at(&coord).unwrap();
            let surrounding_greater = adjacent
                .iter()
                .map(|(x1, y1)| {
                    grid.at(&(x as isize + x1, y as isize + y1))
                        .copied()
                        .unwrap_or(u8::MAX)
                })
                .filter(|v1| v1 > value)
                .count();
            if surrounding_greater == 4 {
                low_points.push(coord);
            }
        }
    }
    let part1 = low_points
        .iter()
        .map(|c| *grid.at(c).unwrap() as u32 + 1)
        .sum::<u32>();
    println!("{}", part1);

    let mut sizes = Vec::new();
    for point in low_points {
        let mut to_search = VecDeque::from([point]);
        let mut seen = HashSet::new();
        let mut size = 0;
        while !to_search.is_empty() {
            let coord = to_search.pop_front().unwrap();
            let maybe_value = grid.at(&coord);
            if seen.contains(&coord) || maybe_value.is_none() {
                continue;
            }
            let value = *maybe_value.unwrap();
            if value == 9 {
                continue;
            }
            size += 1;

            let (x, y) = coord;
            for &(x1, y1) in &adjacent {
                let next = (x + x1, y + y1);
                to_search.push_back(next);
            }

            seen.insert(coord);
        }
        sizes.push(size);
    }
    let part2 = sizes.iter().sorted().rev().take(3).product::<u32>();
    println!("{:?}", part2);
}

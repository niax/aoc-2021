use commons::grid::{Grid, VecGrid};
use commons::io::load_file_lines;
use itertools::Itertools;
use std::collections::{HashSet, VecDeque};

fn main() {
    let mut grid = VecGrid::new();
    let adjacent: Vec<(isize, isize)> = vec![(0, -1), (0, 1), (1, 0), (-1, 0)];

    for line in load_file_lines::<String>("input.txt") {
        grid.add_row(line.unwrap().chars().map(|c| c.to_digit(10).unwrap()));
    }

    let mut low_points = Vec::new();
    for y in 0..grid.height() {
        for x in 0..grid.width() {
            let coord = (x, y);
            let value = grid.at(&coord).unwrap();
            let surrounding_greater = adjacent
                .iter()
                .map(|&(x1, y1)| {
                    let x2 = x as isize + x1;
                    let y2 = y as isize + y1;
                    if x2 < 0 || y2 < 0 {
                        u32::MAX
                    } else {
                        grid.at(&(x2 as usize, y2 as usize))
                            .copied()
                            .unwrap_or(u32::MAX)
                    }
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
        .map(|c| *grid.at(c).unwrap() + 1)
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
                let x2 = x as isize + x1;
                let y2 = y as isize  + y1;
                if x2 >= 0 && y2 >= 0 {
                    let next = (x2 as usize, y2 as usize);
                    to_search.push_back(next);
                }
            }

            seen.insert(coord);
        }
        sizes.push(size);
    }
    let part2 = sizes.iter().sorted().rev().take(3).product::<u32>();
    println!("{:?}", part2);
}

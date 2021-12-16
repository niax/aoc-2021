use aoc2021::commons::{
    grid::{Grid, SingleVecGrid},
    io::load_stdin_lines,
};
use lazy_static::lazy_static;
use std::cmp::Ordering;
use std::collections::binary_heap::BinaryHeap;

lazy_static! {
    static ref ADJACENT: Vec<(isize, isize)> = vec![(0, -1), (0, 1), (1, 0), (-1, 0)];
}

#[derive(PartialEq, Eq, Debug)]
struct PathItem {
    coord: (usize, usize),
    risk: u32,
}

impl Ord for PathItem {
    fn cmp(&self, other: &Self) -> Ordering {
        self.risk.cmp(&other.risk).reverse()
    }
}

impl PartialOrd for PathItem {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

fn expand_grid(original: SingleVecGrid<u32>, factor: usize) -> SingleVecGrid<u32> {
    let mut grid = SingleVecGrid::new(original.width() * factor, original.height() * factor);
    for y in 0..original.height() {
        for x in 0..original.width() {
            grid.set((x, y), *original.at(&(x, y)).unwrap());
        }
    }

    for y in 0..grid.height() {
        let grid_y = y / original.height();
        let subgrid_y = y % original.height();
        for x in 0..grid.width() {
            if *grid.at(&(x, y)).unwrap() != 0 {
                continue;
            }
            // Figure out which subgrid we're in
            let grid_x = x / original.width();
            let copy_from = if grid_x >= 1 {
                (grid_x - 1, grid_y)
            } else {
                (grid_x, grid_y - 1)
            };
            let read_from_x = copy_from.0 * original.width();
            let read_from_y = copy_from.1 * original.height();
            let subgrid_x = x % original.width();

            let mut v = grid
                .at(&(read_from_x + subgrid_x, read_from_y + subgrid_y))
                .unwrap()
                + 1;
            if v > 9 {
                v = 1;
            }
            grid.set((x, y), v);
        }
    }

    grid
}


fn find_risk(grid: &SingleVecGrid<u32>) -> u32 {
    let end = (grid.width() - 1, grid.height() - 1);
    let mut visited = SingleVecGrid::new(grid.width(), grid.height());
    let mut queue = BinaryHeap::new();
    queue.push(PathItem {
        coord: (0, 0),
        risk: 0,
    });

    while !queue.is_empty() {
        let item = queue.pop().unwrap();
        if item.coord == end {
            return item.risk;
        }
        for (coord, risk) in grid.adjacent(item.coord) {
            let risk_to_here = risk + item.risk;

            let v = visited.at(&coord).unwrap();

            if *v == 0 || *v > risk_to_here {
                visited.set(coord, risk_to_here);
                queue.push(PathItem {
                    coord,
                    risk: risk_to_here,
                });
            }

        }
    }
    panic!("Didn't get to the end!");
}

fn main() {
    let mut rows = Vec::new();
    for line in load_stdin_lines::<String>() {
        let row: Vec<_> = line
            .unwrap()
            .chars()
            .map(|c| c.to_digit(10).unwrap())
            .collect();
        rows.push(row);
    }
    let mut grid = SingleVecGrid::new(rows.len(), rows[0].len());
    for (y, row) in rows.iter().enumerate() {
        for (x, c) in row.iter().enumerate() {
            grid.set((x, y), *c);
        }
    }

    println!("{}", find_risk(&grid));

    let part2 = expand_grid(grid, 5);
    println!("{}", find_risk(&part2));
}

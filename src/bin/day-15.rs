use aoc2021::commons::{
    grid::{Grid, SingleVecGrid, BitGrid},
    io::load_stdin_lines,
};
use std::collections::binary_heap::BinaryHeap;
use std::cmp::Ordering;

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

    let end = (grid.width() - 1, grid.height() - 1);

    let mut visited = BitGrid::new(grid.width(), grid.height());
    let mut queue = BinaryHeap::new();
    queue.push(PathItem {coord:(0,0), risk: 0});

    while !queue.is_empty() {
        let item = queue.pop().unwrap();
        if *visited.at(&item.coord).unwrap() {
            continue;
        }
        if item.coord == end {
            println!("{}", item.risk);
            break;
        }
        for (coord, risk) in grid.adjacent(item.coord) {
            queue.push(PathItem { coord, risk: risk + item.risk });
        }
        visited.set(item.coord, true);
    }
}

use aoc2021::commons::{
    grid::{Grid, SingleVecGrid},
    io::load_stdin_lines,
};
use lazy_static::lazy_static;
use std::cell::RefCell;
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

struct CaveGrid {
    grid: RefCell<SingleVecGrid<Option<u32>>>,
    inner_width: usize,
    inner_height: usize,
}

impl CaveGrid {
    pub fn new(original: &SingleVecGrid<u32>, width: usize, height: usize) -> Self {
        let inner_width = original.width();
        let inner_height = original.height();
        let mut grid = SingleVecGrid::new(inner_width * width, inner_height * height);
        for x in 0..original.width() {
            for y in 0..original.height() {
                grid.set((x, y), Some(*original.at(&(x, y)).unwrap()));
            }
        }
        Self {
            grid: RefCell::new(grid),
            inner_width,
            inner_height,
        }
    }

    pub fn width(&self) -> usize {
        self.grid.borrow().width()
    }

    pub fn height(&self) -> usize {
        self.grid.borrow().height()
    }

    pub fn at(&self, coord: &(usize, usize)) -> Option<u32> {
        if self.grid.borrow().at(coord).unwrap().is_none() {
            // Figure out which subgrid we're in
            let grid_x = coord.0 / self.inner_width;
            let grid_y = coord.1 / self.inner_height;
            let copy_from = if grid_x >= 1 {
                (grid_x - 1, grid_y)
            } else {
                (grid_x, grid_y - 1)
            };
            let read_from_x = copy_from.0 * self.inner_width;
            let read_from_y = copy_from.1 * self.inner_width;
            let subgrid_x = coord.0 % self.inner_width;
            let subgrid_y = coord.1 % self.inner_height;

            let mut v = self
                .at(&(read_from_x + subgrid_x, read_from_y + subgrid_y))
                .unwrap()
                + 1;
            if v > 9 {
                v = 1;
            }
            self.grid.borrow_mut().set(*coord, Some(v));
        }

        *self.grid.borrow().at(coord).unwrap()
    }

    pub fn adjacent(
        &self,
        coord: (usize, usize),
    ) -> impl Iterator<Item = ((usize, usize), u32)> + '_ {
        ADJACENT
            .iter()
            .map(move |off| (coord.0 as isize + off.0, coord.1 as isize + off.1))
            .filter(|(x, y)| {
                *x >= 0 && *x < self.width() as isize && *y >= 0 && *y < self.height() as isize
            })
            .map(|(x, y)| {
                let coord = (x as usize, y as usize);
                (coord, self.at(&coord).unwrap())
            })
    }
}

fn find_risk(grid: &CaveGrid) -> u32 {
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

    let part1 = CaveGrid::new(&grid, 1, 1);
    println!("{}", find_risk(&part1));

    let part2 = CaveGrid::new(&grid, 5, 5);
    println!("{}", find_risk(&part2,));
}

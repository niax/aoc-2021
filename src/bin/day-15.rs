use aoc2021::commons::{
    grid::{BitGrid, Grid, SingleVecGrid},
    io::load_stdin_lines,
};
use lazy_static::lazy_static;
use std::cell::RefCell;
use std::cmp::Ordering;
use std::collections::{binary_heap::BinaryHeap, HashMap};

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

struct GridOfGrids {
    grids: RefCell<HashMap<(usize, usize), SingleVecGrid<u32>>>,
    width: usize,
    height: usize,
    inner_width: usize,
    inner_height: usize,
}

impl GridOfGrids {
    pub fn new(original: SingleVecGrid<u32>, width: usize, height: usize) -> Self {
        let mut grids = HashMap::new();
        let inner_width = original.width();
        let inner_height = original.height();
        grids.insert((0, 0), original);
        Self {
            grids: RefCell::new(grids),
            width,
            height,
            inner_width,
            inner_height,
        }
    }

    pub fn width(&self) -> usize {
        self.width * self.inner_width
    }

    pub fn height(&self) -> usize {
        self.height * self.inner_height
    }

    pub fn at(&self, coord: &(usize, usize)) -> Option<u32> {
        let grid_x = coord.0 / self.inner_width;
        let grid_y = coord.1 / self.inner_height;
        let grid_coord = (grid_x, grid_y);

        if !self.grids.borrow().contains_key(&grid_coord) {
            let copy_from = if grid_x >= 1 {
                (grid_x - 1, grid_y)
            } else {
                (grid_x, grid_y - 1)
            };
            let read_from_x = copy_from.0 * self.inner_width;
            let read_from_y = copy_from.1 * self.inner_width;
            let mut new_grid = SingleVecGrid::new(self.inner_width, self.inner_height);

            for x in 0..self.inner_width {
                let read_x = read_from_x + x;
                for y in 0..self.inner_height {
                    let read_y = read_from_y + y;
                    let mut v = self.at(&(read_x, read_y)).unwrap() + 1;
                    if v > 9 {
                        v = 1;
                    }
                    new_grid.set((x, y), v);
                }
            }

            self.grids.borrow_mut().insert(grid_coord, new_grid);
        }

        let x = coord.0 % self.inner_width;
        let y = coord.1 % self.inner_height;
        self.grids
            .borrow()
            .get(&(grid_coord))
            .unwrap()
            .at(&(x, y))
            .copied()
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

fn find_risk(grid: &GridOfGrids, end: (usize, usize)) -> u32 {
    let mut visited = BitGrid::new(grid.width(), grid.height());
    let mut queue = BinaryHeap::new();
    queue.push(PathItem {
        coord: (0, 0),
        risk: 0,
    });

    while !queue.is_empty() {
        let item = queue.pop().unwrap();
        if *visited.at(&item.coord).unwrap() {
            continue;
        }
        if item.coord == end {
            return item.risk;
        }
        for (coord, risk) in grid.adjacent(item.coord) {
            queue.push(PathItem {
                coord,
                risk: risk + item.risk,
            });
        }
        visited.set(item.coord, true);
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

    let grid_o_grids = GridOfGrids::new(grid, 5, 5);

    println!(
        "{}",
        find_risk(
            &grid_o_grids,
            (grid_o_grids.inner_width - 1, grid_o_grids.inner_height - 1)
        )
    );

    println!(
        "{}",
        find_risk(
            &grid_o_grids,
            (grid_o_grids.width() - 1, grid_o_grids.height() - 1)
        )
    );
}

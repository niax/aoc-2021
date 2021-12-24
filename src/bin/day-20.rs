use aoc2021::commons::{
    grid::{BitGrid, Grid},
    io::load_argv_lines,
};
use bitvec::prelude::*;

struct Enhancer {
    enhancement: BitVec,
}

impl Enhancer {
    fn step(&self, grid: BitGrid, default: bool) -> BitGrid {
        let mut new = BitGrid::new(grid.width() + 2, grid.height() + 2);

        for y in 0..new.width() {
            for x in 0..new.height() {
                let mut enhance = 0;
                for dy in -1..=1 {
                    for dx in -1..=1 {
                        enhance <<= 1;
                        let sub_x = x as isize + dx - 1;
                        let sub_y = y as isize + dy - 1;
                        if sub_x < 0 || sub_y < 0 {
                            enhance += if default { 1 } else { 0 };
                        } else {
                            enhance += if *grid
                                .at(&(sub_x as usize, sub_y as usize))
                                .unwrap_or(&default)
                            {
                                1
                            } else {
                                0
                            };
                        }
                    }
                }
                if self.enhancement[enhance] {
                    new.set((x, y), true);
                }
            }
        }

        new
    }
}

fn main() {
    let mut lines = load_argv_lines::<String>().map(|x| x.unwrap());
    let enhancement: BitVec = lines.next().unwrap().chars().map(|c| c == '#').collect();
    lines.next().unwrap(); // skip the newline
    let rows: Vec<_> = lines.collect();
    let enhancer = Enhancer { enhancement };
    let mut grid = BitGrid::new(rows[0].len(), rows.len());
    for (y, line) in rows.iter().enumerate() {
        for (x, c) in line.chars().enumerate() {
            grid.set((x, y), c == '#');
        }
    }

    let mut default = false;
    for i in 0..50 {
        grid = enhancer.step(grid, default);
        default = if default {
            enhancer.enhancement[511]
        } else {
            enhancer.enhancement[0]
        };
        if i == 1 {
            println!("{}", grid.set_cell_count());
        }
    }
    println!("{}", grid.set_cell_count());
}

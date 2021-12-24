use aoc2021::commons::{grid::{Grid, SparseGrid, SURROUND}, io::load_argv_lines};
use bitvec::prelude::*;


struct Enhancer {
    enhancement: BitVec,
}


#[allow(dead_code)]
fn print_grid(g: &SparseGrid<bool>) {
    let min_x = g.with_values().map(|(x, _)| x).min().unwrap();
    let max_x = g.with_values().map(|(x, _)| x).max().unwrap();
    let min_y = g.with_values().map(|(_, y)| y).min().unwrap();
    let max_y = g.with_values().map(|(_, y)| y).max().unwrap();
    println!("{:?} - {:?}", (min_x, min_y), (max_x, max_y));

    for y in *min_y..=*max_y {
        let row = (*min_x..=*max_x)
            .map(|x| if *g.at(&(x, y)).unwrap_or(&false) { '#' } else {'.'})
            .collect::<String>();
        println!("{}", row);
    }
}

impl Enhancer {
    fn step(&self, grid: SparseGrid<bool>, default: bool) -> SparseGrid<bool> {
        let mut new = SparseGrid::new();
        let min_x = grid.with_values().map(|(x, _)| x).min().unwrap() - 1;
        let max_x = grid.with_values().map(|(x, _)| x).max().unwrap() + 1;
        let min_y = grid.with_values().map(|(_, y)| y).min().unwrap() - 1;
        let max_y = grid.with_values().map(|(_, y)| y).max().unwrap() + 1;

        for y in min_y..=max_y {
            for x in min_x..=max_x {
                let mut input = BitVec::<Msb0, u32>::new();
                for dy in -1..=1 {
                    for dx in -1..=1 {
                        input.push(*grid.at(&(x + dx, y+dy)).unwrap_or(&default));
                    }
                }
                let enhance = input.load_be::<u32>();
                new.set((x, y), self.enhancement[enhance as usize]);
            }
        }

        new
    }
}

fn main() {
    let mut lines = load_argv_lines::<String>().map(|x| x.unwrap());
    let enhancement: BitVec = lines.next().unwrap().chars().map(|c| c == '#').collect();
    println!("{}", enhancement.iter().map(|b| if *b { '#' } else { '.' } ).collect::<String>());
    lines.next().unwrap(); // skip the newline
    let enhancer = Enhancer { enhancement };
    let mut grid = SparseGrid::new();
    for (y, line) in lines.enumerate() {
        for (x, c) in line.chars().enumerate() {
            grid.set((x as isize, y as isize), c == '#');
        }
    }

    print_grid(&grid);
    let mut default = false;
    for i in 0..2 {
        grid = enhancer.step(grid, default);
        default = if default { enhancer.enhancement[511] } else { enhancer.enhancement[0] }
    }
    print_grid(&grid);
    println!("{}", grid.points().iter().filter(|(_, x)| **x).count());
}

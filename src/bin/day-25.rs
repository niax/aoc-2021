use aoc2021::commons::{
    grid::{Grid, SingleVecGrid},
    io::load_argv_lines,
};

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
enum State {
    Empty,
    Down,
    Right,
}

impl Default for State {
    fn default() -> Self {
        State::Empty
    }
}

impl State {
    fn is_empty(&self) -> bool {
        matches!(self, Self::Empty)
    }
}

type StateGrid = SingleVecGrid<State>;

#[allow(dead_code)]
fn print_grid(g: &StateGrid) {
    for y in 0..g.height() {
        for x in 0..g.width() {
            print!(
                "{}",
                match g.at(&(x, y)).unwrap() {
                    State::Empty => '.',
                    State::Down => 'v',
                    State::Right => '>',
                }
            );
        }
        println!();
    }
}

type Coord = (usize, usize);

fn mov(coord: Coord, offset: Coord, src: &StateGrid, dst: &mut StateGrid) {
    let new_coord = (
        (coord.0 + offset.0) % src.width(),
        (coord.1 + offset.1) % src.height(),
    );
    let curr = src.at(&coord).unwrap();
    let existing = src.at(&new_coord).unwrap();
    let dst_coord = if existing.is_empty() {
        new_coord
    } else {
        coord
    };
    dst.set(dst_coord, *curr);
}

fn step(g: &StateGrid) -> StateGrid {
    let mut next = StateGrid::new(g.width(), g.height());
    let mut downs = Vec::new();
    // Process rights
    for y in 0..g.height() {
        for x in 0..g.width() {
            let coord = (x, y);
            let c = g.at(&coord).unwrap();
            match c {
                State::Right => {
                    mov(coord, (1, 0), g, &mut next);
                }
                State::Down => {
                    downs.push(coord);
                }
                State::Empty => {}
            }
        }
    }

    let mut checkpoint = next.clone();
    // Process downs
    for coord in &downs {
        checkpoint.set(*coord, *g.at(coord).unwrap());
    }

    // Process downs
    for coord in downs {
        mov(coord, (0, 1), &checkpoint, &mut next);
    }

    next
}

fn main() {
    let rows = load_argv_lines::<String>()
        .map(|r| r.unwrap())
        .collect::<Vec<_>>();
    let mut grid = SingleVecGrid::new(rows[0].len(), rows.len());
    for (y, row) in rows.iter().enumerate() {
        for (x, c) in row.chars().enumerate() {
            let state = match c {
                'v' => State::Down,
                '>' => State::Right,
                _ => State::Empty,
            };
            grid.set((x, y), state);
        }
    }

    for i in 1.. {
        let new = step(&grid);
        if new == grid {
            println!("{}", i);
            break;
        }
        grid = new;
    }
}

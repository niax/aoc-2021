use aoc2021::commons::{
    grid::{BitGrid, Grid},
    io::load_stdin_lines,
};
use bitvec::prelude::*;
use lazy_static::lazy_static;
use std::collections::HashMap;

lazy_static! {
    static ref LETTERS: HashMap<u32, char> = {
        let mut h = HashMap::new();
        h.insert(16685654, 'B');
        h.insert(16685665, 'E');
        h.insert(16681504, 'F');
        h.insert(8001879, 'G');
        h.insert(16548415, 'H');
        h.insert(530558, 'J');
        h.insert(16519233, 'L');
        h.insert(16549281, 'K');
        h.insert(16664985, 'R');
        h.insert(16257150, 'U');
        h.insert(9329265, 'Z');
        h
    };
}

fn decode_letter(g: &BitGrid, offset: usize) -> char {
    let mut letter_bits = BitVec::<Msb0, u32>::with_capacity(32);
    for x in 0..4 {
        let x = offset * 5 + x;
        for y in 0..6 {
            letter_bits.push(*g.at(&(x, y)).unwrap());
        }
    }
    *LETTERS.get(&letter_bits.load::<u32>()).unwrap()
}

#[allow(dead_code)]
fn print_grid(g: &BitGrid) {
    for y in 0..g.height() {
        let row = (0..g.width())
            .map(|x| if *g.at(&(x, y)).unwrap() { '#' } else { ' ' })
            .collect::<String>();
        println!("{}", row);
    }
}

enum Fold {
    X(usize),
    Y(usize),
}

impl Fold {
    pub fn map_coord(&self, coord: (usize, usize)) -> (usize, usize) {
        let (x, y) = coord;
        match *self {
            Fold::X(pos) => {
                if x > pos {
                    let diff = x - pos;
                    (pos - diff, y)
                } else {
                    (x, y)
                }
            }
            Fold::Y(pos) => {
                if y > pos {
                    let diff = y - pos;
                    (x, pos - diff)
                } else {
                    (x, y)
                }
            }
        }
    }
}

fn render_points(points: &[(usize, usize)]) -> BitGrid {
    let max_x = points.iter().map(|p| p.0).max().unwrap() + 1;
    let max_y = points.iter().map(|p| p.1).max().unwrap() + 1;

    let mut grid = BitGrid::new(max_x, max_y);
    for p in points {
        grid.set(*p, true);
    }
    grid
}

fn main() {
    let mut points = Vec::new();
    let mut folds = Vec::new();
    let mut points_done = false;
    for line in load_stdin_lines::<String>() {
        let line = line.unwrap();
        if line.is_empty() {
            points_done = true;
        } else if !points_done {
            let (left, right) = line.split_once(',').unwrap();
            let point = (
                left.parse::<usize>().unwrap(),
                right.parse::<usize>().unwrap(),
            );
            points.push(point);
        } else {
            let part = line.split_whitespace().nth(2).unwrap();
            let (orientation, pos) = part.split_once('=').unwrap();
            let pos = pos.parse::<usize>().unwrap();
            let fold = match orientation {
                "x" => Fold::X(pos),
                "y" => Fold::Y(pos),
                _ => panic!("Bad fold type: {}", orientation),
            };
            folds.push(fold);
        }
    }
    for (i, fold) in folds.iter().enumerate() {
        let next_points = points.iter().map(|p| fold.map_coord(*p)).collect();
        points = next_points;
        if i == 0 {
            let grid = render_points(&points);
            println!("{}", grid.set_cell_count());
        }
    }
    let grid = render_points(&points);
    let decoded = (0..8).map(|i| decode_letter(&grid, i)).collect::<String>();
    println!("{}", decoded);

    print_grid(&grid);
}

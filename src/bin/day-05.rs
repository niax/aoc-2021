use aoc2021::commons::{
    geom::Point,
    grid::{BitGrid, Grid, SingleVecGrid},
    io::load_argv_lines,
};
use std::cmp::{self, Ordering};
use std::num::ParseIntError;
use std::str::FromStr;
use thiserror::Error;

#[derive(Debug)]
struct Line {
    from: Point<usize>,
    to: Point<usize>,
}

#[derive(Debug, Error)]
enum LineParseError {
    #[error("Bad number")]
    BadNumber(#[from] ParseIntError),

    #[error("Missing field")]
    MissingField,
}

fn sign(i: isize) -> isize {
    match i.cmp(&0) {
        Ordering::Greater => 1,
        Ordering::Less => -1,
        Ordering::Equal => 0,
    }
}

impl Line {
    pub fn from(&self) -> &Point<usize> {
        &self.from
    }

    pub fn to(&self) -> &Point<usize> {
        &self.to
    }

    pub fn is_diagonal(&self) -> bool {
        !(self.from().x() == self.to().x() || self.from().y() == self.to().y())
    }

    pub fn draw_on<T, D>(&self, grid: &mut T, danger: &mut D)
    where
        T: Grid<Coordinate = (usize, usize), Value = isize>,
        D: Grid<Coordinate = (usize, usize), Value = bool>,
    {
        let dy: isize = *self.to.y() as isize - *self.from.y() as isize;
        let dx: isize = *self.to.x() as isize - *self.from.x() as isize;
        let grad = (sign(dx), sign(dy));

        let mut x = *self.from().x();
        let mut y = *self.from().y();
        while (x, y) != (*self.to().x(), *self.to().y()) {
            let coord = (x, y);
            let v = grid.at(&coord).unwrap_or(&0) + 1;
            if v > 1 {
                danger.set(coord, true);
            }
            grid.set(coord, v);
            x = (x as isize + grad.0) as usize;
            y = (y as isize + grad.1) as usize;
        }
        let coord = (x, y);
        let v = grid.at(&(x, y)).unwrap_or(&0) + 1;
        if v > 1 {
            danger.set(coord, true);
        }
        grid.set(coord, v);
    }
}

impl FromStr for Line {
    type Err = LineParseError;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let input_string = input.to_string();
        let mut iter = input_string.split_whitespace();

        let from_str: Vec<usize> = iter
            .next()
            .ok_or(LineParseError::MissingField)?
            .split(',')
            .map(|x| x.parse().unwrap())
            .collect();
        let _ = iter.next().ok_or(LineParseError::MissingField)?;
        let to_str: Vec<usize> = iter
            .next()
            .ok_or(LineParseError::MissingField)?
            .split(',')
            .map(|x| x.parse().unwrap())
            .collect();

        Ok(Line {
            from: Point::new(from_str[0], from_str[1]),
            to: Point::new(to_str[0], to_str[1]),
        })
    }
}

fn main() {
    let lines: Vec<Line> = load_argv_lines().map(|res| res.unwrap()).collect();

    let max_x = lines
        .iter()
        .map(|l| cmp::max(l.from.x(), l.to.x()))
        .max()
        .unwrap()
        + 1;
    let max_y = lines
        .iter()
        .map(|l| cmp::max(l.from.y(), l.to.y()))
        .max()
        .unwrap()
        + 1;

    let mut grid = SingleVecGrid::new(max_x, max_y);
    let mut danger = BitGrid::new(max_x, max_y);

    for line in &lines {
        if !line.is_diagonal() {
            line.draw_on(&mut grid, &mut danger);
        }
    }

    let part1 = danger.set_cell_count();
    println!("{}", part1);

    for line in &lines {
        if line.is_diagonal() {
            line.draw_on(&mut grid, &mut danger);
        }
    }

    let part2 = danger.set_cell_count();
    println!("{}", part2);
}

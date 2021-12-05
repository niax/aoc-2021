use commons::geom::Point;
use commons::grid::{Grid, SparseGrid};
use commons::io::load_file_lines;
use std::num::ParseIntError;
use std::str::FromStr;
use std::cmp::Ordering;
use thiserror::Error;

#[derive(Debug)]
struct Line {
    from: Point<isize>,
    to: Point<isize>,
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
        Ordering::Equal => 0 ,
    }
}

impl Line {
    pub fn from(&self) -> &Point<isize> {
        &self.from
    }

    pub fn to(&self) -> &Point<isize> {
        &self.to
    }

    pub fn draw_on<T>(&self, grid: &mut T)
        where T: Grid<Coordinate = (isize, isize), Value=isize>,
    {
        let dy = self.to.y() - self.from.y();
        let dx = self.to.x() - self.from.x();
        let grad = (sign(dx), sign(dy));

        let mut point = *self.from();
        while point != *self.to() {
            let coord = (*point.x(), *point.y());
            let v = grid.at(&coord).unwrap_or(&0);
            grid.set(coord, v + 1);
            point += grad;
        }

        let coord = (*self.to.x(), *self.to.y());
        let v = grid.at(&coord).unwrap_or(&0);
        grid.set(coord, v + 1);
    }
}

impl FromStr for Line {
    type Err = LineParseError;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let input_string = input.to_string();
        let mut iter = input_string.split_whitespace();

        let from_str: Vec<isize> = iter
            .next()
            .ok_or(LineParseError::MissingField)?
            .split(',')
            .map(|x| x.parse().unwrap())
            .collect();
        let _ = iter.next().ok_or(LineParseError::MissingField)?;
        let to_str: Vec<isize> = iter
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

fn print_grid(grid: &SparseGrid<isize>) {
    for y in 0..grid.height() {
        for x in 0..grid.width() {
            let coord = (x as isize, y as isize);
            print!(
                "{}",
                grid.at(&coord)
                    .map(|i| i.to_string())
                    .unwrap_or_else(|| ".".to_string())
            );
        }
        println!();
    }
}

fn overlapping_points<T, C>(grid: &T) -> usize 
    where T: Grid<Coordinate=C, Value=isize>
{
    grid.points().iter().filter(|(_, v)| **v > 1).count()
}

fn main() {
    let lines: Vec<Line> = load_file_lines("input.txt")
        .map(|res| res.unwrap())
        .collect();

    let mut grid = SparseGrid::new();

    for line in &lines {
        if line.from().x() == line.to().x() || line.from().y() == line.to().y() {
            line.draw_on(&mut grid);
        }
    }

    let part1 = overlapping_points(&grid);
    println!("{}", part1);

    grid = SparseGrid::new();
    grid.set((0, 0), 0);
    grid.set((9, 9), 0);
    for line in &lines {
        //println!("{:?}", line);
        line.draw_on(&mut grid);
        //print_grid(&grid);
    }

    let part2 = overlapping_points(&grid);
    println!("{}", part2);
}

use aoc2021::commons::{geom::Point, io::load_stdin_lines};
use std::num::ParseIntError;
use std::str::FromStr;
use thiserror::Error;

#[derive(Debug)]
enum Direction {
    Forward(u32),
    Down(u32),
    Up(u32),
}

#[derive(Debug, Error)]
enum DirectionParseError {
    #[error("Unknown direction")]
    UnknownDirection,

    #[error("Unknown unit")]
    UnknownUnit(#[from] ParseIntError),

    #[error("Missing field")]
    MissingField,
}

impl FromStr for Direction {
    type Err = DirectionParseError;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let input_string = input.to_string();
        let mut iter = input_string.split_whitespace();

        let dir_str = iter.next().ok_or(DirectionParseError::MissingField)?;
        let unit = iter
            .next()
            .ok_or(DirectionParseError::MissingField)?
            .parse()?;

        match dir_str {
            "forward" => Ok(Direction::Forward(unit)),
            "down" => Ok(Direction::Down(unit)),
            "up" => Ok(Direction::Up(unit)),
            _ => Err(DirectionParseError::UnknownDirection),
        }
    }
}

fn main() {
    let dirs: Vec<Direction> = load_stdin_lines().map(|res| res.unwrap()).collect();
    // Part 1
    let mut point = Point::origin();
    for dir in dirs.iter() {
        let offset = match dir {
            Direction::Forward(x) => (*x as i32, 0),
            Direction::Up(x) => (0, -(*x as i32)),
            Direction::Down(x) => (0, *x as i32),
        };
        point += offset;
    }
    println!("{}", point.x() * point.y());

    // Part 2
    let mut aim: i32 = 0;
    point = Point::origin();
    for dir in dirs {
        let offset = match dir {
            Direction::Forward(x) => (x as i32, aim * x as i32),
            Direction::Up(x) => {
                aim -= x as i32;
                (0, 0)
            }
            Direction::Down(x) => {
                aim += x as i32;
                (0, 0)
            }
        };
        point += offset;
    }
    println!("{}", point.x() * point.y());
}

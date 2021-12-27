use aoc2021::commons::{geom::Point3D, io::load_argv_lines};
use lazy_static::lazy_static;
use regex::Regex;
use std::num::ParseIntError;
use std::str::FromStr;
use thiserror::Error;

lazy_static! {
    static ref RE: Regex =
        Regex::new(r"(on|off) x=(-?\d+)..(-?\d+),y=(-?\d+)..(-?\d+),z=(-?\d+)..(-?\d+)").unwrap();
}

type Point = Point3D<isize>;

#[derive(Debug, Clone)]
struct Cuboid {
    from: Point,
    to: Point,
    sign: isize,
}

impl Cuboid {
    pub fn new(
        x1: isize,
        x2: isize,
        y1: isize,
        y2: isize,
        z1: isize,
        z2: isize,
        sign: isize,
    ) -> Self {
        let x_min = x1.min(x2);
        let x_max = x1.max(x2);
        let y_min = y1.min(y2);
        let y_max = y1.max(y2);
        let z_min = z1.min(z2);
        let z_max = z1.max(z2);
        Self {
            from: Point::new(x_min, y_min, z_min),
            to: Point::new(x_max, y_max, z_max),
            sign,
        }
    }

    pub fn volume(&self) -> isize {
        self.sign
            * (self.to.x() - self.from.x() + 1)
            * (self.to.y() - self.from.y() + 1)
            * (self.to.z() - self.from.z() + 1)
    }

    pub fn intersects(&self, other: &Self) -> bool {
        self.from.x() <= other.to.x()
            && self.to.x() >= other.from.x()
            && self.from.y() <= other.to.y()
            && self.to.y() >= other.from.y()
            && self.from.z() <= other.to.z()
            && self.to.z() >= other.from.z()
    }

    pub fn intersection(&self, other: &Self) -> Option<Self> {
        if self.intersects(other) {
            let sign = if self.sign == other.sign {
                -self.sign
            } else if self.sign == 1 && other.sign == -1 {
                1
            } else {
                self.sign * other.sign
            };

            Some(Cuboid::new(
                *self.from.x().max(other.from.x()),
                *self.to.x().min(other.to.x()),
                *self.from.y().max(other.from.y()),
                *self.to.y().min(other.to.y()),
                *self.from.z().max(other.from.z()),
                *self.to.z().min(other.to.z()),
                sign,
            ))
        } else {
            None
        }
    }
}

#[derive(Debug, Error)]
enum InstructionParseError {
    #[error("Bad number")]
    BadNumber(#[from] ParseIntError),

    #[error("Missing field")]
    MissingField,
}

impl FromStr for Cuboid {
    type Err = InstructionParseError;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let caps = RE
            .captures(&input)
            .ok_or(InstructionParseError::MissingField)?;

        let on = match caps
            .get(1)
            .ok_or(InstructionParseError::MissingField)?
            .as_str()
        {
            "on" => 1,
            _ => -1,
        };
        let x1 = caps
            .get(2)
            .ok_or(InstructionParseError::MissingField)?
            .as_str()
            .parse()?;
        let x2 = caps
            .get(3)
            .ok_or(InstructionParseError::MissingField)?
            .as_str()
            .parse()?;
        let y1 = caps
            .get(4)
            .ok_or(InstructionParseError::MissingField)?
            .as_str()
            .parse()?;
        let y2 = caps
            .get(5)
            .ok_or(InstructionParseError::MissingField)?
            .as_str()
            .parse()?;
        let z1 = caps
            .get(6)
            .ok_or(InstructionParseError::MissingField)?
            .as_str()
            .parse()?;
        let z2 = caps
            .get(7)
            .ok_or(InstructionParseError::MissingField)?
            .as_str()
            .parse()?;

        Ok(Cuboid::new(x1, x2, y1, y2, z1, z2, on))
    }
}

fn print_answer(cuboids: &Vec<Cuboid>) {
    println!(
        "{}",
        cuboids.clone().iter().map(|c| c.volume()).sum::<isize>()
    );
}

fn main() {
    let in_cuboids: Vec<Cuboid> = load_argv_lines().map(|x| x.unwrap()).collect();

    let part1_cuboid = Cuboid::new(-50, 50, -50, 50, -50, 50, 1);
    let mut done_part1 = false;
    let mut seen_cuboids: Vec<Cuboid> = Vec::new();
    for cuboid in in_cuboids {
        let mut intersections = Vec::with_capacity(seen_cuboids.len());
        if !done_part1 && !cuboid.intersects(&part1_cuboid) {
            done_part1 = true;
            print_answer(&seen_cuboids);
        }

        for existing in &seen_cuboids {
            if let Some(intersect) = cuboid.intersection(&existing) {
                intersections.push(intersect);
            }
        }
        seen_cuboids.extend(intersections);
        if cuboid.sign > 0 {
            seen_cuboids.push(cuboid);
        }
    }

    print_answer(&seen_cuboids);
}
